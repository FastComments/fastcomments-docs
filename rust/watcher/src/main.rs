//! Replaces `src/watch.js`. Watches `src/content/`, `src/templates/`,
//! `src/static/`, and `src/translations.json`. On change, debounces,
//! classifies the changed paths into the minimal set of sitegen
//! invocations needed to bring the output back in sync, and runs each
//! as a subprocess.
//!
//! Spawning the binary instead of calling the library directly keeps this
//! crate's dep graph tiny (no chromium, no quickjs). The classification
//! work lives in [`plan`] so per-edit dev rebuilds touch only the
//! changed guide+locale (mirrors src/watch.js:74-156 + adds locale
//! granularity that the original Node watcher lacked because there was
//! effectively one locale at the time).

mod plan;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use tokio::process::Command;
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::plan::{classify, Action};

/// Cap how many pending change batches we buffer behind the debouncer.
/// Above this, we drop the oldest — the rebuild that runs next will
/// process the latest state anyway.
const EVENT_CHANNEL_CAP: usize = 32;

#[tokio::main]
async fn main() -> Result<()> {
    fcdocs_shared::repo::init_tracing();

    let repo = repo_root()?;
    let sitegen = resolve_sitegen()
        .context("no compiled sitegen binary; run `cargo build --release` or set FCDOCS_SITEGEN_BIN")?;
    info!(sitegen = %sitegen.display(), repo = %repo.display(), "watcher starting");

    // Bounded channel so a flaky filesystem can't OOM us.
    let (tx, mut rx) = tokio::sync::mpsc::channel(EVENT_CHANNEL_CAP);
    let mut debouncer = new_debouncer(Duration::from_millis(200), move |res| {
        // try_send drops on backpressure — exactly what we want for a
        // batch-rebuild loop. The next rebuild we DO trigger will see
        // the up-to-date filesystem anyway.
        let _ = tx.try_send(res);
    })?;

    // Watch the directories most likely to trigger a rebuild.
    for dir in [
        "src/content/guides",
        "src/templates",
        "src/static/css",
        "src/static/images",
        "src/static/js",
        "src/static/csv",
        "src/snippets",
    ] {
        let p = repo.join(dir);
        if p.exists() {
            debouncer
                .watcher()
                .watch(&p, RecursiveMode::Recursive)
                .with_context(|| format!("watch {p:?}"))?;
        }
    }
    let translations = repo.join("src/translations.json");
    if translations.exists() {
        debouncer
            .watcher()
            .watch(&translations, RecursiveMode::NonRecursive)
            .with_context(|| format!("watch {translations:?}"))?;
    }

    info!("watching (press Ctrl-C to stop)");

    // Hold a handle to the running build so a new batch can cancel the
    // outgoing one — no point finishing a rebuild whose inputs already
    // changed.
    let running: Arc<Mutex<Option<tokio::process::Child>>> = Arc::new(Mutex::new(None));

    while let Some(events) = rx.recv().await {
        match events {
            Ok(evs) => {
                if evs.is_empty() {
                    continue;
                }
                let mut paths: Vec<PathBuf> = evs.into_iter().map(|e| e.path).collect();
                paths.sort();
                paths.dedup();
                let plan = classify(&repo, &paths);
                if plan.actions.is_empty() {
                    continue;
                }

                // Cancel any in-flight rebuild before queueing the new
                // batch — its inputs are already stale.
                {
                    let mut guard = running.lock().await;
                    if let Some(mut prev) = guard.take() {
                        let _ = prev.start_kill();
                    }
                }

                for action in &plan.actions {
                    let args = action.to_args();
                    match action {
                        Action::BuildStatic => info!("running: sitegen build-static"),
                        Action::Build { guides, locales } => info!(
                            guides = ?guides,
                            locales = ?locales,
                            "running: sitegen build"
                        ),
                    }

                    let mut cmd = Command::new(&sitegen);
                    cmd.args(&args).current_dir(&repo).kill_on_drop(true);
                    let child = match cmd.spawn() {
                        Ok(c) => c,
                        Err(e) => {
                            warn!(args = ?args, "failed to spawn sitegen: {e}");
                            continue;
                        }
                    };
                    let pid = child.id();
                    // Park the running child so the NEXT debounced batch
                    // can preempt it. We then wait inline so this batch's
                    // actions run sequentially — a static copy must
                    // finish before the content rebuild that wraps
                    // around it kicks off.
                    {
                        let mut guard = running.lock().await;
                        *guard = Some(child);
                    }
                    let mut child_opt = {
                        let mut guard = running.lock().await;
                        guard.take()
                    };
                    if let Some(child) = &mut child_opt {
                        match child.wait().await {
                            Ok(s) if s.success() => {
                                info!(pid = ?pid, args = ?args, "step done")
                            }
                            Ok(s) => warn!(
                                pid = ?pid,
                                args = ?args,
                                "sitegen exited with status {s}"
                            ),
                            Err(e) => warn!(pid = ?pid, args = ?args, "sitegen wait: {e}"),
                        }
                    } else {
                        // Preempted by a new batch — stop this batch's
                        // remaining actions and let the next batch
                        // classify from the new filesystem state.
                        info!("rebuild preempted by newer change batch");
                        break;
                    }
                }
            }
            Err(e) => {
                warn!("watch error: {e:?}");
            }
        }
    }
    Ok(())
}

/// Resolve the sitegen binary:
///
/// 1. `FCDOCS_SITEGEN_BIN` env var (explicit override).
/// 2. Same directory as the running watcher binary
///    (`current_exe()/../sitegen`). This is the normal install layout
///    and the only one that's safe against PATH spoofing.
/// 3. Workspace target dir variants used during development.
///
/// We do *not* prepend `~/.cache/cargo-target` blindly — if the user
/// has stale binaries there from a different checkout they could end
/// up running the wrong code.
fn resolve_sitegen() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("FCDOCS_SITEGEN_BIN") {
        let pb = PathBuf::from(p);
        if pb.exists() {
            return Some(pb);
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let cand = dir.join("sitegen");
            if cand.exists() && cand != exe {
                return Some(cand);
            }
        }
    }
    for cand in [
        "rust/target/release/sitegen",
        "target/release/sitegen",
    ] {
        let pb = PathBuf::from(cand);
        if pb.exists() {
            return Some(pb);
        }
    }
    None
}

use fcdocs_shared::repo::repo_root;

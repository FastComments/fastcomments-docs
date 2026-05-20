//! Replaces `src/watch.js`. Watches `src/content/`, `src/templates/`,
//! `src/static/`, and `src/translations.json`. On change, debounces and
//! re-runs the sitegen binary as a subprocess.
//!
//! Spawning the binary instead of calling the library directly keeps this
//! crate's dep graph tiny (no chromium, no quickjs). For dev-mode latency
//! that doesn't matter — sitegen's incremental case still completes in a
//! few hundred ms per locale.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use tokio::process::Command;
use tokio::sync::Mutex;
use tracing::{info, warn};

/// Cap how many pending change batches we buffer behind the debouncer.
/// Above this, we drop the oldest — the rebuild that runs next will
/// process the latest state anyway.
const EVENT_CHANNEL_CAP: usize = 32;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

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
                let mut only_static = true;
                for p in &paths {
                    let rel = p.strip_prefix(&repo).unwrap_or(p);
                    if !(rel.starts_with("src/static/css")
                        || rel.starts_with("src/static/images")
                        || rel.starts_with("src/static/js")
                        || rel.starts_with("src/static/csv"))
                    {
                        only_static = false;
                        break;
                    }
                }
                let cmd = if only_static {
                    info!("change in src/static — running sitegen build-static");
                    "build-static"
                } else {
                    info!("change detected — running sitegen build");
                    "build"
                };

                // Cancel any running build first.
                {
                    let mut guard = running.lock().await;
                    if let Some(mut prev) = guard.take() {
                        let _ = prev.start_kill();
                    }
                }

                let mut child = match Command::new(&sitegen)
                    .arg(cmd)
                    .current_dir(&repo)
                    .kill_on_drop(true)
                    .spawn()
                {
                    Ok(c) => c,
                    Err(e) => {
                        warn!("failed to spawn sitegen: {e}");
                        continue;
                    }
                };
                let pid = child.id();
                let stored = running.clone();
                // Wait in a separate task so the next event can preempt.
                let log_pid = pid;
                let new_handle = child;
                {
                    let mut guard = stored.lock().await;
                    *guard = Some(new_handle);
                }
                let stored2 = stored.clone();
                tokio::spawn(async move {
                    // Take ownership of the child for waiting.
                    let mut child_opt = stored2.lock().await.take();
                    if let Some(child) = &mut child_opt {
                        match child.wait().await {
                            Ok(s) if s.success() => info!(pid = ?log_pid, "rebuild done"),
                            Ok(s) => warn!(pid = ?log_pid, "sitegen exited with status {s}"),
                            Err(e) => warn!(pid = ?log_pid, "sitegen wait: {e}"),
                        }
                    }
                });
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

fn repo_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let mut cur: &std::path::Path = cwd.as_path();
    loop {
        if cur.join("package.json").exists() && cur.join("src/locales.json").exists() {
            return Ok(cur.to_path_buf());
        }
        match cur.parent() {
            Some(p) => cur = p,
            None => anyhow::bail!("could not locate repo root from {cwd:?}"),
        }
    }
}

//! Replaces `src/watch.js`. Watches `src/content/`, `src/templates/`,
//! `src/static/`, and `src/translations.json`. On change, debounces and
//! re-runs the sitegen binary as a subprocess.
//!
//! Spawning the binary instead of calling the library directly keeps this
//! crate's dep graph tiny (no chromium, no quickjs). For dev-mode latency
//! that doesn't matter — sitegen's incremental case still completes in a
//! few hundred ms per locale.

use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use anyhow::{Context, Result};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let repo = repo_root()?;
    let sitegen = sitegen_path()
        .with_context(|| "no compiled sitegen binary; run `cargo build --release`")?;
    info!(sitegen = %sitegen.display(), repo = %repo.display(), "watcher starting");

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let mut debouncer = new_debouncer(Duration::from_millis(200), move |res| {
        let _ = tx.send(res);
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

    while let Some(events) = rx.recv().await {
        match events {
            Ok(evs) => {
                if evs.is_empty() {
                    continue;
                }
                // Categorize the change.
                let mut paths: Vec<PathBuf> =
                    evs.into_iter().map(|e| e.path).collect();
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
                let status = Command::new(&sitegen)
                    .arg(cmd)
                    .current_dir(&repo)
                    .status();
                match status {
                    Ok(s) if s.success() => info!("rebuild done"),
                    Ok(s) => warn!("sitegen exited with status {s}"),
                    Err(e) => warn!("failed to spawn sitegen: {e}"),
                }
            }
            Err(e) => {
                warn!("watch error: {e}");
            }
        }
    }
    Ok(())
}

fn sitegen_path() -> Option<PathBuf> {
    let candidates = [
        dirs_home().map(|h| h.join(".cache/cargo-target/release/sitegen")),
        Some(PathBuf::from("rust/target/release/sitegen")),
    ];
    candidates.into_iter().flatten().find(|p| p.exists())
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
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

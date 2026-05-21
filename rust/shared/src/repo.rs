//! Locate the FastComments docs repo root + initialize tracing.
//!
//! Both helpers used to be inlined into every workspace binary's
//! `main`. The tracing setup in particular was identical across six
//! binaries (`sdkgen`, `trans`, `watcher`, `indexer`, `sitegen`,
//! `server`).
//!
//! Every binary in the workspace (sitegen, sdkgen, trans, indexer,
//! server) carried an identical 12-line `repo_root()` helper that
//! walks up from the current working directory looking for the
//! `package.json` + `src/locales.json` sentinel pair. Hoisted here so
//! the seven copies collapse to one.

use std::path::PathBuf;

use anyhow::Result;

/// Walk up from the current working directory until we hit a
/// directory containing both `package.json` and `src/locales.json`.
/// That sentinel pair uniquely identifies the FastComments docs repo
/// root (the standalone search-server install lacks them, dev
/// checkouts have both).
pub fn repo_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    locate_from(&cwd)
}

/// Initialize `tracing_subscriber` with the standard FastComments
/// docs filter: `RUST_LOG` if set, else `info`. Every workspace
/// binary used to inline this same six-line setup.
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
}

/// Like [`repo_root`] but starts from `from` instead of the CWD —
/// useful for tests that need to anchor against a fixture directory.
pub fn locate_from(from: &std::path::Path) -> Result<PathBuf> {
    let mut cur: &std::path::Path = from;
    loop {
        if cur.join("package.json").exists() && cur.join("src/locales.json").exists() {
            return Ok(cur.to_path_buf());
        }
        match cur.parent() {
            Some(p) => cur = p,
            None => anyhow::bail!("could not locate repo root from {from:?}"),
        }
    }
}

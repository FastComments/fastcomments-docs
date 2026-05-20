//! `trans cleanup` — port of `src/cleanup-empty-translations.js` and
//! `src/cleanup-empty-generated.js`. Removes zero-byte `.md` files under
//! `src/content/guides/*/items/<locale>/`.

use std::path::{Path, PathBuf};

use anyhow::Result;
use tracing::info;

pub async fn run() -> Result<()> {
    let repo = repo_root()?;
    let guides = repo.join("src/content/guides");
    let mut removed = 0usize;
    for entry in walkdir::WalkDir::new(&guides) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let p = entry.path();
        if p.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let Ok(meta) = entry.metadata() else { continue };
        if meta.len() == 0 {
            if let Err(e) = std::fs::remove_file(p) {
                tracing::warn!(path = %p.display(), error = %e, "failed to remove");
            } else {
                removed += 1;
            }
        }
    }
    info!(removed, "trans cleanup complete");
    Ok(())
}

fn repo_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let mut cur: &Path = cwd.as_path();
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

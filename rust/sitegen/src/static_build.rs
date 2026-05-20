//! Replaces `build-static.sh`. Copies `src/static/{css,csv,images,js}/**`
//! into `src/static/generated/{css,csv,images,js}/`.

use std::path::Path;

use anyhow::{Context, Result};
use tracing::info;

pub async fn run() -> Result<()> {
    let repo = super::build::repo_root()?;
    let src = repo.join("src/static");
    let dst = repo.join("src/static/generated");
    std::fs::create_dir_all(&dst)?;
    let mut copied = 0usize;
    for sub in ["css", "csv", "images", "js"] {
        let s = src.join(sub);
        let d = dst.join(sub);
        if !s.exists() {
            continue;
        }
        copied += copy_tree(&s, &d).with_context(|| format!("copy {sub}"))?;
    }
    info!(copied, "build-static complete");
    Ok(())
}

fn copy_tree(src: &Path, dst: &Path) -> Result<usize> {
    std::fs::create_dir_all(dst)?;
    let mut n = 0;
    for entry in walkdir::WalkDir::new(src) {
        let entry = entry?;
        let rel = entry.path().strip_prefix(src)?;
        let to = dst.join(rel);
        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&to)?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = to.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(entry.path(), &to)?;
            n += 1;
        }
    }
    Ok(n)
}

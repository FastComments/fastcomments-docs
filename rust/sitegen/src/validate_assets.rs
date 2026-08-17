//! Build gate: every asset a generated page points `src=` at must exist
//! on disk.
//!
//! `process_screenshots` inlines the `<img>` for an `[app-screenshot-*]`
//! marker before it captures the PNG, and a capture that fails is a
//! `warn!` rather than an error - so the build exits 0 and the site ships
//! a 404 image. That is how `/images/9d8122ad089178a3b538115b5c83d720.png`
//! reached production: the ja_jp `title=` on the comment-verification
//! email screenshot was re-translated (`確認` -> `検証`), the filename is
//! `md5(url-selector-title)`, and the capture under the new name failed.
//! The old file stayed on disk, so nothing local looked wrong. An
//! external crawl was the only thing that noticed.
//!
//! Runs after `build-static`, which is what puts the hand-authored
//! `src/static/images/**` under `generated/`.

use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::info;

/// Any `src="..."` - `<img>`, `<script>`, `<iframe>`. The templates
/// always quote attributes, and nothing in the generated output uses
/// `srcset`.
static SRC: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)\bsrc\s*=\s*["']([^"']+)["']"#).expect("regex"));

/// A screenshot filename is `md5(url-selector-title)`, so a missing one
/// under `/images/` is a capture that failed, not a typo'd path. Worth
/// naming in the failure so the next person doesn't go hunting for a
/// bad `src` in the markdown.
static MD5_PNG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9a-f]{32}\.png$").expect("regex"));

pub async fn run() -> Result<()> {
    let repo = super::build::repo_root()?;
    let generated = repo.join("src/static/generated");

    let mut pages: Vec<_> = std::fs::read_dir(&generated)
        .with_context(|| format!("read {}", generated.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("html"))
        .collect();
    pages.sort();

    // Existence is memoized per URL: ~4300 pages share ~2200 unique
    // refs (every page carries the same nav icons and bundles), so a
    // stat per occurrence is millions of pointless syscalls.
    let mut exists: HashMap<String, bool> = HashMap::new();
    let mut missing: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for path in &pages {
        let html =
            std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        let page = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        for cap in SRC.captures_iter(&html) {
            let url = &cap[1];
            let Some(rel) = local_path(url) else {
                continue;
            };
            let ok = *exists
                .entry(url.to_string())
                .or_insert_with(|| generated.join(&rel).exists());
            if !ok {
                missing.entry(url.to_string()).or_default().push(page.clone());
            }
        }
    }

    if !missing.is_empty() {
        // One bad item file fans out across every locale, so cap the
        // listing the way validate-headings does.
        const MAX_LISTED: usize = 30;
        for (url, refs) in missing.iter().take(MAX_LISTED) {
            eprintln!("  {url}: no such file, referenced by {} page(s) e.g. {}", refs.len(), refs[0]);
        }
        if missing.len() > MAX_LISTED {
            eprintln!("  ... and {} more", missing.len() - MAX_LISTED);
        }
        if missing.keys().any(|u| is_screenshot_ref(u)) {
            eprintln!(
                "  at least one of these is an app-screenshot capture that failed - \
                 grep the sitegen build log for \"screenshot failed\" to see why"
            );
        }
        anyhow::bail!(
            "{} asset(s) referenced by generated pages are missing under src/static/generated",
            missing.len()
        );
    }

    info!(pages = pages.len(), refs = exists.len(), "validate-assets passed");
    Ok(())
}

/// The path a `src` resolves to inside `generated/`, or `None` when the
/// ref isn't ours to check.
fn local_path(url: &str) -> Option<PathBuf> {
    // `//host/x` is protocol-relative, ie. external. `data:`, `http:`
    // and page-relative refs all fail the leading-slash test.
    if !url.starts_with('/') || url.starts_with("//") {
        return None;
    }
    let path = url.split(['?', '#']).next().unwrap_or(url).trim_start_matches('/');
    if path.is_empty() {
        return None;
    }
    let decoded = urlencoding::decode(path).ok()?.into_owned();
    // Nothing emits a `..` segment; refusing to resolve one keeps the
    // check from stat'ing outside the generated tree if something ever
    // does.
    if decoded.split('/').any(|s| s == "..") {
        return None;
    }
    Some(PathBuf::from(decoded))
}

fn is_screenshot_ref(url: &str) -> bool {
    url.strip_prefix("/images/")
        .map(|name| MD5_PNG.is_match(name))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn srcs(html: &str) -> Vec<String> {
        SRC.captures_iter(html).map(|c| c[1].to_string()).collect()
    }

    #[test]
    fn finds_src_on_any_tag_and_either_quote() {
        let html = r#"<img src='/images/a.png'><script src="/js/b.js"></script>"#;
        assert_eq!(srcs(html), vec!["/images/a.png", "/js/b.js"]);
    }

    #[test]
    fn ignores_refs_we_do_not_own() {
        assert_eq!(local_path("https://cdn.example.com/a.png"), None);
        assert_eq!(local_path("//cdn.example.com/a.png"), None);
        assert_eq!(local_path("data:image/png;base64,iVBOR"), None);
        assert_eq!(local_path("images/a.png"), None);
        assert_eq!(local_path("/"), None);
        assert_eq!(local_path("/../../etc/passwd"), None);
    }

    #[test]
    fn resolves_root_relative_refs() {
        assert_eq!(local_path("/images/a.png"), Some(PathBuf::from("images/a.png")));
        assert_eq!(local_path("/images/a.png?v=2"), Some(PathBuf::from("images/a.png")));
        assert_eq!(local_path("/images/a.png#x"), Some(PathBuf::from("images/a.png")));
        // Percent-encoded, because the code-* pages carry localized names.
        assert_eq!(
            local_path("/images/%E3%81%82.png"),
            Some(PathBuf::from("images/あ.png"))
        );
    }

    #[test]
    fn recognizes_a_screenshot_filename() {
        // The ja_jp comment-verification email that shipped as a 404.
        assert!(is_screenshot_ref("/images/9d8122ad089178a3b538115b5c83d720.png"));
        assert!(!is_screenshot_ref("/images/menu.png"));
        assert!(!is_screenshot_ref("/images/og/9d8122ad089178a3b538115b5c83d720.png"));
    }
}

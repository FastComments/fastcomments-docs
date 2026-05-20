//! Replaces `src/custom-styling-guide-generator.js`. Fetches the upstream
//! widget CSS JS file, extracts the `<style>...</style>` body, and writes
//! it into the custom-styling guide as an inline-code markdown block.

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::info;

const CSS_URL: &str = "https://fastcomments.com/js/css-v2-source.js";

pub async fn run() -> Result<()> {
    info!("fetching CSS from {CSS_URL}");
    let body = reqwest::get(CSS_URL)
        .await
        .with_context(|| format!("GET {CSS_URL}"))?
        .error_for_status()
        .context("CSS endpoint returned error status")?
        .text()
        .await
        .context("read CSS body")?;

    // Matches `export default `<style>...css...</style>`. Mirrors the
    // regex at custom-styling-guide-generator.js:23.
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?s)export\s+default\s+`<style>(.*?)</style>`").expect("regex")
    });
    let cap = RE
        .captures(&body)
        .context("could not extract <style> from JS file")?;
    let css = cap.get(1).unwrap().as_str().trim();
    info!(chars = css.len(), "extracted CSS body");

    let repo = super::build::repo_root()?;
    let generated_dir = repo
        .join("src/content/guides/custom-styling/items/generated");
    std::fs::create_dir_all(&generated_dir)
        .with_context(|| format!("create {generated_dir:?}"))?;

    let markdown = format!(
        "[inline-code-attrs-start title = 'Comment Widget Default CSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]\n[inline-code-start]\n{css}\n[inline-code-end]"
    );
    let out = generated_dir.join("css-v2-generated.md");
    std::fs::write(&out, &markdown).with_context(|| format!("write {out:?}"))?;
    info!(path = %out.display(), "wrote custom-styling guide");
    Ok(())
}

// reqwest dep — pulled in via fcdocs-shared workspace dep. If sitegen's
// Cargo.toml doesn't already have it, we add via fcdocs-shared which
// re-exports nothing of reqwest. So we depend on it directly here:
// see sitegen/Cargo.toml.
//
// (Note: the `reqwest` crate IS in the workspace and shared/Cargo.toml
// pulls it in as a transitive dep, but sitegen needs it as a direct
// dependency to use top-level helpers like `reqwest::get`.)

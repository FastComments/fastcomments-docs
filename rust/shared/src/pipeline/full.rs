//! Full content pipeline used by `fcdocs-sitegen`.
//!
//! Stub — real implementation lands as part of task #16. Skeleton sketches
//! the stages; each gets filled in as the corresponding upstream module
//! (markers::qjs, browser, sidecar /highlight) becomes real.

use std::path::Path;

use anyhow::Result;
use serde::Serialize;

use crate::sidecar::SidecarClient;

/// Result of running the full pipeline on one guide item.
#[derive(Debug, Clone, Serialize)]
pub struct BuiltItem {
    /// Section id (e.g. `wordpress` from `wordpress.md`).
    pub id: String,
    /// Display title from meta.itemsOrdered.
    pub title: String,
    /// Final HTML (markers expanded, code highlighted, snippets substituted).
    pub html: String,
    /// True when this item fell back to the default-locale source.
    pub is_fallback: bool,
}

/// End-to-end build for a single guide item.
///
/// Stages:
/// 1. Handlebars `{{ExampleTenantId}}` substitution.
/// 2. Marker tokenization (inline-code, code-example, api-resource-header,
///    related-parameter, app-screenshot, flow-diagram) — uses
///    `markers::qjs::eval_marker` to parse JS config blocks.
/// 3. Markdown -> HTML via pulldown-cmark.
/// 4. Per `<code class="language-X">` block: call sidecar `/highlight`.
/// 5. Snippet substitution (`[snippet id="..."]` -> file contents).
/// 6. Append the inlined `highlight.js/styles/monokai-sublime.css` block.
pub async fn build_item(
    _markdown_path: &Path,
    _raw_markdown: &str,
    _snippets_dir: &Path,
    _sidecar: &SidecarClient,
) -> Result<BuiltItem> {
    anyhow::bail!("full pipeline not yet implemented")
}

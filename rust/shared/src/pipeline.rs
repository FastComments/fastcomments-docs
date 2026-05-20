//! Content pipeline: markdown source -> final indexable text.
//!
//! Mirrors the behavior of `buildGuideItemForMeta` in `src/guides.js:96-166`
//! plus `cleanSearchText` in `src/build-search-index-worker.js:48-51`.
//!
//! Pipeline order (each stage's behavior is referenced from the current
//! Node source):
//!
//! 1. **handlebars** — `{{ExampleTenantId}}` substitution.
//!    Reference: `src/guides.js:141`, with the context object built at line 142.
//! 2. **markers** — expand `[inline-code-...]`, `[code-example-...]`, and
//!    `[api-resource-header-...]` tags. Config parsing is delegated to the
//!    Node sidecar's `/eval-marker` endpoint because the embedded values are
//!    real JS object literals (see `src/inline-code-generator.js:72-80`,
//!    `src/code-example-generator.js:61-68`,
//!    `src/api-resource-header-generator.js:20-27`).
//!    Emits simplified text-bearing HTML — no syntax highlighting, because
//!    hljs spans wrap text that html-to-text would extract identically.
//! 3. **markdown -> HTML** via `pulldown-cmark`. Replaces `marked@0.6.2`.
//! 4. **snippets** — substitute `[snippet id="..."]` with file contents from
//!    `src/snippets/`. Mirrors `src/snippet-processor.js`.
//! 5. **HTML -> indexable text** — skip `.line-number`, `.copy`, `.top-right`,
//!    `img`, strip `<style>pre code.hljs{...}</style>` blobs. Mirrors
//!    `cleanSearchText` and the `htmlToTextOptions` in
//!    `src/build-search-index-worker.js:39-51`.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Node, Selector};

use crate::sidecar::{MarkerKind, SidecarClient};

/// Hardcoded demo tenant matching `ExampleTenantId` in `src/utils.js`.
pub const EXAMPLE_TENANT_ID: &str = "aKa2Z4Q=";

/// Result of running the pipeline against a markdown source.
#[derive(Debug, Clone)]
pub struct ProcessedItem {
    pub html: String,
    pub indexable_text: String,
}

/// End-to-end pipeline.
pub async fn process_markdown(
    _markdown_path: &Path,
    raw_markdown: &str,
    snippets_dir: &Path,
    sidecar: &SidecarClient,
) -> Result<ProcessedItem> {
    let with_handlebars = apply_handlebars(raw_markdown);
    let with_markers = expand_markers(&with_handlebars, sidecar).await?;
    let html_from_md = markdown_to_html(&with_markers);
    let with_snippets = expand_snippets(&html_from_md, snippets_dir);
    let indexable = html_to_text(&with_snippets);
    Ok(ProcessedItem {
        html: with_snippets,
        indexable_text: indexable,
    })
}

// ------------------------------------------------------------------
// Stage 1: handlebars
// ------------------------------------------------------------------

/// Substitute `{{ExampleTenantId}}` (the only context var passed by the Node
/// pipeline at `src/guides.js:141-143`).
///
/// Using a regex instead of pulling in a full handlebars compiler keeps this
/// hot and predictable. If the markdown ever introduces more variables, swap
/// to the `handlebars` crate.
fn apply_handlebars(input: &str) -> String {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\{\{\s*ExampleTenantId\s*\}\}").expect("regex"));
    RE.replace_all(input, EXAMPLE_TENANT_ID).into_owned()
}

// ------------------------------------------------------------------
// Stage 2: marker expansion
// ------------------------------------------------------------------

const INLINE_CODE_START: &str = "[inline-code-start]";
const INLINE_CODE_END: &str = "[inline-code-end]";
const INLINE_CODE_ATTRS_START: &str = "[inline-code-attrs-start";
const INLINE_CODE_ATTRS_END: &str = "inline-code-attrs-end]";

const CODE_EXAMPLE_START: &str = "[code-example-start";
const CODE_EXAMPLE_END: &str = "code-example-end]";

const API_RES_START: &str = "[api-resource-header-start";
const API_RES_END: &str = "api-resource-header-end]";

const RELATED_PARAM_START: &str = "[related-parameter-start";
const RELATED_PARAM_END: &str = "related-parameter-end]";

const APP_SCREENSHOT_START: &str = "[app-screenshot-start";
const APP_SCREENSHOT_END: &str = "app-screenshot-end]";

const FLOW_DIAGRAM_START: &str = "[flow-diagram-start";
const FLOW_DIAGRAM_END: &str = "flow-diagram-end]";

/// Apply the three marker transforms in the same order as
/// `src/guide-dynamic-content-transformer.js` (skipping
/// `app-screenshot-generator`, `related-parameter-generator`, and
/// `flow-diagram-generator` — those have side effects irrelevant to the
/// indexer's text output, and the existing indexer effectively no-ops their
/// visual output via html-to-text's image-skip rule).
async fn expand_markers(input: &str, sidecar: &SidecarClient) -> Result<String> {
    let after_code_example = expand_code_example(input, sidecar).await?;
    let after_inline_code = expand_inline_code(&after_code_example, sidecar).await?;
    let after_app_screenshot = strip_marker_block(
        &after_inline_code,
        APP_SCREENSHOT_START,
        APP_SCREENSHOT_END,
        "",
    );
    let after_flow_diagram = strip_marker_block(
        &after_app_screenshot,
        FLOW_DIAGRAM_START,
        FLOW_DIAGRAM_END,
        "",
    );
    let after_related = expand_related_parameter(&after_flow_diagram, sidecar).await?;
    let after_api_res = expand_api_resource_header(&after_related, sidecar).await?;
    Ok(after_api_res)
}

/// Strip every block bounded by `start_token`...`end_token`, replacing it
/// with `replacement`. Used for markers whose output we don't care about
/// for indexing purposes (screenshots emit images that html-to-text drops
/// anyway).
fn strip_marker_block(input: &str, start_token: &str, end_token: &str, replacement: &str) -> String {
    let mut current = input.to_string();
    loop {
        let Some(start_idx) = current.find(start_token) else { break };
        let Some(end_idx) = current.find(end_token) else { break };
        if end_idx < start_idx {
            break;
        }
        current = format!(
            "{prefix}{replacement}{suffix}",
            prefix = &current[..start_idx],
            replacement = replacement,
            suffix = &current[end_idx + end_token.len()..],
        );
    }
    current
}

async fn expand_related_parameter(input: &str, sidecar: &SidecarClient) -> Result<String> {
    let mut current = input.to_string();
    loop {
        let Some(start_idx) = current.find(RELATED_PARAM_START) else { break };
        let Some(end_idx) = current.find(RELATED_PARAM_END) else {
            anyhow::bail!("related-parameter start without end");
        };
        if end_idx < start_idx {
            anyhow::bail!("related-parameter end before start");
        }
        let config_source = &current[start_idx + RELATED_PARAM_START.len()..end_idx];
        let parsed = sidecar
            .eval_marker(MarkerKind::RelatedParameter, config_source)
            .await
            .context("parse related-parameter config via sidecar")?;
        let name = parsed.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let type_ = parsed.get("type").and_then(|v| v.as_str()).unwrap_or("");
        // Mirrors src/related-parameter-generator.js:7-16. We skip the
        // optional <a href=typeLink> wrapping because text content is
        // identical with or without the anchor.
        let replacement = format!(
            "<div class=\"related-parameter\">Related Parameter in Code: <span>{name}</span> <span class=\"as\">as</span> <span>{type_}</span></div>",
            name = html_escape::encode_text(name),
            type_ = html_escape::encode_text(type_),
        );
        current = format!(
            "{prefix}{repl}{suffix}",
            prefix = &current[..start_idx],
            repl = replacement,
            suffix = &current[end_idx + RELATED_PARAM_END.len()..],
        );
    }
    Ok(current)
}

async fn expand_inline_code(input: &str, sidecar: &SidecarClient) -> Result<String> {
    let mut current = input.to_string();
    loop {
        let Some(start_idx) = current.find(INLINE_CODE_START) else { break };
        let Some(end_idx) = current.find(INLINE_CODE_END) else {
            anyhow::bail!("inline-code start token without matching end");
        };
        if end_idx < start_idx {
            anyhow::bail!("inline-code end token appears before start");
        }
        let code_body = &current[start_idx + INLINE_CODE_START.len()..end_idx];

        // Locate the matching attrs block (the Node generator just searches
        // from index 0, see src/inline-code-generator.js:67-68).
        let Some(attrs_start) = current.find(INLINE_CODE_ATTRS_START) else {
            anyhow::bail!("inline-code body without attrs block");
        };
        let Some(attrs_end) = current.find(INLINE_CODE_ATTRS_END) else {
            anyhow::bail!("inline-code attrs block without end token");
        };
        let attrs_source = &current[attrs_start + INLINE_CODE_ATTRS_START.len()..attrs_end];

        let parsed = sidecar
            .eval_marker(MarkerKind::InlineCode, attrs_source)
            .await
            .context("parse inline-code attrs via sidecar")?;
        let title = parsed
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let lang = parsed
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("html");

        let replacement = render_inline_code(title, lang, code_body);

        // Splice replacement in for the body, then strip the attrs block.
        let new_body = format!(
            "{prefix}{repl}{suffix}",
            prefix = &current[..start_idx],
            repl = replacement,
            suffix = &current[end_idx + INLINE_CODE_END.len()..],
        );
        let after_body = new_body;

        // Now strip attrs (positions may have shifted; re-find).
        let stripped = if let Some(as_idx) = after_body.find(INLINE_CODE_ATTRS_START) {
            if let Some(ae_idx) = after_body.find(INLINE_CODE_ATTRS_END) {
                format!(
                    "{}{}",
                    &after_body[..as_idx],
                    &after_body[ae_idx + INLINE_CODE_ATTRS_END.len()..]
                )
            } else {
                after_body
            }
        } else {
            after_body
        };
        current = stripped;
    }
    Ok(current)
}

fn render_inline_code(title: &str, lang: &str, code: &str) -> String {
    // Simplified template: text content matches the Node version
    // (`src/inline-code-generator.js:35-55`) after html-to-text strips the
    // `.line-number`, `.copy`, `.top-right`, and `img` selectors. We keep the
    // outer wrapper classes so per-line newline semantics stay similar.
    let title_html = html_escape::encode_text(title);
    let mut out = String::new();
    out.push_str(&format!("<div class=\"code language-{lang}\">"));
    out.push_str("<div class=\"code-head\">");
    out.push_str(&format!("<div class=\"title\">{title_html}</div>"));
    // top-right is `format: 'skip'` in htmlToTextOptions, so its contents are
    // never indexed — we still emit a placeholder so the document structure is
    // visually consistent for downstream HTML consumers.
    out.push_str("<div class=\"top-right\"></div>");
    out.push_str("</div>");
    out.push_str("<div class=\"code-content\">");
    for (i, line) in code.split('\n').enumerate() {
        let n = i + 1;
        out.push_str(&format!(
            "<div class=\"line\"><span class=\"line-number\">{n}</span><line-content class=\"line-content\">{}</line-content></div>",
            html_escape::encode_text(line),
        ));
    }
    out.push_str("</div></div>");
    out
}

async fn expand_code_example(input: &str, sidecar: &SidecarClient) -> Result<String> {
    let mut current = input.to_string();
    loop {
        let Some(start_idx) = current.find(CODE_EXAMPLE_START) else { break };
        let Some(end_idx) = current.find(CODE_EXAMPLE_END) else {
            anyhow::bail!("code-example start token without matching end");
        };
        if end_idx < start_idx {
            anyhow::bail!("code-example end before start");
        }
        let config_source = &current[start_idx + CODE_EXAMPLE_START.len()..end_idx];

        let parsed = sidecar
            .eval_marker(MarkerKind::CodeExample, config_source)
            .await
            .context("parse code-example config via sidecar")?;
        let title = parsed
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let config = parsed
            .get("config")
            .cloned()
            .unwrap_or(serde_json::Value::Null);

        let replacement = render_code_example(title, &config);
        current = format!(
            "{prefix}{repl}{suffix}",
            prefix = &current[..start_idx],
            repl = replacement,
            suffix = &current[end_idx + CODE_EXAMPLE_END.len()..],
        );
    }
    Ok(current)
}

fn render_code_example(title: &str, config: &serde_json::Value) -> String {
    // Mirrors src/code-example-generator.js:70-78 / 31-49. We embed a JSON
    // config blob so the indexable text includes things like `urlId`.
    let title_html = html_escape::encode_text(title);
    let id = title.replace(' ', "-").to_ascii_lowercase();
    let mut full_config = serde_json::json!({"tenantId": "demo"});
    if let serde_json::Value::Object(cfg) = config {
        if let serde_json::Value::Object(full) = &mut full_config {
            for (k, v) in cfg {
                full.insert(k.clone(), v.clone());
            }
        }
    }
    let config_pretty = serde_json::to_string_pretty(&full_config).unwrap_or_default();
    let code_html = format!(
        "<script async src=\"https://cdn.fastcomments.com/js/embed-v2-async.min.js\"></script>\n<div id=\"fastcomments-widget\"></div>\n<script>\nwindow.fcConfigs = [{config_pretty}];\n</script>",
    );

    let mut out = String::new();
    out.push_str(&format!("<div class=\"code\" id=\"{id}\">"));
    out.push_str("<div class=\"code-head\">");
    out.push_str(&format!("<div class=\"title\">{title_html}</div>"));
    out.push_str("<div class=\"top-right\"></div>");
    out.push_str("</div>");
    out.push_str("<div class=\"code-content\">");
    for (i, line) in code_html.split('\n').enumerate() {
        let n = i + 1;
        out.push_str(&format!(
            "<div class=\"line\"><span class=\"line-number\">{n}</span><line-content class=\"line-content\">{}</line-content></div>",
            html_escape::encode_text(line),
        ));
    }
    out.push_str("</div></div>");
    out
}

async fn expand_api_resource_header(input: &str, sidecar: &SidecarClient) -> Result<String> {
    let mut current = input.to_string();
    loop {
        let Some(start_idx) = current.find(API_RES_START) else { break };
        let Some(end_idx) = current.find(API_RES_END) else {
            anyhow::bail!("api-resource-header start without end");
        };
        if end_idx < start_idx {
            anyhow::bail!("api-resource-header end before start");
        }
        let config_source = &current[start_idx + API_RES_START.len()..end_idx];
        let parsed = sidecar
            .eval_marker(MarkerKind::ApiResourceHeader, config_source)
            .await
            .context("parse api-resource-header config via sidecar")?;

        let name = parsed.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let route = parsed.get("route").and_then(|v| v.as_str()).unwrap_or("");
        let credits = parsed.get("creditsCost").and_then(|v| v.as_i64()).unwrap_or(0);
        // Match Number(x).toLocaleString() formatting (en-US default in Node).
        let credits_str = format_with_commas(credits);

        let replacement = format!(
            "<div class=\"api-resource-header\">Resource: <span>{name}</span> <span class=\"as\">as</span> <span>{route}</span> Credit Cost: <span>{credits_str}</span></div>",
            name = html_escape::encode_text(name),
            route = html_escape::encode_text(route),
            credits_str = credits_str,
        );

        current = format!(
            "{prefix}{repl}{suffix}",
            prefix = &current[..start_idx],
            repl = replacement,
            suffix = &current[end_idx + API_RES_END.len()..],
        );
    }
    Ok(current)
}

fn format_with_commas(n: i64) -> String {
    let s = n.abs().to_string();
    let mut out = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    if n < 0 {
        out.push('-');
    }
    out.chars().rev().collect()
}

// ------------------------------------------------------------------
// Stage 3: markdown -> HTML
// ------------------------------------------------------------------

fn markdown_to_html(input: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(input, opts);
    let mut out = String::with_capacity(input.len());
    html::push_html(&mut out, parser);
    out
}

// ------------------------------------------------------------------
// Stage 4: snippets
// ------------------------------------------------------------------

fn expand_snippets(html: &str, snippets_dir: &Path) -> String {
    // Mirrors src/snippet-processor.js:9. Handles both raw `"` and `&quot;`
    // because marked may HTML-encode quotes inside paragraphs.
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"\[snippet\s+id=(?:&quot;|")([^"&]+)(?:&quot;|")\]"#).expect("regex")
    });
    RE.replace_all(html, |caps: &regex::Captures| {
        let id = &caps[1];
        let path: PathBuf = snippets_dir.join(format!("{id}.html"));
        match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!(snippet_id = %id, error = %e, "snippet load failed");
                format!(
                    "<div style=\"color: red; border: 1px solid red; padding: 10px;\">Error: Could not load snippet '{}'</div>",
                    html_escape::encode_text(id),
                )
            }
        }
    })
    .into_owned()
}

// ------------------------------------------------------------------
// Stage 5: HTML -> indexable text
// ------------------------------------------------------------------

static STYLE_HLJS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?s)<style>pre code\.hljs\{.*?</style>").expect("regex")
});

static SKIP_SELECTORS: Lazy<Vec<Selector>> = Lazy::new(|| {
    [".line-number", ".copy", ".top-right", "img", "style", "script"]
        .iter()
        .map(|s| Selector::parse(s).expect("valid selector"))
        .collect()
});

/// HTML -> indexable plain text. Mirrors the behavior of `cleanSearchText`
/// plus the html-to-text v9 defaults configured at
/// `src/build-search-index-worker.js:39-51`.
fn html_to_text(html: &str) -> String {
    // Strip the inline highlight.js stylesheet (it would otherwise leak into
    // the indexable text via html-to-text's `<style>` skip — actually
    // html-to-text v9 *does* skip `<style>` by default, but the original
    // code explicitly strings-out this stylesheet first, so we match.)
    let stripped = STYLE_HLJS.replace_all(html, "").into_owned();

    let doc = Html::parse_fragment(&stripped);
    let mut skip_node_ids: std::collections::HashSet<ego_tree::NodeId> =
        std::collections::HashSet::new();
    for sel in SKIP_SELECTORS.iter() {
        for el in doc.select(sel) {
            mark_subtree(&doc, el.id(), &mut skip_node_ids);
        }
    }

    let mut buf = String::new();
    walk(&doc, doc.tree.root().id(), &skip_node_ids, &mut buf, false);
    normalize_whitespace(&buf)
}

fn mark_subtree(
    doc: &Html,
    id: ego_tree::NodeId,
    out: &mut std::collections::HashSet<ego_tree::NodeId>,
) {
    out.insert(id);
    let node = doc.tree.get(id).expect("node exists");
    for child in node.children() {
        mark_subtree(doc, child.id(), out);
    }
}

#[derive(Clone, Copy)]
enum ListCtx {
    Unordered,
    Ordered(usize),
}

fn walk(
    doc: &Html,
    id: ego_tree::NodeId,
    skip: &std::collections::HashSet<ego_tree::NodeId>,
    out: &mut String,
    inside_pre: bool,
) {
    walk_inner(doc, id, skip, out, inside_pre, false, None);
}

fn walk_inner(
    doc: &Html,
    id: ego_tree::NodeId,
    skip: &std::collections::HashSet<ego_tree::NodeId>,
    out: &mut String,
    inside_pre: bool,
    uppercase: bool,
    parent_list: Option<ListCtx>,
) {
    if skip.contains(&id) {
        return;
    }
    let node = doc.tree.get(id).expect("node exists");
    match node.value() {
        Node::Text(text) => {
            if uppercase {
                for ch in text.text.chars() {
                    out.extend(ch.to_uppercase());
                }
            } else {
                out.push_str(&text.text);
            }
        }
        Node::Element(el) => {
            let tag = el.name();
            let block = matches!(
                tag,
                "p" | "div" | "section" | "article" | "header" | "footer"
                    | "nav" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
                    | "ul" | "ol" | "li" | "tr" | "td" | "th" | "blockquote"
                    | "pre" | "table" | "hr" | "details" | "summary"
                    | "line-content"
            );
            let is_br = tag == "br";
            let is_li = tag == "li";
            let is_anchor = tag == "a";
            let now_in_pre = inside_pre || tag == "pre";
            let now_uppercase = uppercase
                || matches!(tag, "h1" | "h2" | "h3" | "h4" | "h5" | "h6");

            // Track which kind of list we're descending into so each child
            // `<li>` gets the right prefix.
            let child_list = match tag {
                "ul" => Some(ListCtx::Unordered),
                "ol" => Some(ListCtx::Ordered(1)),
                _ => parent_list,
            };

            if block && !out.is_empty() && !out.ends_with('\n') {
                out.push('\n');
            }
            if is_br && !out.ends_with('\n') {
                out.push('\n');
            }
            if is_li {
                match parent_list {
                    Some(ListCtx::Unordered) | None => out.push_str("* "),
                    Some(ListCtx::Ordered(n)) => {
                        out.push_str(&n.to_string());
                        out.push_str(". ");
                    }
                }
            }

            // For ordered lists, walk siblings ourselves so we can increment
            // the counter per <li>.
            if matches!(tag, "ol") {
                let mut idx: usize = 1;
                for child in node.children() {
                    let child_node = doc.tree.get(child.id());
                    let is_li_child = child_node
                        .map(|n| matches!(n.value(), Node::Element(e) if e.name() == "li"))
                        .unwrap_or(false);
                    walk_inner(
                        doc,
                        child.id(),
                        skip,
                        out,
                        now_in_pre,
                        now_uppercase,
                        Some(ListCtx::Ordered(idx)),
                    );
                    if is_li_child {
                        idx += 1;
                    }
                }
            } else {
                for child in node.children() {
                    walk_inner(
                        doc,
                        child.id(),
                        skip,
                        out,
                        now_in_pre,
                        now_uppercase,
                        child_list,
                    );
                }
            }

            if is_anchor {
                if let Some(href) = el.attr("href") {
                    if !href.is_empty() && !href.starts_with('#') {
                        out.push_str(" [");
                        out.push_str(href);
                        out.push(']');
                    }
                }
            }

            if block && !out.ends_with('\n') {
                out.push('\n');
            }
        }
        _ => {
            for child in node.children() {
                walk_inner(doc, child.id(), skip, out, inside_pre, uppercase, parent_list);
            }
        }
    }
}

fn normalize_whitespace(s: &str) -> String {
    // Collapse runs of blank lines down to a single newline pair, and trim
    // trailing whitespace on each line. Roughly mirrors html-to-text's
    // wordwrap-disabled paragraph behavior.
    let mut out = String::with_capacity(s.len());
    let mut prev_blank = false;
    for line in s.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            if !prev_blank && !out.is_empty() {
                out.push('\n');
                prev_blank = true;
            }
        } else {
            out.push_str(trimmed);
            out.push('\n');
            prev_blank = false;
        }
    }
    out.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handlebars_substitutes_tenant_id() {
        let out = apply_handlebars("hello {{ExampleTenantId}} world");
        assert_eq!(out, "hello aKa2Z4Q= world");
    }

    #[test]
    fn handlebars_handles_whitespace() {
        let out = apply_handlebars("{{ ExampleTenantId }}");
        assert_eq!(out, "aKa2Z4Q=");
    }

    #[test]
    fn snippet_expansion_falls_back_on_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let html = r#"<p>before [snippet id="missing"] after</p>"#;
        let out = expand_snippets(html, tmp.path());
        assert!(out.contains("Could not load snippet"));
    }

    #[test]
    fn snippet_expansion_loads_existing() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("foo.html"), "<b>hi</b>").unwrap();
        let html = r#"[snippet id="foo"]"#;
        let out = expand_snippets(html, tmp.path());
        assert_eq!(out, "<b>hi</b>");
    }

    #[test]
    fn snippet_expansion_decoded_entities() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("foo.html"), "<b>hi</b>").unwrap();
        let html = "<p>[snippet id=&quot;foo&quot;]</p>";
        let out = expand_snippets(html, tmp.path());
        assert!(out.contains("<b>hi</b>"));
    }

    #[test]
    fn markdown_basic() {
        let out = markdown_to_html("# Hi\n\nhello *world*");
        assert!(out.contains("<h1>Hi</h1>"));
        assert!(out.contains("<em>world</em>"));
    }

    #[test]
    fn format_with_commas_basic() {
        assert_eq!(format_with_commas(0), "0");
        assert_eq!(format_with_commas(999), "999");
        assert_eq!(format_with_commas(1_000), "1,000");
        assert_eq!(format_with_commas(10_000_000), "10,000,000");
    }

    #[test]
    fn html_to_text_skips_marked_classes() {
        let html = r#"<div class="code"><div class="line"><span class="line-number">1</span><line-content>hello world</line-content></div></div>"#;
        let out = html_to_text(html);
        assert!(out.contains("hello world"), "got: {out:?}");
        assert!(!out.contains('1'), "line number should be stripped, got: {out:?}");
    }

    #[test]
    fn html_to_text_strips_hljs_style() {
        let html = r#"<style>pre code.hljs{padding:1em;}</style><p>visible</p>"#;
        let out = html_to_text(html);
        assert_eq!(out, "visible");
    }

    #[test]
    fn html_to_text_strips_images() {
        let html = r#"<p>before <img src="x.png"> after</p>"#;
        let out = html_to_text(html);
        assert_eq!(out, "before  after");
    }
}

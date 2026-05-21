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
use scraper::{Html, Node};

use crate::markers::qjs;
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

/// Substitute `{{ExampleTenantId}}` / `{{{ExampleTenantId}}}` via the
/// shared helper. Forwarded so the local module retains a stable
/// name; both pipelines must use `super::substitute_example_tenant_id`
/// to avoid a recurrence of the indexer-vs-sitegen regex drift.
fn apply_handlebars(input: &str) -> String {
    super::substitute_example_tenant_id(input)
}

// ------------------------------------------------------------------
// Stage 2: marker expansion
// ------------------------------------------------------------------

use super::marker_names::{
    API_RES_END, API_RES_START, APP_SCREENSHOT_END, APP_SCREENSHOT_START,
    CODE_EXAMPLE_END, CODE_EXAMPLE_START, FLOW_DIAGRAM_END, FLOW_DIAGRAM_START,
    INLINE_CODE_ATTRS_END, INLINE_CODE_ATTRS_START, INLINE_CODE_END, INLINE_CODE_START,
    RELATED_PARAM_END, RELATED_PARAM_START,
};

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
    super::rewrite_blocks_sync(input, start_token, end_token, |_body| {
        Ok(replacement.to_string())
    })
    // Both expand sites in the indexer accepted a missing end token by
    // breaking the loop; preserve that lenient behavior for stripped
    // markers (some legacy source files have orphan tokens).
    .unwrap_or_else(|_| input.to_string())
}

async fn expand_related_parameter(input: &str, _sidecar: &SidecarClient) -> Result<String> {
    // Shared emission with the full pipeline. The optional <a> wrap
    // around typeLink is preserved in both — downstream html_to_text
    // strips it for the indexed search_text either way, but using
    // ONE emission point means a typo in the HTML can't drift
    // silently between pipelines.
    super::rewrite_blocks_sync(input, RELATED_PARAM_START, RELATED_PARAM_END, |config_source| {
        super::render_related_parameter(config_source)
    })
}

async fn expand_inline_code(input: &str, _sidecar: &SidecarClient) -> Result<String> {
    // Inline-code is a 2-block marker: a [inline-code-start]...[inline-code-end]
    // body, paired with the next [inline-code-attrs-start ...
    // inline-code-attrs-end] block in source order (matches the Node
    // implementation at src/inline-code-generator.js:67-68, which find()s
    // both from index 0 of the current string).
    //
    // The old impl re-allocated the whole document on every iteration; this
    // version collects all spans in one scan, evaluates each pair, then
    // stitches the result in a single linear pass.
    expand_inline_code_blocks(input, |title, lang, code| {
        render_inline_code(title, lang, code)
    })
}

/// Shared single-pass scanner used by both indexer + full pipelines so the
/// pairing rules stay in sync. `render(title, lang, body)` returns the
/// replacement for the body span; the attrs span is always deleted.
pub(crate) fn expand_inline_code_blocks(
    input: &str,
    mut render: impl FnMut(&str, &str, &str) -> String,
) -> Result<String> {
    // Pair body N with attrs N in source-position order (the same order
    // Node's `String.prototype.indexOf` produces under the iterate-and-splice
    // pattern).
    let bodies = find_balanced_spans(input, INLINE_CODE_START, INLINE_CODE_END)?;
    let attrs = find_balanced_spans(input, INLINE_CODE_ATTRS_START, INLINE_CODE_ATTRS_END)?;
    if bodies.len() != attrs.len() {
        anyhow::bail!(
            "inline-code: {} body block(s) vs {} attrs block(s)",
            bodies.len(),
            attrs.len()
        );
    }
    if bodies.is_empty() {
        return Ok(input.to_string());
    }

    // Build a sorted list of edits: each body span is replaced with the
    // rendered HTML; each attrs span is deleted.
    let mut edits: Vec<(usize, usize, String)> = Vec::with_capacity(bodies.len() * 2);
    for (body, attrs) in bodies.iter().zip(attrs.iter()) {
        let attrs_source = &input[attrs.body_start..attrs.body_end];
        let parsed = qjs::eval_marker_sync(MarkerKind::InlineCode, attrs_source)
            .context("parse inline-code attrs via qjs")?;
        let title = parsed.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let lang = parsed.get("type").and_then(|v| v.as_str()).unwrap_or("html");
        let body_text = &input[body.body_start..body.body_end];
        let replacement = render(title, lang, body_text);
        edits.push((body.outer_start, body.outer_end, replacement));
        edits.push((attrs.outer_start, attrs.outer_end, String::new()));
    }
    edits.sort_by_key(|e| e.0);

    let mut out = String::with_capacity(input.len());
    let mut cursor = 0;
    for (start, end, repl) in edits {
        out.push_str(&input[cursor..start]);
        out.push_str(&repl);
        cursor = end;
    }
    out.push_str(&input[cursor..]);
    Ok(out)
}

#[derive(Debug)]
pub(crate) struct Span {
    pub outer_start: usize,
    pub outer_end: usize,
    pub body_start: usize,
    pub body_end: usize,
}

pub(crate) fn find_balanced_spans(input: &str, start: &str, end: &str) -> Result<Vec<Span>> {
    let mut out = Vec::new();
    let mut cursor = 0;
    while let Some(rel_s) = input[cursor..].find(start) {
        let s = cursor + rel_s;
        let body_start = s + start.len();
        let Some(rel_e) = input[body_start..].find(end) else {
            anyhow::bail!("'{start}' without matching '{end}'");
        };
        let body_end = body_start + rel_e;
        let outer_end = body_end + end.len();
        out.push(Span {
            outer_start: s,
            outer_end,
            body_start,
            body_end,
        });
        cursor = outer_end;
    }
    Ok(out)
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

async fn expand_code_example(input: &str, _sidecar: &SidecarClient) -> Result<String> {
    super::rewrite_blocks_sync(input, CODE_EXAMPLE_START, CODE_EXAMPLE_END, |config_source| {
        let parsed = qjs::eval_marker_sync(MarkerKind::CodeExample, config_source)
            .context("parse code-example config via qjs")?;
        let title = parsed.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let config = parsed
            .get("config")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        Ok(render_code_example(title, &config))
    })
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

async fn expand_api_resource_header(input: &str, _sidecar: &SidecarClient) -> Result<String> {
    super::rewrite_blocks_sync(input, API_RES_START, API_RES_END, |config_source| {
        let parsed = qjs::eval_marker_sync(MarkerKind::ApiResourceHeader, config_source)
            .context("parse api-resource-header config via qjs")?;
        let name = parsed.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let route = parsed.get("route").and_then(|v| v.as_str()).unwrap_or("");
        let credits = parsed.get("creditsCost").and_then(|v| v.as_i64()).unwrap_or(0);
        // Match Number(x).toLocaleString() formatting (en-US default in Node).
        let credits_str = format_with_commas(credits);
        Ok(format!(
            "<div class=\"api-resource-header\">Resource: <span>{name}</span> <span class=\"as\">as</span> <span>{route}</span> Credit Cost: <span>{credits_str}</span></div>",
            name = html_escape::encode_text(name),
            route = html_escape::encode_text(route),
            credits_str = credits_str,
        ))
    })
}

use super::format_with_commas;

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
    // Per-call memo so a doc that references the same snippet 30 times
    // reads the file once.
    let cache: std::cell::RefCell<std::collections::HashMap<String, String>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
    RE.replace_all(html, |caps: &regex::Captures| {
        let id = &caps[1];
        // Traversal guard: snippet IDs are written into a Path::join.
        let id_path = std::path::Path::new(id);
        let safe = id_path
            .components()
            .all(|c| matches!(c, std::path::Component::Normal(_)))
            && !id.is_empty();
        if !safe {
            tracing::warn!(snippet_id = %id, "rejecting snippet id with unsafe path");
            return format!(
                "<div style=\"color: red; border: 1px solid red; padding: 10px;\">Error: Could not load snippet '{}'</div>",
                html_escape::encode_text(id),
            );
        }
        if let Some(hit) = cache.borrow().get(id) {
            return hit.clone();
        }
        let path: PathBuf = snippets_dir.join(format!("{id}.html"));
        if let Ok(parent_canon) = path.parent().unwrap_or(snippets_dir).canonicalize() {
            if let Ok(base_canon) = snippets_dir.canonicalize() {
                if !parent_canon.starts_with(&base_canon) {
                    tracing::warn!(snippet_id = %id, "snippet path escapes snippets_dir");
                    return format!(
                        "<div style=\"color: red; border: 1px solid red; padding: 10px;\">Error: Could not load snippet '{}'</div>",
                        html_escape::encode_text(id),
                    );
                }
            }
        }
        match std::fs::read_to_string(&path) {
            Ok(s) => {
                cache.borrow_mut().insert(id.to_string(), s.clone());
                s
            }
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
    // Old impl: doc.select(s) per selector = 6 separate DOM walks. We
    // walk once and dispatch on tag/class/attr against the same intent.
    // The selector list is fixed and tiny (.line-number, .copy,
    // .top-right, img, style, script), so an inline matcher is both
    // faster and easier to read than rebuilding the SKIP_SELECTORS
    // vector.
    let mut skip_node_ids: std::collections::HashSet<ego_tree::NodeId> =
        std::collections::HashSet::new();
    collect_skip_subtrees(&doc, doc.tree.root().id(), &mut skip_node_ids);

    let mut buf = String::new();
    walk(&doc, doc.tree.root().id(), &skip_node_ids, &mut buf, false);
    normalize_whitespace(&buf)
}

fn matches_skip_intent(el: &scraper::node::Element) -> bool {
    let tag = el.name();
    if tag == "img" || tag == "style" || tag == "script" {
        return true;
    }
    if let Some(class) = el.attr("class") {
        for c in class.split_ascii_whitespace() {
            if c == "line-number" || c == "copy" || c == "top-right" {
                return true;
            }
        }
    }
    false
}

fn collect_skip_subtrees(
    doc: &Html,
    id: ego_tree::NodeId,
    out: &mut std::collections::HashSet<ego_tree::NodeId>,
) {
    let node = doc.tree.get(id).expect("node exists");
    if let Some(el) = node.value().as_element() {
        if matches_skip_intent(el) {
            mark_subtree(doc, id, out);
            return;
        }
    }
    for child in node.children() {
        collect_skip_subtrees(doc, child.id(), out);
    }
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

    /// Pins the divergence the indexer used to have: triple-brace
    /// `{{{ExampleTenantId}}}` was matched only by the sitegen regex,
    /// so the indexer left stray `{aKa2Z4Q=}` braces in search text.
    /// Every real content occurrence (tenant-id.md × 29 locales) uses
    /// the triple form.
    #[test]
    fn handlebars_handles_triple_brace_form() {
        let out = apply_handlebars("see {{{ExampleTenantId}}} here");
        assert_eq!(out, "see aKa2Z4Q= here");
        // Inside a JS string literal — the exact shape tenant-id.md
        // uses.
        let real = "tenantId: '{{{ExampleTenantId}}}'";
        assert_eq!(apply_handlebars(real), "tenantId: 'aKa2Z4Q='");
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

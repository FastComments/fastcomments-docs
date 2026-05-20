//! Full content pipeline used by `fcdocs-sitegen` to produce the real
//! per-section HTML that goes into `src/static/generated/`.
//!
//! Mirrors the chain in `src/guides.js:141-152`:
//!
//!   1. handlebars `{{ExampleTenantId}}` substitution
//!   2. dynamic content transformers (the marker chain — see
//!      `src/guide-dynamic-content-transformer.js`):
//!      a. code-example
//!      b. inline-code
//!      c. app-screenshot (handled by sitegen because it needs a browser)
//!      d. related-parameter
//!      e. api-resource-header
//!   3. markdown -> HTML via `pulldown-cmark`, with per-fenced-block
//!      highlight via the sidecar's `/highlight`
//!   4. snippet substitution (`[snippet id="..."]` -> contents of
//!      `src/snippets/<id>.html`)
//!   5. append the inlined highlight.js stylesheet
//!
//! Step 2c (app-screenshot) and the flow-diagram stub are NOT performed
//! here — they require chromiumoxide and live in `fcdocs-sitegen` where
//! the browser is launched. This module marks them with a placeholder
//! that the sitegen post-processes.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::markers::qjs;
use crate::sidecar::{MarkerKind, SidecarClient};

pub const EXAMPLE_TENANT_ID: &str = "aKa2Z4Q=";

/// Bundled highlight.js Monokai Sublime stylesheet. Inlined at build time
/// so the binary doesn't need `node_modules/highlight.js/` at runtime.
/// Mirrors the appendage at `src/guides.js:152`.
pub const HLJS_STYLE_CSS: &str = include_str!(
    "../../../../node_modules/highlight.js/styles/monokai-sublime.css"
);

#[derive(Debug, Clone)]
pub struct FullPipelineConfig {
    /// `src/snippets/` directory.
    pub snippets_dir: PathBuf,
    /// `src/static/generated/` — where code-snippet pages get written.
    pub static_generated_dir: PathBuf,
    /// `src/templates/` — for the `code.html` template used to build
    /// per-snippet runnable pages.
    pub template_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ScreenshotPlaceholder {
    pub token: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct FullProcessed {
    /// HTML with `app-screenshot` placeholders. sitegen substitutes these
    /// after capturing the images.
    pub html: String,
    /// One placeholder per `[app-screenshot-...]` marker found in source.
    pub screenshots: Vec<ScreenshotPlaceholder>,
}

/// Run the pipeline up through markdown + snippets + style, but leave
/// `[app-screenshot-...]` as placeholders. The caller (sitegen) is
/// responsible for capturing the screenshots and substituting back.
pub async fn process_markdown(
    raw_markdown: &str,
    cfg: &FullPipelineConfig,
    sidecar: &SidecarClient,
) -> Result<FullProcessed> {
    let s1 = apply_handlebars(raw_markdown);

    // Marker order mirrors the chain in
    // src/guide-dynamic-content-transformer.js. We hoist out app-screenshot
    // first (as placeholders), since the rest of the pipeline doesn't need
    // a browser.
    let (s2, screenshots) = extract_app_screenshot_placeholders(&s1)?;
    let s3 = strip_flow_diagram(&s2);
    let s4 = expand_code_example(&s3, sidecar, cfg).await?;
    let s5 = expand_inline_code(&s4, sidecar, cfg).await?;
    let s6 = expand_api_resource_header(&s5).await?;
    let s7 = expand_related_parameter(&s6).await?;

    let html = markdown_to_html_with_highlight(&s7, sidecar).await?;
    let html = expand_snippets(&html, &cfg.snippets_dir);
    let html = format!("{html}<style>{HLJS_STYLE_CSS}</style>");

    Ok(FullProcessed { html, screenshots })
}

// ------------------------------------------------------------------
// Stage 1: handlebars
// ------------------------------------------------------------------

fn apply_handlebars(input: &str) -> String {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\{\{\s*ExampleTenantId\s*\}\}").expect("regex"));
    RE.replace_all(input, EXAMPLE_TENANT_ID).into_owned()
}

// ------------------------------------------------------------------
// Stage 2a: app-screenshot placeholders
// ------------------------------------------------------------------

const APP_SCREENSHOT_START: &str = "[app-screenshot-start";
const APP_SCREENSHOT_END: &str = "app-screenshot-end]";

fn extract_app_screenshot_placeholders(
    input: &str,
) -> Result<(String, Vec<ScreenshotPlaceholder>)> {
    let mut current = input.to_string();
    let mut out = Vec::new();
    let mut idx = 0usize;
    loop {
        let Some(s) = current.find(APP_SCREENSHOT_START) else {
            break;
        };
        let Some(e) = current[s..].find(APP_SCREENSHOT_END) else {
            anyhow::bail!("app-screenshot start without matching end");
        };
        let block_end = s + e + APP_SCREENSHOT_END.len();
        let config_source = &current[s + APP_SCREENSHOT_START.len()..s + e];
        // Use the same QuickJS evaluator we use for the other markers.
        // app-screenshot configs are JS object literals just like the
        // others; they include `url`, `selector`, `title`, etc.
        let config = qjs::eval_marker_sync(MarkerKind::ApiResourceHeader, config_source)
            .context("eval app-screenshot config")?;
        let token = format!("__FCDOCS_APP_SCREENSHOT_{idx}__");
        out.push(ScreenshotPlaceholder {
            token: token.clone(),
            config,
        });
        current = format!("{}{token}{}", &current[..s], &current[block_end..]);
        idx += 1;
    }
    Ok((current, out))
}

const FLOW_DIAGRAM_START: &str = "[flow-diagram-start";
const FLOW_DIAGRAM_END: &str = "flow-diagram-end]";

fn strip_flow_diagram(input: &str) -> String {
    let mut current = input.to_string();
    loop {
        let Some(s) = current.find(FLOW_DIAGRAM_START) else { break };
        let Some(e) = current[s..].find(FLOW_DIAGRAM_END) else {
            break;
        };
        let block_end = s + e + FLOW_DIAGRAM_END.len();
        current = format!("{}{}", &current[..s], &current[block_end..]);
    }
    current
}

// ------------------------------------------------------------------
// Stage 2b: code-example markers
// ------------------------------------------------------------------

const CODE_EXAMPLE_START: &str = "[code-example-start";
const CODE_EXAMPLE_END: &str = "code-example-end]";

async fn expand_code_example(
    input: &str,
    sidecar: &SidecarClient,
    cfg: &FullPipelineConfig,
) -> Result<String> {
    let mut current = input.to_string();
    loop {
        let Some(s) = current.find(CODE_EXAMPLE_START) else { break };
        let Some(e) = current[s..].find(CODE_EXAMPLE_END) else {
            anyhow::bail!("code-example start without matching end");
        };
        let block_end = s + e + CODE_EXAMPLE_END.len();
        let config_source = &current[s + CODE_EXAMPLE_START.len()..s + e];
        let parsed = qjs::eval_marker_sync(MarkerKind::CodeExample, config_source)
            .context("eval code-example config")?;
        let title = parsed.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let config = parsed.get("config").cloned().unwrap_or(serde_json::Value::Null);
        let lines_to_highlight = parsed
            .get("linesToHighlight")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_i64().map(|n| n as usize))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let is_functional = parsed
            .get("isFunctional")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let additional_demo_code = parsed
            .get("additionalDemoCode")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_default();

        // Build the embed snippet's source HTML and the rendered template,
        // mirroring src/code-example-generator.js:70-89.
        let code_html = render_code_example_source(&config);
        let full_code = if additional_demo_code.is_empty() {
            code_html.clone()
        } else {
            format!("{code_html}\n{additional_demo_code}")
        };

        let code_snippet_name = code_snippet_id("code-example", title);
        let snippet_page_file_name = format!("{code_snippet_name}.html");

        if is_functional {
            write_code_snippet_page(
                cfg,
                &full_code,
                title,
                &snippet_page_file_name,
                &lines_to_highlight,
            )?;
        }

        let highlighted = sidecar
            .highlight(&code_html, Some("html"))
            .await
            .context("sidecar /highlight for code-example")?;
        let body = render_inline_lines(&highlighted.html, &lines_to_highlight, true);

        let replacement = wrap_code_block(
            title,
            None,
            &code_snippet_name,
            is_functional,
            &snippet_page_file_name,
            &body,
            true,
        );
        current = format!("{}{}{}", &current[..s], replacement, &current[block_end..]);
    }
    Ok(current)
}

// ------------------------------------------------------------------
// Stage 2c: inline-code markers
// ------------------------------------------------------------------

const INLINE_CODE_START: &str = "[inline-code-start]";
const INLINE_CODE_END: &str = "[inline-code-end]";
const INLINE_CODE_ATTRS_START: &str = "[inline-code-attrs-start";
const INLINE_CODE_ATTRS_END: &str = "inline-code-attrs-end]";

async fn expand_inline_code(
    input: &str,
    sidecar: &SidecarClient,
    cfg: &FullPipelineConfig,
) -> Result<String> {
    let mut current = input.to_string();
    loop {
        let Some(start_idx) = current.find(INLINE_CODE_START) else { break };
        let Some(end_idx) = current.find(INLINE_CODE_END) else {
            anyhow::bail!("inline-code start without end");
        };
        if end_idx < start_idx {
            anyhow::bail!("inline-code end before start");
        }
        let body = current[start_idx + INLINE_CODE_START.len()..end_idx].to_string();

        // Like the Node generator, we just search from index 0 for the
        // attrs block (src/inline-code-generator.js:67-68).
        let attrs_start = current
            .find(INLINE_CODE_ATTRS_START)
            .context("inline-code body without attrs block")?;
        let attrs_end = current
            .find(INLINE_CODE_ATTRS_END)
            .context("inline-code attrs without end token")?;
        let attrs_source = current
            [attrs_start + INLINE_CODE_ATTRS_START.len()..attrs_end]
            .to_string();

        let parsed = qjs::eval_marker_sync(MarkerKind::InlineCode, &attrs_source)
            .context("eval inline-code attrs")?;
        let title = parsed.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let lang = parsed.get("type").and_then(|v| v.as_str()).unwrap_or("html").to_string();
        let is_functional = parsed
            .get("isFunctional")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let use_demo_tenant = parsed
            .get("useDemoTenant")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let code_snippet_name = code_snippet_id("inline-code", &title);
        let snippet_page_file_name = format!("{code_snippet_name}.html");

        if is_functional {
            write_code_snippet_page(cfg, &body, &title, &snippet_page_file_name, &[])?;
        }

        let highlighted = sidecar
            .highlight(&body, Some(&lang))
            .await
            .context("sidecar /highlight for inline-code")?;
        let body_html = render_inline_lines(&highlighted.html, &[], use_demo_tenant);

        let replacement = wrap_code_block(
            &title,
            Some(&lang),
            &code_snippet_name,
            is_functional,
            &snippet_page_file_name,
            &body_html,
            false,
        );

        // Splice in the replacement for the body, then strip the attrs
        // block (positions shifted; re-find).
        let after_body = format!(
            "{}{}{}",
            &current[..start_idx],
            replacement,
            &current[end_idx + INLINE_CODE_END.len()..]
        );
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

// ------------------------------------------------------------------
// Stage 2d/e: api-resource-header + related-parameter
// ------------------------------------------------------------------

const API_RES_START: &str = "[api-resource-header-start";
const API_RES_END: &str = "api-resource-header-end]";
const RELATED_PARAM_START: &str = "[related-parameter-start";
const RELATED_PARAM_END: &str = "related-parameter-end]";

async fn expand_api_resource_header(input: &str) -> Result<String> {
    let mut current = input.to_string();
    loop {
        let Some(s) = current.find(API_RES_START) else { break };
        let Some(e) = current[s..].find(API_RES_END) else {
            anyhow::bail!("api-resource-header start without end");
        };
        let block_end = s + e + API_RES_END.len();
        let config_source = &current[s + API_RES_START.len()..s + e];
        let parsed = qjs::eval_marker_sync(MarkerKind::ApiResourceHeader, config_source)
            .context("eval api-resource-header config")?;
        let name = parsed.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let route = parsed.get("route").and_then(|v| v.as_str()).unwrap_or("");
        let credits = parsed
            .get("creditsCost")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        // Number(x).toLocaleString() with en-US default produces "10,000".
        let credits_str = format_with_commas(credits);
        // Mirrors src/api-resource-header-generator.js:7.
        let replacement = format!(
            "<div class=\"api-resource-header\">Resource: <span>{name}</span> <span class=\"as\">as</span> <span>{route}</span> Credit Cost: <span>{credits_str}</span></div>",
            name = html_escape::encode_text(name),
            route = html_escape::encode_text(route),
            credits_str = credits_str,
        );
        current = format!("{}{}{}", &current[..s], replacement, &current[block_end..]);
    }
    Ok(current)
}

async fn expand_related_parameter(input: &str) -> Result<String> {
    let mut current = input.to_string();
    loop {
        let Some(s) = current.find(RELATED_PARAM_START) else { break };
        let Some(e) = current[s..].find(RELATED_PARAM_END) else {
            anyhow::bail!("related-parameter start without end");
        };
        let block_end = s + e + RELATED_PARAM_END.len();
        let config_source = &current[s + RELATED_PARAM_START.len()..s + e];
        let parsed = qjs::eval_marker_sync(MarkerKind::RelatedParameter, config_source)
            .context("eval related-parameter config")?;
        let name = parsed.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let type_ = parsed.get("type").and_then(|v| v.as_str()).unwrap_or("");
        let type_link = parsed.get("typeLink").and_then(|v| v.as_str());
        let type_html = match type_link {
            Some(href) if !href.is_empty() => format!(
                "<a href=\"{href}\" target=\"_blank\">{t}</a>",
                href = html_escape::encode_text(href),
                t = html_escape::encode_text(type_),
            ),
            _ => html_escape::encode_text(type_).to_string(),
        };
        let replacement = format!(
            "<div class=\"related-parameter\">Related Parameter in Code: <span>{name}</span> <span class=\"as\">as</span> <span>{type_html}</span></div>",
            name = html_escape::encode_text(name),
        );
        current = format!("{}{}{}", &current[..s], replacement, &current[block_end..]);
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
// Stage 3: markdown -> HTML with per-block highlighting
// ------------------------------------------------------------------

async fn markdown_to_html_with_highlight(input: &str, sidecar: &SidecarClient) -> Result<String> {
    use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    // Single pass: collect raw events, capturing code blocks separately.
    let mut events: Vec<Event<'static>> = Vec::new();
    let mut code_blocks: Vec<(usize, String, String)> = Vec::new(); // (event_idx, lang, code)
    let mut in_code = false;
    let mut code_lang = String::new();
    let mut code_buf = String::new();
    let mut placeholder_idx = 0usize;
    for ev in Parser::new_ext(input, opts) {
        match ev {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                in_code = true;
                code_lang = lang.to_string();
                code_buf.clear();
            }
            Event::Text(t) if in_code => {
                code_buf.push_str(&t);
            }
            Event::End(TagEnd::CodeBlock) => {
                let placeholder = format!("\u{0000}HL{placeholder_idx}\u{0000}");
                placeholder_idx += 1;
                events.push(Event::Html(placeholder.clone().into()));
                code_blocks.push((
                    events.len() - 1,
                    std::mem::take(&mut code_lang),
                    std::mem::take(&mut code_buf),
                ));
                in_code = false;
            }
            // Indented code blocks: we won't highlight them; pass through.
            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)) => {
                in_code = true;
                code_lang.clear();
                code_buf.clear();
            }
            other => {
                events.push(into_owned_event(other));
            }
        }
    }
    // Now render the events to HTML with placeholders in place.
    let mut html = String::with_capacity(input.len());
    pulldown_cmark::html::push_html(&mut html, events.into_iter());

    // Substitute each placeholder with the highlighted <pre><code> block.
    for (_idx, lang, code) in &code_blocks {
        let placeholder = format!("\u{0000}HL{}\u{0000}", _idx_of(&code_blocks, *_idx));
        let resp = sidecar
            .highlight(code, if lang.is_empty() { None } else { Some(lang) })
            .await
            .context("sidecar /highlight for markdown code block")?;
        let language = resp.language.unwrap_or_else(|| lang.clone());
        let body = format!(
            "<pre><code class=\"hljs language-{lang}\">{body}</code></pre>",
            lang = html_escape::encode_double_quoted_attribute(&language),
            body = resp.html,
        );
        html = html.replace(&placeholder, &body);
    }
    Ok(html)
}

fn _idx_of(blocks: &[(usize, String, String)], event_idx: usize) -> usize {
    blocks
        .iter()
        .position(|(i, _, _)| *i == event_idx)
        .expect("placeholder idx found")
}

/// Convert a borrowed `Event<'a>` into an `Event<'static>` by cloning the
/// strings. Necessary because the parser yields events tied to its input
/// borrow, but we want to collect them into a Vec that outlives the
/// borrow.
fn into_owned_event(ev: pulldown_cmark::Event<'_>) -> pulldown_cmark::Event<'static> {
    use pulldown_cmark::{CowStr, Event, Tag, TagEnd};
    fn own_str(s: CowStr<'_>) -> CowStr<'static> {
        CowStr::Boxed(s.to_string().into_boxed_str())
    }
    fn own_tag(t: Tag<'_>) -> Tag<'static> {
        use pulldown_cmark::{
            BlockQuoteKind, CodeBlockKind, HeadingLevel, LinkType, MetadataBlockKind,
        };
        match t {
            Tag::Paragraph => Tag::Paragraph,
            Tag::Heading {
                level,
                id,
                classes,
                attrs,
            } => Tag::Heading {
                level,
                id: id.map(own_str),
                classes: classes.into_iter().map(own_str).collect(),
                attrs: attrs
                    .into_iter()
                    .map(|(k, v)| (own_str(k), v.map(own_str)))
                    .collect(),
            },
            Tag::BlockQuote(k) => Tag::BlockQuote(k),
            Tag::CodeBlock(CodeBlockKind::Fenced(s)) => {
                Tag::CodeBlock(CodeBlockKind::Fenced(own_str(s)))
            }
            Tag::CodeBlock(CodeBlockKind::Indented) => {
                Tag::CodeBlock(CodeBlockKind::Indented)
            }
            Tag::HtmlBlock => Tag::HtmlBlock,
            Tag::List(n) => Tag::List(n),
            Tag::Item => Tag::Item,
            Tag::FootnoteDefinition(s) => Tag::FootnoteDefinition(own_str(s)),
            Tag::DefinitionList => Tag::DefinitionList,
            Tag::DefinitionListTitle => Tag::DefinitionListTitle,
            Tag::DefinitionListDefinition => Tag::DefinitionListDefinition,
            Tag::Table(a) => Tag::Table(a),
            Tag::TableHead => Tag::TableHead,
            Tag::TableRow => Tag::TableRow,
            Tag::TableCell => Tag::TableCell,
            Tag::Emphasis => Tag::Emphasis,
            Tag::Strong => Tag::Strong,
            Tag::Strikethrough => Tag::Strikethrough,
            Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            } => Tag::Link {
                link_type,
                dest_url: own_str(dest_url),
                title: own_str(title),
                id: own_str(id),
            },
            Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            } => Tag::Image {
                link_type,
                dest_url: own_str(dest_url),
                title: own_str(title),
                id: own_str(id),
            },
            Tag::MetadataBlock(MetadataBlockKind::YamlStyle) => {
                Tag::MetadataBlock(MetadataBlockKind::YamlStyle)
            }
            Tag::MetadataBlock(MetadataBlockKind::PlusesStyle) => {
                Tag::MetadataBlock(MetadataBlockKind::PlusesStyle)
            }
            _ => Tag::Paragraph,
        }
    }
    match ev {
        Event::Start(t) => Event::Start(own_tag(t)),
        Event::End(TagEnd::Heading(level)) => Event::End(TagEnd::Heading(level)),
        Event::End(other) => Event::End(other),
        Event::Text(s) => Event::Text(own_str(s)),
        Event::Code(s) => Event::Code(own_str(s)),
        Event::Html(s) => Event::Html(own_str(s)),
        Event::InlineHtml(s) => Event::InlineHtml(own_str(s)),
        Event::FootnoteReference(s) => Event::FootnoteReference(own_str(s)),
        Event::SoftBreak => Event::SoftBreak,
        Event::HardBreak => Event::HardBreak,
        Event::Rule => Event::Rule,
        Event::TaskListMarker(b) => Event::TaskListMarker(b),
        Event::InlineMath(s) => Event::InlineMath(own_str(s)),
        Event::DisplayMath(s) => Event::DisplayMath(own_str(s)),
    }
}

// ------------------------------------------------------------------
// Stage 4: snippet substitution
// ------------------------------------------------------------------

fn expand_snippets(html: &str, snippets_dir: &Path) -> String {
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
// Code-block template helpers (mirrors inline-code/code-example wrappers)
// ------------------------------------------------------------------

/// Wrap a highlighted body with the standard code-head + top-right + copy
/// + (optional) "Run" link. Mirrors `getTemplate` and `getTemplateWrapped`
/// in inline-code-generator.js and code-example-generator.js.
fn wrap_code_block(
    title: &str,
    lang: Option<&str>,
    code_snippet_name: &str,
    is_functional: bool,
    snippet_page_file_name: &str,
    highlighted_body: &str,
    is_code_example: bool,
) -> String {
    let title_esc = html_escape::encode_text(title);
    let mut out = String::new();
    if is_code_example {
        let id = title.to_ascii_lowercase().replace(' ', "-");
        out.push_str(&format!(
            "<div class=\"code\" id=\"{id}\">",
            id = html_escape::encode_double_quoted_attribute(&id),
        ));
    } else {
        let lang_class = lang.unwrap_or("html");
        out.push_str(&format!(
            "<div class=\"code language-{lang_class}\">",
            lang_class = html_escape::encode_double_quoted_attribute(lang_class),
        ));
    }
    out.push_str("<div class=\"code-head\">");
    out.push_str(&format!("<div class=\"title\">{title_esc}</div>"));
    out.push_str("<div class=\"top-right\">");
    out.push_str(&format!(
        "<span class=\"copy\" data-snippet-id=\"{}\"><span>Copy</span> <img src=\"/images/copy-white-24px.png\" alt=\"Copy\" title=\"Copy\"></span>",
        html_escape::encode_double_quoted_attribute(code_snippet_name),
    ));
    if is_functional {
        out.push_str(&format!(
            "<a href=\"/{}\" target=\"_blank\"><span>Run</span> <img src=\"/images/link-external.png\" alt=\"External Link\" title=\"Run This Code Snippet\"></a>",
            html_escape::encode_double_quoted_attribute(snippet_page_file_name),
        ));
    }
    out.push_str("</div>");
    out.push_str("</div>");
    out.push_str("<div class=\"code-content\">");
    out.push_str(highlighted_body);
    out.push_str("</div>");
    out.push_str("</div>");
    out
}

/// Build the per-line `<div class="line">...` wrapping. Mirrors
/// `getTemplateLinesWithHighlight` in both inline-code-generator.js
/// (lines 13-33) and code-example-generator.js (lines 9-29).
fn render_inline_lines(
    highlighted_html: &str,
    lines_to_highlight: &[usize],
    use_demo_tenant: bool,
) -> String {
    let mut out = String::new();
    for (i, line) in highlighted_html.split('\n').enumerate() {
        let mut classes = vec!["line"];
        if lines_to_highlight.contains(&i) {
            classes.push("highlight");
        }
        if line.contains("tenantId") && !use_demo_tenant {
            classes.push("has-tenant-id");
        }
        if line.contains("http") {
            classes.push("has-url");
        }
        out.push_str(&format!(
            "<div class=\"{cls}\"><span class=\"line-number\">{n}</span><line-content class=\"line-content\">{body}</line-content></div>",
            cls = classes.join(" "),
            n = i + 1,
            body = line,
        ));
    }
    out
}

/// Render the embed `<script>` snippet that goes inside a code-example
/// block. Mirrors `codeHTML` template literal at code-example-generator.js:70-78.
fn render_code_example_source(config: &serde_json::Value) -> String {
    let mut full = serde_json::json!({"tenantId": "demo"});
    if let serde_json::Value::Object(cfg) = config {
        if let serde_json::Value::Object(full_obj) = &mut full {
            for (k, v) in cfg {
                full_obj.insert(k.clone(), v.clone());
            }
        }
    }
    let pretty = serde_json::to_string_pretty(&full).unwrap_or_default();
    // Node's JSON.stringify with 4-space indent. serde_json::to_string_pretty
    // uses 2-space indent — replace each leading pair of spaces with 4
    // spaces in indented lines for closer parity.
    let pretty = reindent_to_four_spaces(&pretty);
    format!(
        "\n<script async src=\"https://cdn.fastcomments.com/js/embed-v2-async.min.js\"></script>\n<div id=\"fastcomments-widget\"></div>\n<script>\nwindow.fcConfigs = [{pretty}];\n</script>\n"
    )
}

fn reindent_to_four_spaces(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for line in s.split_inclusive('\n') {
        let leading = line.chars().take_while(|c| *c == ' ').count();
        // serde uses 2-space indent; we want 4.
        out.push_str(&" ".repeat(leading * 2));
        out.push_str(&line[leading..]);
    }
    out
}

fn code_snippet_id(prefix: &str, title: &str) -> String {
    // Mirrors src/inline-code-generator.js:84:
    //   `code-${path.basename(filePath).replace('.md','')}-${args.title.replace(/ /g,'')}`
    // We don't have the file basename plumbed in yet; use a prefix instead.
    // The snippet_page_file_name is mostly opaque to the rendered template
    // (only the link href cares), so any unique string works.
    format!(
        "{prefix}-{}",
        title.replace(' ', "")
    )
}

fn write_code_snippet_page(
    cfg: &FullPipelineConfig,
    code_html: &str,
    snippet_name: &str,
    target_file_name: &str,
    lines_to_highlight: &[usize],
) -> Result<()> {
    use handlebars::Handlebars;
    let template_path = cfg.template_dir.join("code.html");
    let tpl = std::fs::read_to_string(&template_path)
        .with_context(|| format!("read code template {template_path:?}"))?;
    let mut hb = Handlebars::new();
    hb.register_template_string("code", &tpl)
        .context("register code template")?;
    let highlight_from = lines_to_highlight.first().copied().unwrap_or(0);
    let highlight_to = lines_to_highlight.last().copied().unwrap_or(0);
    let ctx = serde_json::json!({
        "codeHTML": code_html,
        "snippetName": snippet_name,
        "highLightLineFrom": highlight_from,
        "highLightLineTo": highlight_to,
        "ExampleTenantId": EXAMPLE_TENANT_ID,
        "lang": "en",
    });
    let rendered = hb.render("code", &ctx).context("render code template")?;
    std::fs::create_dir_all(&cfg.static_generated_dir)?;
    let out_path = cfg.static_generated_dir.join(target_file_name);
    std::fs::write(&out_path, rendered)
        .with_context(|| format!("write snippet page {out_path:?}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handlebars_substitution() {
        assert_eq!(apply_handlebars("hi {{ExampleTenantId}}"), "hi aKa2Z4Q=");
    }

    #[test]
    fn screenshot_placeholders_extracted() {
        let src = r#"before [app-screenshot-start url = '/auth/me'; selector = 'body'; title = 'X'; app-screenshot-end] after"#;
        let (out, ph) = extract_app_screenshot_placeholders(src).unwrap();
        assert_eq!(ph.len(), 1);
        assert!(out.contains(&ph[0].token));
        assert_eq!(ph[0].config["url"], "/auth/me");
        assert_eq!(ph[0].config["selector"], "body");
    }

    #[test]
    fn flow_diagram_stripped() {
        let src = "before [flow-diagram-start title = 'x'; flow-diagram-end] after";
        let out = strip_flow_diagram(src);
        assert_eq!(out, "before  after");
    }

    #[test]
    fn format_with_commas_basic() {
        assert_eq!(format_with_commas(10_000), "10,000");
        assert_eq!(format_with_commas(0), "0");
    }

    #[test]
    fn related_parameter_renders() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let out = rt
            .block_on(expand_related_parameter(
                "[related-parameter-start name = 'foo'; type = 'string'; related-parameter-end]",
            ))
            .unwrap();
        assert!(out.contains("Related Parameter in Code"));
        assert!(out.contains("foo"));
        assert!(out.contains("string"));
    }

    #[test]
    fn api_resource_header_renders() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let out = rt
            .block_on(expand_api_resource_header(
                "[api-resource-header-start name = 'X'; route = 'GET /a'; creditsCost = 100; api-resource-header-end]",
            ))
            .unwrap();
        assert!(out.contains("Resource"));
        assert!(out.contains("GET /a"));
        assert!(out.contains("100"));
    }
}

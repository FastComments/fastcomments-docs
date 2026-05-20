//! OpenAPI doc generator. Port of
//! `src/sdk-doc-generators/openapi-generator.js`.
//!
//! Parses the SDK's `openapi.json`, cross-references operations against
//! the per-class SDK doc tables (DefaultApi.md / PublicApi.md) to find
//! the actual method names, then for each operation emits a markdown
//! section with: description, parameters table, return type link, and
//! the SDK's own code example block (extracted from the generated docs).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use async_trait::async_trait;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;

use super::base::{DocGenerator, DocSection, GeneratedDocs, GeneratorCtx};

pub struct OpenApiGenerator;

#[async_trait]
impl DocGenerator for OpenApiGenerator {
    async fn generate(&self, ctx: &GeneratorCtx) -> Result<GeneratedDocs> {
        let cfg = match ctx.sdk.extra().get("openApiConfig") {
            Some(c) => c.clone(),
            None => {
                anyhow::bail!("openApiConfig missing for SDK {}", ctx.sdk.id);
            }
        };
        let spec_path = cfg
            .get("specPath")
            .and_then(|v| v.as_str())
            .context("openApiConfig.specPath missing")?;
        let docs_path_rel = cfg
            .get("generatedDocsPath")
            .and_then(|v| v.as_str())
            .context("openApiConfig.generatedDocsPath missing")?;
        let doc_file_pattern = cfg
            .get("docFilePattern")
            .and_then(|v| v.as_str())
            .unwrap_or("{ApiClass}.md");
        let api_classes: Vec<String> = cfg
            .get("apiClasses")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_else(|| vec!["DefaultApi".into(), "PublicApi".into()]);

        let spec = parse_spec(&ctx.repo_path.join(spec_path))?;
        let method_lookup =
            build_method_lookup(&ctx.repo_path, docs_path_rel, doc_file_pattern, &api_classes);
        let grouped = group_operations(&spec);

        let mut sections = Vec::new();
        // Validation errors: missing methods + un-extractable return types.
        // Mirrors Node's behavior at openapi-generator.js:172-182 +
        // sdk-guide-generator.js:268-309 — we capture every such defect
        // (rather than bailing on the first) so the build log lists all
        // of them at once, then `guide::generate_all` aborts the run.
        let mut validation_errors: Vec<String> = Vec::new();
        for (resource, operations) in &grouped {
            for op in operations {
                match generate_operation_section(
                    op,
                    resource,
                    &ctx.repo_path,
                    docs_path_rel,
                    doc_file_pattern,
                    &api_classes,
                    &method_lookup,
                    &ctx.sdk,
                ) {
                    Ok(Some(s)) => sections.push(s),
                    Ok(None) => {}
                    Err(e) => {
                        // Format the error consistently across SDKs so
                        // the aggregated build report at end-of-run
                        // groups them cleanly.
                        let msg = format!(
                            "sdk={} operation={} : {e:#}",
                            ctx.sdk.id,
                            op.method_path()
                        );
                        tracing::error!(error = %msg, "openapi validation error");
                        validation_errors.push(msg);
                    }
                }
            }
        }

        Ok(GeneratedDocs {
            intro: None,
            conclusion: None,
            sections,
            validation_errors,
        })
    }
}

// ------------------------------------------------------------------
// Spec parsing + grouping
// ------------------------------------------------------------------

fn parse_spec(path: &Path) -> Result<Value> {
    if !path.exists() {
        anyhow::bail!("OpenAPI spec not found: {path:?}");
    }
    let bytes = std::fs::read(path).with_context(|| format!("read {path:?}"))?;
    let v: Value = serde_json::from_slice(&bytes).with_context(|| format!("parse {path:?}"))?;
    Ok(v)
}

#[derive(Debug, Clone)]
struct Operation {
    operation_id: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    method: String,
    path: String,
    parameters: Vec<Value>,
    /// Full `responses` object from the spec.
    responses: Value,
    tags: Vec<String>,
}

impl Operation {
    fn method_path(&self) -> String {
        format!("{} {}", self.method, self.path)
    }
}

const HTTP_METHODS: &[&str] = &["get", "post", "put", "patch", "delete", "options", "head"];

fn group_operations(spec: &Value) -> HashMap<String, Vec<Operation>> {
    let mut grouped: HashMap<String, Vec<Operation>> = HashMap::new();
    let Some(paths) = spec.get("paths").and_then(|v| v.as_object()) else {
        return grouped;
    };
    for (path_str, path_item) in paths {
        let Some(item_obj) = path_item.as_object() else {
            continue;
        };
        for (method, op) in item_obj {
            if !HTTP_METHODS.contains(&method.as_str()) {
                continue;
            }
            let tags: Vec<String> = op
                .get("tags")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|t| t.as_str().map(String::from)).collect())
                .unwrap_or_default();
            let primary_tag = tags.first().cloned().unwrap_or_else(|| "default".into());
            if primary_tag == "Hidden" {
                continue;
            }
            let resource = if primary_tag == "default" || primary_tag == "Public" {
                let inferred = infer_resource_from_path(path_str);
                if inferred.is_empty() || inferred == "api" {
                    "Misc Apis".to_string()
                } else {
                    inferred
                }
            } else {
                primary_tag.clone()
            };

            let parameters: Vec<Value> = op
                .get("parameters")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            let operation = Operation {
                operation_id: op
                    .get("operationId")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                summary: op.get("summary").and_then(|v| v.as_str()).map(String::from),
                description: op
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                method: method.to_uppercase(),
                path: path_str.clone(),
                parameters,
                responses: op.get("responses").cloned().unwrap_or(Value::Null),
                tags,
            };
            grouped.entry(resource).or_default().push(operation);
        }
    }
    grouped
}

/// Infer the resource name from an API path. Mirrors
/// `inferResourceFromPath` in src/sdk-doc-generators/base-generator.js:99-121.
/// Returns the raw (kebab-case) resource — capitalization happens later
/// via `format_resource_name`.
fn infer_resource_from_path(path: &str) -> String {
    if path.is_empty() {
        return "api".to_string();
    }
    // Pattern 1: /api/vN/<resource> -> <resource>.
    static V_API: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"/api/v\d+/([^/]+)").expect("regex"));
    if let Some(c) = V_API.captures(path) {
        return c[1].to_string();
    }
    // Pattern 2: /<first-segment-before-path-params> -> first segment.
    static FIRST: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^/([^/{?]+)").expect("regex"));
    if let Some(c) = FIRST.captures(path) {
        return c[1].to_string();
    }
    "api".to_string()
}

// ------------------------------------------------------------------
// Method name lookup from SDK doc tables
// ------------------------------------------------------------------

fn build_method_lookup(
    repo_path: &Path,
    docs_path_rel: &str,
    doc_file_pattern: &str,
    api_classes: &[String],
) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for class in api_classes {
        let doc_file = doc_file_pattern.replace("{ApiClass}", class);
        let doc_path = repo_path.join(docs_path_rel).join(&doc_file);
        if !doc_path.exists() {
            tracing::warn!(path = %doc_path.display(), "SDK doc file not found");
            continue;
        }
        let Ok(content) = std::fs::read_to_string(&doc_path) else {
            continue;
        };
        for (k, v) in parse_doc_table(&content) {
            out.insert(k, v);
        }
    }
    out
}

fn parse_doc_table(content: &str) -> Vec<(String, String)> {
    static TABLE_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"(?s)\|?\s*Method\s*\|\s*HTTP request\s*\|\s*Description\s*\|?\s*\n\|?[-\s|]+\|?\s*\n(.*?)(?:\n#{1,2}\s|\z)",
        )
        .expect("regex")
    });
    static ROW_RE: Lazy<Regex> = Lazy::new(|| {
        // | [**method_name**](link) | **METHOD** /path | description
        // Leading pipe optional (Python omits it).
        Regex::new(
            r"\|?\s*\[\*\*([^*]+)\*\*\][^|]*\|\s*\*\*([A-Z][A-Za-z]*)\*\*\s+([^\s|]+)",
        )
        .expect("regex")
    });

    let mut out = Vec::new();
    let Some(cap) = TABLE_RE.captures(content) else {
        return out;
    };
    let body = &cap[1];
    for row in ROW_RE.captures_iter(body) {
        let mut name = row[1].to_string();
        if let Some(stripped) = name.strip_suffix("()") {
            name = stripped.to_string();
        }
        let http_method = row[2].to_uppercase();
        let api_path = row[3].to_string();
        out.push((format!("{http_method} {api_path}"), name));
    }
    out
}

// ------------------------------------------------------------------
// Per-operation markdown generation
// ------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn generate_operation_section(
    op: &Operation,
    resource: &str,
    repo_path: &Path,
    docs_path_rel: &str,
    doc_file_pattern: &str,
    api_classes: &[String],
    method_lookup: &HashMap<String, String>,
    sdk: &crate::config::SdkConfig,
) -> Result<Option<DocSection>> {
    let lookup_key = format!("{} {}", op.method, op.path);
    let name = method_lookup.get(&lookup_key).cloned().ok_or_else(|| {
        anyhow::anyhow!(
            "method not found in SDK docs for {lookup_key} (operation {})",
            op.operation_id.clone().unwrap_or_else(|| op.summary.clone().unwrap_or_default())
        )
    })?;

    let (code_example, return_type) = extract_code_example_and_return_type(
        op,
        repo_path,
        docs_path_rel,
        doc_file_pattern,
        api_classes,
        &name,
    );

    let return_type = return_type.ok_or_else(|| {
        anyhow::anyhow!("failed to extract return type for method: {name} ({lookup_key})")
    })?;

    let github_url = generate_type_github_url(&return_type, sdk, repo_path);
    let content = generate_operation_markdown(op, code_example.as_deref(), &name, &return_type, github_url.as_deref(), sdk);
    let sub_cat = format_resource_name(resource);
    let filename = format!("{}-api-generated.md", sanitize_filename(&name));

    Ok(Some(DocSection {
        name,
        file: Some(filename),
        content,
        sub_cat: Some(sub_cat),
        type_: Some("api".to_string()),
        sidebar_item_classes: None,
    }))
}

fn extract_code_example_and_return_type(
    op: &Operation,
    repo_path: &Path,
    docs_path_rel: &str,
    doc_file_pattern: &str,
    api_classes: &[String],
    method_name: &str,
) -> (Option<String>, Option<String>) {
    let api_class = determine_api_class(op, api_classes);
    let doc_file = doc_file_pattern.replace("{ApiClass}", &api_class);
    let doc_path = repo_path.join(docs_path_rel).join(&doc_file);
    if !doc_path.exists() {
        tracing::warn!(path = %doc_path.display(), "doc file not found");
        return (None, None);
    }
    let Ok(content) = std::fs::read_to_string(&doc_path) else {
        return (None, None);
    };
    parse_code_example_and_return_type(&content, method_name)
}

fn determine_api_class(op: &Operation, api_classes: &[String]) -> String {
    let default_class = api_classes
        .first()
        .cloned()
        .unwrap_or_else(|| "DefaultApi".to_string());
    let public_class = api_classes
        .get(1)
        .cloned()
        .unwrap_or_else(|| "PublicApi".to_string());
    if op.tags.iter().any(|t| t == "Public") {
        return public_class;
    }
    if op.tags.iter().any(|t| t == "Hidden") {
        return "HiddenApi".to_string();
    }
    default_class
}

fn parse_code_example_and_return_type(
    markdown: &str,
    method_name: &str,
) -> (Option<String>, Option<String>) {
    if method_name.is_empty() {
        return (None, None);
    }
    let escaped = regex::escape(method_name);
    // Three header patterns: `# **name**`, `## `name()` `, plain `## name`.
    let patterns = [
        format!(r"(?m)^#+\s*\*\*{escaped}\*\*\s*$"),
        format!(r"(?m)^#+\s*`{escaped}\(\)`\s*$"),
        format!(r"(?m)^#+\s*{escaped}\s*$"),
    ];
    let mut header_match: Option<(usize, usize)> = None;
    for p in &patterns {
        let re = Regex::new(p).expect("regex");
        if let Some(m) = re.find(markdown) {
            header_match = Some((m.start(), m.end()));
            break;
        }
    }
    let Some((_, end)) = header_match else {
        return (None, None);
    };
    let after = &markdown[end..];
    // Find next H1/H2 boundary (matches `\n#{1,2}\s\*\*`).
    static NEXT_HEADER: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\n#{1,2}\s+\*\*").expect("regex"));
    let section_end = NEXT_HEADER
        .find(after)
        .map(|m| m.start())
        .unwrap_or(after.len());
    let section = &after[..section_end];

    // Code example: prefer the `### Example` section, then any fenced block.
    static EX_SECTION: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?s)###\s*Example.*?```[\w]*\n(.*?)```").expect("regex")
    });
    static CODE_BLOCK: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?s)```[\w]*\n(.*?)```").expect("regex"));
    let code_example = EX_SECTION
        .captures(section)
        .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
        .or_else(|| {
            CODE_BLOCK
                .captures(section)
                .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
        });

    // Return type from `### Return type` section.
    static RT: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?s)###\s*Return type\s*\n+\[\*\*(.+?)\*\*\]\(").expect("regex")
    });
    let return_type = RT.captures(section).and_then(|c| {
        let raw = c.get(1)?.as_str().trim();
        let decoded = decode_html_entities(raw);
        Some(normalize_return_type(&decoded))
    });
    (code_example, return_type)
}

fn decode_html_entities(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

fn normalize_return_type(full: &str) -> String {
    let mut working = full.to_string();
    // Java/Ruby generics: List<T>, Optional<T>, Set<T>, Array<T>, Hash<K,V>
    static GENERIC: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^(?:List|Optional|Set|Array|Hash)<(.+)>$").expect("regex"));
    static PY_GENERIC: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^(?:List|Optional|Set)\[([^\]]+)\]$").expect("regex"));
    static PHP_ARRAY: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.+?)\[\]$").expect("regex"));
    static PHP_NAMESPACE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^\\?(?:[A-Za-z]+\\)+([A-Za-z0-9_]+)$").expect("regex"));

    if let Some(c) = GENERIC.captures(&working) {
        let inner = c[1].to_string();
        if !inner.contains(',') && !inner.contains('<') {
            return inner;
        }
        return working;
    }
    if let Some(c) = PY_GENERIC.captures(&working) {
        return c[1].to_string();
    }
    if let Some(c) = PHP_ARRAY.captures(&working) {
        working = c[1].to_string();
    }
    if let Some(c) = PHP_NAMESPACE.captures(&working) {
        return c[1].to_string();
    }
    working
}

fn generate_type_github_url(
    type_name: &str,
    sdk: &crate::config::SdkConfig,
    repo_path: &Path,
) -> Option<String> {
    let base_url = sdk.repo.trim_end_matches(".git").to_string();
    let branch = &sdk.branch;
    let file_path = get_type_file_path(type_name, sdk, repo_path)?;
    Some(format!("{base_url}/blob/{branch}/{file_path}"))
}

fn get_type_file_path(type_name: &str, sdk: &crate::config::SdkConfig, repo_path: &Path) -> Option<String> {
    let language = sdk.language.as_deref()?;
    match language {
        "java" => Some(format!(
            "client/src/main/java/com/fastcomments/model/{type_name}.java"
        )),
        "php" => Some(format!("lib/Model/{type_name}.php")),
        "javascript" => Some(format!("src/generated/src/models/{type_name}.ts")),
        "python" => python_type_file_path(type_name, sdk, repo_path),
        "rust" => rust_type_file_path(type_name, repo_path),
        "go" => go_type_file_path(type_name, repo_path),
        "cpp" => Some(format!(
            "client/include/FastCommentsClient/model/{type_name}.h"
        )),
        "swift" => Some(format!(
            "client/FastCommentsSwift/Models/{type_name}.swift"
        )),
        "ruby" => Some(ruby_type_file_path(type_name)),
        "nim" => Some(nim_type_file_path(type_name)),
        _ => None,
    }
}

fn python_type_file_path(type_name: &str, sdk: &crate::config::SdkConfig, repo_path: &Path) -> Option<String> {
    let cfg = sdk.extra().get("openApiConfig")?;
    let docs_path = cfg.get("generatedDocsPath")?.as_str()?;
    let model_path = repo_path.join(docs_path).join(format!("{type_name}.md"));
    let content = std::fs::read_to_string(&model_path).ok()?;
    static IMPORT_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"from\s+([\w.]+)\s+import\s+\w+").expect("regex"));
    let cap = IMPORT_RE.captures(&content)?;
    let module = cap[1].replace('.', "/");
    Some(format!("{module}.py"))
}

fn rust_type_file_path(type_name: &str, repo_path: &Path) -> Option<String> {
    let models_dir = repo_path.join("client/src/models");
    if !models_dir.exists() {
        return None;
    }
    let pattern = Regex::new(&format!(r"pub\s+(?:struct|enum)\s+{}\b", regex::escape(type_name)))
        .ok()?;
    for entry in std::fs::read_dir(&models_dir).ok()?.flatten() {
        let p = entry.path();
        if p.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        if let Ok(content) = std::fs::read_to_string(&p) {
            if pattern.is_match(&content) {
                let fname = p.file_name()?.to_string_lossy().into_owned();
                return Some(format!("client/src/models/{fname}"));
            }
        }
    }
    None
}

fn go_type_file_path(type_name: &str, repo_path: &Path) -> Option<String> {
    let models_dir = repo_path.join("client");
    if !models_dir.exists() {
        return None;
    }
    let mut tn = type_name.to_string();
    if let Some(rest) = tn.strip_prefix("[]") {
        tn = rest.to_string();
    }
    let pattern = Regex::new(&format!(
        r"type\s+{}\s+(?:struct|interface)",
        regex::escape(&tn)
    ))
    .ok()?;
    for entry in std::fs::read_dir(&models_dir).ok()?.flatten() {
        let p = entry.path();
        let Some(fname) = p.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        if !fname.starts_with("model_") || !fname.ends_with(".go") {
            continue;
        }
        if let Ok(content) = std::fs::read_to_string(&p) {
            if pattern.is_match(&content) {
                return Some(format!("client/{fname}"));
            }
        }
    }
    None
}

fn ruby_type_file_path(type_name: &str) -> String {
    let snake = pascal_to_snake(type_name);
    format!("client/lib/fastcomments-client/models/{snake}.rb")
}

fn nim_type_file_path(type_name: &str) -> String {
    let without_underscores: String = type_name.chars().filter(|c| *c != '_').collect();
    let snake = pascal_to_snake(&without_underscores);
    format!("client/fastcomments/models/model_{snake}.nim")
}

fn pascal_to_snake(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() && i > 0 {
            out.push('_');
        }
        for c in ch.to_lowercase() {
            out.push(c);
        }
    }
    // Trim leading underscore (Node's `.replace(/^_/, '')`).
    out.trim_start_matches('_').to_string()
}

fn generate_operation_markdown(
    op: &Operation,
    code_example: Option<&str>,
    method_name: &str,
    return_type: &str,
    github_url: Option<&str>,
    sdk: &crate::config::SdkConfig,
) -> String {
    let mut lines: Vec<String> = Vec::new();
    if let Some(desc) = &op.description {
        lines.push(desc.clone());
        lines.push(String::new());
    }
    if !op.parameters.is_empty() {
        lines.push("## Parameters".to_string());
        lines.push(String::new());
        lines.push("| Name | Type | Location | Required | Description |".to_string());
        lines.push("|------|------|----------|----------|-------------|".to_string());
        for param in &op.parameters {
            let name = param.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let type_ = param
                .get("schema")
                .and_then(|s| s.get("type"))
                .and_then(|v| v.as_str())
                .or_else(|| param.get("type").and_then(|v| v.as_str()))
                .unwrap_or("string");
            let location = param.get("in").and_then(|v| v.as_str()).unwrap_or("");
            let required = if param
                .get("required")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                "Yes"
            } else {
                "No"
            };
            let description = param
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .replace('|', "\\|");
            lines.push(format!(
                "| {name} | {type_} | {location} | {required} | {description} |"
            ));
        }
        lines.push(String::new());
    }

    lines.push("## Response".to_string());
    lines.push(String::new());
    if let Some(url) = github_url {
        lines.push(format!("Returns: [`{return_type}`]({url})"));
    } else {
        lines.push(format!("Returns: `{return_type}`"));
    }
    lines.push(String::new());

    if let Some(code) = code_example {
        lines.push("## Example".to_string());
        lines.push(String::new());
        let language = language_for_sdk(&sdk.id);
        let title = format!("{method_name} Example");
        lines.push(format!(
            "[inline-code-attrs-start title = '{title}'; type = '{language}'; isFunctional = false; inline-code-attrs-end]"
        ));
        lines.push("[inline-code-start]".to_string());
        lines.push(code.to_string());
        lines.push("[inline-code-end]".to_string());
        lines.push(String::new());
    }

    lines.join("\n")
}

fn language_for_sdk(id: &str) -> &'static str {
    let lower = id.to_lowercase();
    if lower.contains("javascript") || lower.contains("js") {
        "javascript"
    } else if lower.contains("java") {
        "java"
    } else if lower.contains("php") {
        "php"
    } else if lower.contains("python") {
        "python"
    } else if lower.contains("go") {
        "go"
    } else if lower.contains("rust") {
        "rust"
    } else if lower.contains("cpp") || lower.contains("c++") {
        "cpp"
    } else if lower.contains("swift") {
        "swift"
    } else if lower.contains("ruby") {
        "ruby"
    } else if lower.contains("nim") {
        "nim"
    } else {
        ""
    }
}

pub fn format_resource_name(resource: &str) -> String {
    // Mirrors `formatResourceName` in
    // src/sdk-doc-generators/base-generator.js:128-145. kebab/snake-case
    // → Title Case; `sso` and `api` are emitted as full caps.
    let acronyms = ["sso", "api"];
    let cleaned = resource.replace(['-', '_'], " ");
    let mut out = Vec::new();
    for word in cleaned.split(' ') {
        if word.is_empty() {
            continue;
        }
        let lower = word.to_lowercase();
        if acronyms.contains(&lower.as_str()) {
            out.push(word.to_uppercase());
        } else {
            let mut chars = word.chars();
            let upper_first: String = match chars.next() {
                Some(c) => c.to_uppercase().chain(chars).collect(),
                None => String::new(),
            };
            out.push(upper_first);
        }
    }
    out.join(" ")
}

pub fn sanitize_filename(name: &str) -> String {
    let lower = name.to_lowercase();
    static NON_ALNUM: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^a-z0-9]+").expect("regex"));
    NON_ALNUM
        .replace_all(&lower, "-")
        .trim_matches('-')
        .to_string()
}

// ------------------------------------------------------------------
// SdkConfig extra-fields accessor
// ------------------------------------------------------------------

trait SdkExtra {
    fn extra(&self) -> &serde_json::Map<String, Value>;
}

impl SdkExtra for crate::config::SdkConfig {
    fn extra(&self) -> &serde_json::Map<String, Value> {
        self.extra.as_ref().expect("SdkConfig.extra populated")
    }
}

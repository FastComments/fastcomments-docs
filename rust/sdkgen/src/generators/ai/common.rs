//! Shared helpers for the per-language AI doc generators.

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use fcdocs_llm::LlmClient;
use futures::stream::{FuturesUnordered, StreamExt};
use serde::Serialize;
use serde_json::Value;

use crate::generators::base::DocSection;

/// Read and parse an OpenAPI spec from disk. Mirrors `parseOpenAPISpec`
/// in `typescript-ai-generator.js:174-189`.
pub fn parse_openapi_spec(path: &std::path::Path) -> Result<Value> {
    if !path.exists() {
        anyhow::bail!("OpenAPI spec not found: {path:?}");
    }
    let bytes = std::fs::read(path).with_context(|| format!("read {path:?}"))?;
    Ok(serde_json::from_slice(&bytes).with_context(|| format!("parse {path:?}"))?)
}

/// Operation info keyed by operationId. Mirrors `buildOperationMap`
/// in `typescript-ai-generator.js:196-223`.
pub struct OperationInfo {
    pub method: String,
    pub path: String,
    pub tag: Option<String>,
    pub description: Option<String>,
}

const HTTP_METHODS: &[&str] = &["get", "post", "put", "patch", "delete"];

pub fn build_operation_map(spec: &Value) -> indexmap::IndexMap<String, OperationInfo> {
    let mut out = indexmap::IndexMap::new();
    let Some(paths) = spec.get("paths").and_then(|v| v.as_object()) else {
        return out;
    };
    for (path_str, path_item) in paths {
        let Some(item) = path_item.as_object() else { continue };
        for (method, op) in item {
            if !HTTP_METHODS.contains(&method.as_str()) {
                continue;
            }
            let tag = op
                .get("tags")
                .and_then(|v| v.as_array())
                .and_then(|a| a.first())
                .and_then(|t| t.as_str())
                .map(String::from);
            if tag.as_deref() == Some("Hidden") {
                continue;
            }
            let Some(op_id) = op.get("operationId").and_then(|v| v.as_str()) else {
                continue;
            };
            let description = op
                .get("summary")
                .or_else(|| op.get("description"))
                .and_then(|v| v.as_str())
                .map(String::from);
            out.insert(
                op_id.to_string(),
                OperationInfo {
                    method: method.clone(),
                    path: path_str.clone(),
                    tag,
                    description,
                },
            );
        }
    }
    out
}

/// Default `sdk-ai-cache/<sdk-id>/` location for cache files. Mirrors
/// the path constructed at `typescript-ai-generator.js:32`.
pub fn ai_cache_dir(repo_root: &std::path::Path, sdk_id: &str) -> PathBuf {
    repo_root.join("src/sdk-ai-cache").join(sdk_id)
}

/// Sanitize a method name into a filename component. Mirrors
/// `sanitizeFilename` in `base-generator.js`. Used both for marker
/// filenames and for code-snippet IDs.
pub fn sanitize_filename(name: &str) -> String {
    let lower = name.to_lowercase();
    let re = regex::Regex::new(r"[^a-z0-9]+").expect("regex");
    re.replace_all(&lower, "-")
        .trim_matches('-')
        .to_string()
}

/// Find the repo root from a sub-path.
pub fn repo_root_from(p: &std::path::Path) -> Option<PathBuf> {
    let mut cur = p;
    loop {
        if cur.join("src").join("locales.json").exists() {
            return Some(cur.to_path_buf());
        }
        cur = cur.parent()?;
    }
}

/// Uppercase the first character (preserving multi-codepoint
/// uppercase forms). Used by the Method-name → operationId fallback
/// in cpp/typescript AI generators.
pub fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(first) => first.to_uppercase().chain(c).collect(),
        None => String::new(),
    }
}

/// Lowercase the first character. Used by the cpp AI generator's
/// 3-way operationId fallback chain (exact, lower-first, upper-first).
pub fn lowercase_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(first) => first.to_lowercase().chain(c).collect(),
        None => String::new(),
    }
}

/// Infer a REST resource name from a path like `/api/v3/comments` →
/// `"comments"`. Mirrors the two-pattern regex helper that every
/// per-language AI generator carried independently (and the OpenAPI
/// generator originally derived).
pub fn infer_resource_from_path(path: &str) -> String {
    if path.is_empty() {
        return "api".to_string();
    }
    let v = regex::Regex::new(r"/api/v\d+/([^/]+)").unwrap();
    if let Some(c) = v.captures(path) {
        return c[1].to_string();
    }
    let first = regex::Regex::new(r"^/([^/{?]+)").unwrap();
    if let Some(c) = first.captures(path) {
        return c[1].to_string();
    }
    "api".to_string()
}

/// Format the sub-category name for an AI-generated section, applying
/// the same tag → resource → display-case pipeline every per-language
/// AI generator used to carry inline.
pub fn format_resource_name(tag: Option<&str>, path: Option<&str>) -> String {
    let tag = tag.unwrap_or("api");
    let mut resource = tag.to_string();
    if tag.is_empty() || tag == "api" || tag == "Public" {
        let inferred = infer_resource_from_path(path.unwrap_or(""));
        if !inferred.is_empty() && inferred != "api" {
            resource = inferred;
        } else if resource == "Public" {
            resource = "Misc Apis".to_string();
        }
    }
    crate::generators::openapi::format_resource_name(&resource)
}

/// Build the GitHub blob URL for a model file. `prepend_models_path`
/// captures the per-language quirk: cpp/typescript concatenate the
/// configured `modelsPath` in front of the file path (matching the
/// doubled-path Node bug we preserve for byte parity); nim/rust use
/// the path AS-IS because it's already repo-root-relative.
pub fn type_github_url(
    file_path: &str,
    sdk: &crate::config::SdkConfig,
    models_path_rel: &str,
    prepend_models_path: bool,
) -> String {
    let base = sdk.repo.trim_end_matches(".git").trim_end_matches('/');
    let branch = &sdk.branch;
    if prepend_models_path {
        format!("{base}/blob/{branch}/{models_path_rel}{file_path}")
    } else {
        format!("{base}/blob/{branch}/{file_path}")
    }
}

/// Initialized context shared by every per-language AI generator —
/// resolved spec, operationId → info map, the cache-backed LlmClient,
/// and the configured `apiFiles` + `modelsPath`. Lifted out of each
/// generator's `generate()` to dedup ~20 lines of init boilerplate
/// per language.
pub struct AiContext {
    pub op_map: indexmap::IndexMap<String, OperationInfo>,
    pub models_path: String,
    pub api_files: Vec<String>,
    pub llm: LlmClient,
}

/// Look up `<lang>AiConfig.{specPath, modelsPath, apiFiles}` on the
/// SDK, parse the OpenAPI spec, build the operationId map, and stand
/// up the `LlmClient` against `sdk-ai-cache/<sdk-id>/`.
///
/// `lang` is the cache-namespace string (`"typescript"`, `"cpp"`, etc.).
/// `config_key` is the SDK extra key (`"typescriptAiConfig"`, …).
/// `default_models_path` is the per-language fallback when the SDK
/// config omits `modelsPath`.
pub fn init_ai_context(
    ctx: &crate::generators::base::GeneratorCtx,
    lang: &str,
    config_key: &str,
    default_models_path: &str,
) -> Result<AiContext> {
    let cfg = ctx
        .sdk
        .extra
        .as_ref()
        .and_then(|e| e.get(config_key))
        .cloned();
    let Some(cfg) = cfg else {
        anyhow::bail!("{config_key} missing for SDK {}", ctx.sdk.id);
    };
    let spec_path = cfg
        .get("specPath")
        .and_then(|v| v.as_str())
        .unwrap_or("openapi.json");
    let models_path = cfg
        .get("modelsPath")
        .and_then(|v| v.as_str())
        .unwrap_or(default_models_path)
        .to_string();
    let api_files: Vec<String> = cfg
        .get("apiFiles")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let spec = parse_openapi_spec(&ctx.repo_path.join(spec_path))?;
    let op_map = build_operation_map(&spec);
    let Some(repo_root) = repo_root_from(&ctx.repo_path) else {
        anyhow::bail!("could not locate repo root from {:?}", ctx.repo_path);
    };
    let cache_dir = ai_cache_dir(&repo_root, &ctx.sdk.id);
    // Primary model from LLM_MODEL, defaulting to the DeepInfra open model.
    let model =
        std::env::var("LLM_MODEL").unwrap_or_else(|_| "openai/gpt-oss-120b-Turbo".to_string());
    let llm = LlmClient::new(&cache_dir, &model, lang)?;
    Ok(AiContext {
        op_map,
        models_path,
        api_files,
        llm,
    })
}

/// Inputs to [`render_method_section`]. Per-language generators
/// preprocess their own `Method` types into this shape (handling
/// language-specific quirks like nim's `httpClient` filter and
/// `Option[T]` response wrapping) and then defer all formatting to
/// the shared body.
pub struct SectionInput<'a> {
    pub name: &'a str,
    /// Method description. Empty string when none.
    pub description: &'a str,
    /// Parameter rows: `(name, type, required)` — already filtered
    /// by the language (e.g. nim's `httpClient` skip).
    pub parameters: &'a [(String, String, bool)],
    /// Empty when the method has no response body.
    pub response_type: &'a str,
    /// What to print in the `Returns: ...` line. Defaults to
    /// `response_type` for most languages; nim wraps it as
    /// `Option[T]`.
    pub response_display: &'a str,
    /// `Some(file_path)` if the response type is in the parser's
    /// `nested_types` map, used to build a `Returns: [`T`](url)`
    /// link to the GitHub source.
    pub nested_file_path: Option<&'a str>,
    pub code_example: &'a str,
    /// Tag for the `inline-code-attrs type='...'` attribute. One of
    /// `"typescript"`, `"cpp"`, `"nim"`, `"rust"`.
    pub lang_tag: &'static str,
    /// See [`type_github_url`].
    pub prepend_models_path: bool,
    /// OpenAPI tag for the method (sub-category resolution input).
    pub tag: Option<&'a str>,
    /// OpenAPI path for the method (sub-category resolution input).
    pub path: Option<&'a str>,
}

/// `(type, required)` parameter info shared by cpp/rust/typescript
/// parsers. nim has an extra `is_array` field so it keeps its own
/// local variant. The serialized shape (`{"type": "...", "required":
/// true}`) is what every Method's `IndexMap<String, ParamInfo>` ends
/// up with through `serialize_indexmap`, so cache keys stay byte-
/// identical to the Node original.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParamInfo {
    #[serde(rename = "type")]
    pub type_: String,
    pub required: bool,
}

/// Serialize an [`indexmap::IndexMap`] with insertion order
/// preserved. Every per-language parser's `Method` carries the same
/// `#[serde(serialize_with = "serialize_indexmap")] parameters:
/// IndexMap<...>` annotation; they used to declare this helper
/// locally because no shared crate hosted it.
pub fn serialize_indexmap<S, K, V>(
    map: &indexmap::IndexMap<K, V>,
    ser: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    K: serde::Serialize + Eq + std::hash::Hash,
    V: serde::Serialize,
{
    use serde::ser::SerializeMap;
    let mut m = ser.serialize_map(Some(map.len()))?;
    for (k, v) in map {
        m.serialize_entry(k, v)?;
    }
    m.end()
}

/// Per-language `Method` types implement this to share the
/// operationId-enrichment body. Every field has the same `Option<String>`
/// shape across languages EXCEPT `description`, which is `Option<String>`
/// in typescript/rust (cleared+rewritten unconditionally) and `String`
/// in cpp/nim (overridden only when the OpenAPI description is
/// non-empty) — captured by [`override_description_with_openapi`].
pub trait EnrichableMethod {
    fn set_http_method(&mut self, v: Option<String>);
    fn set_path(&mut self, v: Option<String>);
    fn set_tag(&mut self, v: Option<String>);
    fn set_auth_type(&mut self, v: Option<String>);
    fn override_description_with_openapi(&mut self, openapi_description: Option<&str>);
}

/// Provide the trivial setters for an `EnrichableMethod` impl. The
/// description rule is the only language-specific piece, so it stays
/// out of this macro — the caller writes the `override_description...`
/// fn explicitly. Saves four identical `self.xxx = v` lines per
/// language × four languages.
#[macro_export]
macro_rules! impl_enrichable_method_setters {
    ($t:ty) => {
        impl $crate::generators::ai::common::EnrichableMethod for $t {
            fn set_http_method(&mut self, v: Option<String>) { self.http_method = v; }
            fn set_path(&mut self, v: Option<String>) { self.path = v; }
            fn set_tag(&mut self, v: Option<String>) { self.tag = v; }
            fn set_auth_type(&mut self, v: Option<String>) { self.auth_type = v; }
            fn override_description_with_openapi(&mut self, d: Option<&str>) {
                <Self as $crate::generators::ai::common::DescriptionOverride>::override_description(self, d);
            }
        }
    };
}

/// Companion to [`impl_enrichable_method_setters!`]: the only piece
/// the macro can't capture is how the language treats
/// description-overrides (Option<String> raw assign vs String
/// non-empty override). Each language implements just this one
/// method.
pub trait DescriptionOverride {
    fn override_description(&mut self, d: Option<&str>);
}

/// Apply an OpenAPI operation's metadata to a parsed method, mirroring
/// the field-set loop every per-language AI generator carried inline.
/// Auth-type derivation matches Node's `tag === 'Public' ? 'none' :
/// 'x-api-key'` at e.g. typescript-ai-generator.js:80-86.
pub fn apply_operation_info<M: EnrichableMethod>(method: &mut M, info: &OperationInfo) {
    method.set_http_method(Some(info.method.to_uppercase()));
    method.set_path(Some(info.path.clone()));
    method.set_tag(Some(info.tag.clone().unwrap_or_else(|| "api".to_string())));
    method.set_auth_type(Some(
        if info.tag.as_deref() == Some("Public") {
            "none".to_string()
        } else {
            "x-api-key".to_string()
        },
    ));
    method.override_description_with_openapi(info.description.as_deref());
}


/// Try each candidate operationId against the OpenAPI map (in order)
/// and apply the first match's info to the method. Returns whether
/// a match was found — the caller decides whether to log a warning
/// (Node typescript-ai-generator warns; cpp/nim silently skip).
pub fn enrich_with_first_match<M: EnrichableMethod>(
    op_map: &indexmap::IndexMap<String, OperationInfo>,
    method: &mut M,
    candidate_ids: &[&str],
) -> bool {
    for id in candidate_ids {
        if let Some(info) = op_map.get(*id) {
            apply_operation_info(method, info);
            return true;
        }
    }
    false
}

/// Per-language `Method` types implement this to drive the shared
/// `build_method_section` helper. Each generator previously carried
/// a ~25-line wrapper that constructed a [`SectionInput`] inline;
/// the trait lifts the per-language quirks (description shape,
/// param-filtering, response wrapping, language tag, URL strategy)
/// into focused accessor methods so the construction lives once.
pub trait MethodForSection {
    /// Language tag for the `inline-code-attrs type='...'` attribute.
    const LANG_TAG: &'static str;
    /// Whether [`type_github_url`] should prepend `models_path_rel`.
    /// True for cpp/typescript/rust; false for nim.
    const PREPEND_MODELS_PATH: bool;
    fn section_name(&self) -> &str;
    fn section_description(&self) -> &str;
    /// Parameter rows (post-filtering, e.g. nim drops `httpClient`).
    fn section_params(&self) -> Vec<(String, String, bool)>;
    fn section_response_type(&self) -> &str;
    /// What to print in `Returns: ...`. Same as `response_type` for
    /// most languages; nim wraps it as `Option[T]`.
    fn section_response_display(&self) -> String;
    fn section_nested_file_path(&self) -> Option<&str>;
    fn section_tag(&self) -> Option<&str>;
    fn section_path(&self) -> Option<&str>;
}

/// Render a doc section for any method that implements
/// [`MethodForSection`]. Replaces each per-language
/// `build_method_section` wrapper.
pub fn build_method_section<M: MethodForSection>(
    method: &M,
    code_example: &str,
    sdk: &crate::config::SdkConfig,
    models_path_rel: &str,
) -> Option<DocSection> {
    let params = method.section_params();
    let response_display = method.section_response_display();
    render_method_section(
        SectionInput {
            name: method.section_name(),
            description: method.section_description(),
            parameters: &params,
            response_type: method.section_response_type(),
            response_display: &response_display,
            nested_file_path: method.section_nested_file_path(),
            code_example,
            lang_tag: M::LANG_TAG,
            prepend_models_path: M::PREPEND_MODELS_PATH,
            tag: method.section_tag(),
            path: method.section_path(),
        },
        sdk,
        models_path_rel,
    )
}

/// Shared body of `build_method_section` for every per-language AI
/// generator. Every generator previously carried ~50 lines of this
/// verbatim — the only language-specific bits are the preprocessing
/// (option-vs-string description, response wrapping, param filtering)
/// which the caller handles before constructing [`SectionInput`].
pub fn render_method_section(
    input: SectionInput<'_>,
    sdk: &crate::config::SdkConfig,
    models_path_rel: &str,
) -> Option<DocSection> {
    if input.name.is_empty() {
        return None;
    }
    let mut lines: Vec<String> = Vec::new();
    if !input.description.is_empty() {
        lines.push(input.description.to_string());
        lines.push(String::new());
    }
    if !input.parameters.is_empty() {
        lines.push("## Parameters".to_string());
        lines.push(String::new());
        lines.push("| Name | Type | Required | Description |".to_string());
        lines.push("|------|------|----------|-------------|".to_string());
        for (name, type_, required) in input.parameters {
            let required_str = if *required { "Yes" } else { "No" };
            let type_escaped = type_.replace('|', "\\|");
            lines.push(format!("| {name} | {type_escaped} | {required_str} |  |"));
        }
        lines.push(String::new());
    }
    if !input.response_type.is_empty() {
        lines.push("## Response".to_string());
        lines.push(String::new());
        if let Some(file_path) = input.nested_file_path {
            let url = type_github_url(file_path, sdk, models_path_rel, input.prepend_models_path);
            lines.push(format!("Returns: [`{}`]({url})", input.response_display));
        } else {
            lines.push(format!("Returns: `{}`", input.response_display));
        }
        lines.push(String::new());
    }
    if !input.code_example.is_empty() {
        lines.push("## Example".to_string());
        lines.push(String::new());
        let title = format!("{} Example", input.name);
        let lang = input.lang_tag;
        lines.push(format!(
            "[inline-code-attrs-start title = '{title}'; type = '{lang}'; isFunctional = false; inline-code-attrs-end]"
        ));
        lines.push("[inline-code-start]".to_string());
        lines.push(input.code_example.to_string());
        lines.push("[inline-code-end]".to_string());
        lines.push(String::new());
    }
    let content = lines.join("\n");
    let sub_cat = format_resource_name(input.tag, input.path);
    let filename = format!("{}-api-generated.md", sanitize_filename(input.name));
    Some(DocSection {
        name: input.name.to_string(),
        file: Some(filename),
        content,
        sub_cat: Some(sub_cat),
        type_: Some("api".to_string()),
        sidebar_item_classes: None,
    })
}

/// Single method's "cached or generate" dance, shared by every
/// per-language AI generator. Returns:
///   - `Ok(Some(code))`: cache hit, or fresh LLM call succeeded.
///   - `Ok(None)`: cache miss AND no `LLM_API_KEY` (Node treats
///                 missing-key as "skip silently" rather than fail).
///   - `Err(e)`:    LLM call attempted but failed.
pub async fn cached_or_generate<M: Serialize + ?Sized>(
    llm: &LlmClient,
    method: &M,
    prompt: &str,
) -> Result<Option<String>> {
    let meta = serde_json::to_value(method).context("serialize method")?;
    if let Some(c) = llm.get_cached(&meta, prompt).await {
        return Ok(Some(c));
    }
    if llm.api_key.is_none() {
        return Ok(None);
    }
    Ok(Some(llm.generate(&meta, prompt).await?.text))
}

/// Fan out a per-language AI generator's main loop: for each method,
/// resolve the cache (or call the LLM), then render a doc section.
/// Spawns one tokio task per method, drains in order, and returns
/// `(sections, cache_misses)`.
///
/// Every per-language generator previously carried this same ~30-line
/// pattern verbatim (FuturesUnordered + spawn + cached_or_generate +
/// section build + drain + sort + log). The only language-specific
/// bits are the prompt and section closures, which the caller passes
/// as fn pointers (avoids the FnMut+Send+Clone+'static gymnastics).
pub async fn fanout_methods<M>(
    methods: Vec<M>,
    llm: Arc<LlmClient>,
    sdk: Arc<crate::config::SdkConfig>,
    models_path: String,
    prompt_fn: fn(&M) -> String,
    section_fn: fn(&M, &str, &crate::config::SdkConfig, &str) -> Option<DocSection>,
) -> (Vec<DocSection>, usize)
where
    M: Send + Sync + Serialize + 'static,
{
    let mut tasks = FuturesUnordered::new();
    for (idx, method) in methods.into_iter().enumerate() {
        let llm = llm.clone();
        let sdk = sdk.clone();
        let models_path = models_path.clone();
        tasks.push(tokio::spawn(async move {
            let prompt = prompt_fn(&method);
            let code = match cached_or_generate(&llm, &method, &prompt).await {
                Ok(Some(c)) => c,
                Ok(None) => return (idx, Ok(None)),
                Err(e) => return (idx, Err(e)),
            };
            (idx, Ok(section_fn(&method, &code, &sdk, &models_path)))
        }));
    }
    let mut indexed: Vec<(usize, Option<DocSection>)> = Vec::new();
    let mut cache_misses = 0usize;
    while let Some(joined) = tasks.next().await {
        match joined {
            Ok((idx, Ok(maybe))) => indexed.push((idx, maybe)),
            Ok((idx, Err(e))) => {
                cache_misses += 1;
                tracing::warn!(idx, error = %e, "ai gen failed; skip");
            }
            Err(e) => {
                cache_misses += 1;
                tracing::warn!(error = %e, "ai task panicked; skip");
            }
        }
    }
    indexed.sort_by_key(|(i, _)| *i);
    let sections: Vec<DocSection> = indexed.into_iter().filter_map(|(_, s)| s).collect();
    if cache_misses > 0 {
        tracing::warn!(cache_misses, sdk = %sdk.id, "AI cache misses");
    }
    (sections, cache_misses)
}

/// End-to-end driver every per-language AI generator delegates to.
/// Bundles `fanout_methods` + the trailing `GeneratedDocs`
/// construction so each generator's `generate()` collapses to a
/// single `await`.
pub async fn run_ai_generator<M>(
    methods: Vec<M>,
    ai: AiContext,
    sdk: crate::config::SdkConfig,
    prompt_fn: fn(&M) -> String,
    section_fn: fn(&M, &str, &crate::config::SdkConfig, &str) -> Option<DocSection>,
) -> crate::generators::base::GeneratedDocs
where
    M: Send + Sync + Serialize + 'static,
{
    let (sections, _miss) = fanout_methods(
        methods,
        Arc::new(ai.llm),
        Arc::new(sdk),
        ai.models_path,
        prompt_fn,
        section_fn,
    )
    .await;
    crate::generators::base::GeneratedDocs {
        intro: None,
        conclusion: None,
        sections,
        validation_errors: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalize_first_handles_empty_and_unicode() {
        assert_eq!(capitalize_first(""), "");
        assert_eq!(capitalize_first("foo"), "Foo");
        assert_eq!(capitalize_first("éclair"), "ÉCLAIR".chars().next().unwrap().to_string() + "clair");
    }

    #[test]
    fn lowercase_first_handles_empty() {
        assert_eq!(lowercase_first(""), "");
        assert_eq!(lowercase_first("Foo"), "foo");
        assert_eq!(lowercase_first("FOO"), "fOO");
    }

    #[test]
    fn infer_resource_from_path_basic() {
        assert_eq!(infer_resource_from_path("/api/v3/comments"), "comments");
        assert_eq!(infer_resource_from_path("/api/v1/forums"), "forums");
        assert_eq!(infer_resource_from_path("/health"), "health");
        assert_eq!(infer_resource_from_path(""), "api");
    }

    #[test]
    fn format_resource_name_uses_tag_when_specific() {
        // Non-generic tag wins, regardless of path.
        assert_eq!(
            format_resource_name(Some("Forums"), Some("/api/v3/forums")),
            crate::generators::openapi::format_resource_name("Forums")
        );
    }

    #[test]
    fn format_resource_name_falls_back_to_path_when_tag_generic() {
        assert_eq!(
            format_resource_name(Some("api"), Some("/api/v3/comments")),
            crate::generators::openapi::format_resource_name("comments")
        );
    }

    #[test]
    fn format_resource_name_public_tag_becomes_misc_apis_when_path_inference_fails() {
        assert_eq!(
            format_resource_name(Some("Public"), Some("")),
            crate::generators::openapi::format_resource_name("Misc Apis")
        );
    }
}

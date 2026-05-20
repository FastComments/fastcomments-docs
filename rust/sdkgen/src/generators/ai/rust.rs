//! Rust AI doc generator. Port of
//! `src/sdk-doc-generators/rust-ai-generator.js`.

use anyhow::Result;
use async_trait::async_trait;
use fcdocs_llm::LlmClient;

use super::common;
use super::prompts;
use super::rust_parser::{snake_to_camel_case, Method, RustParser};
use crate::generators::base::{DocGenerator, DocSection, GeneratedDocs, GeneratorCtx};

pub struct RustAiGenerator;

#[async_trait]
impl DocGenerator for RustAiGenerator {
    async fn generate(&self, ctx: &GeneratorCtx) -> Result<GeneratedDocs> {
        let cfg = ctx
            .sdk
            .extra
            .as_ref()
            .and_then(|e| e.get("rustAiConfig"))
            .cloned();
        let Some(cfg) = cfg else {
            anyhow::bail!("rustAiConfig missing for SDK {}", ctx.sdk.id);
        };
        let spec_path = cfg
            .get("specPath")
            .and_then(|v| v.as_str())
            .unwrap_or("openapi.json");
        let models_path = cfg
            .get("modelsPath")
            .and_then(|v| v.as_str())
            .unwrap_or("client/src/models/");
        let api_files: Vec<String> = cfg
            .get("apiFiles")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        let spec = common::parse_openapi_spec(&ctx.repo_path.join(spec_path))?;
        let op_map = common::build_operation_map(&spec);
        let parser = RustParser::new(ctx.repo_path.clone(), models_path);
        let Some(repo_root) = common::repo_root_from(&ctx.repo_path) else {
            anyhow::bail!("could not locate repo root from {:?}", ctx.repo_path);
        };
        let cache_dir = common::ai_cache_dir(&repo_root, &ctx.sdk.id);
        let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-5-mini".to_string());
        let llm = LlmClient::new(&cache_dir, &model, "rust")?;

        let mut all_methods: Vec<Method> = Vec::new();
        for api_file in &api_files {
            let methods = parser.extract_api_methods(api_file);
            tracing::info!(file = %api_file, count = methods.len(), "parsed");
            for mut m in methods {
                let camel = snake_to_camel_case(&m.name);
                let cap = capitalize_first(&camel);
                if let Some(info) = op_map.get(&camel).or_else(|| op_map.get(&cap)) {
                    m.http_method = Some(info.method.to_uppercase());
                    m.path = Some(info.path.clone());
                    m.tag = Some(info.tag.clone().unwrap_or_else(|| "api".to_string()));
                    m.auth_type = Some(
                        if info.tag.as_deref() == Some("Public") { "none" } else { "x-api-key" }
                            .to_string(),
                    );
                    m.description = info.description.clone();
                } else {
                    tracing::warn!(method = %m.name, "no OpenAPI operation found");
                }
                all_methods.push(m);
            }
        }

        let llm = std::sync::Arc::new(llm);
        let sdk = std::sync::Arc::new(ctx.sdk.clone());
        let models_path_owned: String = models_path.to_string();
        let mut tasks = futures::stream::FuturesUnordered::new();
        for (idx, method) in all_methods.into_iter().enumerate() {
            let llm = llm.clone();
            let sdk = sdk.clone();
            let models_path = models_path_owned.clone();
            tasks.push(tokio::spawn(async move {
                let meta_value = match serde_json::to_value(&method) {
                    Ok(v) => v,
                    Err(e) => return (idx, Err(anyhow::anyhow!("serialize method: {e}"))),
                };
                let prompt = prompts::rust_prompt(&method);
                let code = match llm.get_cached(&meta_value, &prompt).await {
                    Some(c) => c,
                    None => {
                        if llm.api_key.is_none() {
                            return (idx, Ok(None));
                        }
                        match llm.generate(&meta_value, &prompt).await {
                            Ok(r) => r.text,
                            Err(e) => return (idx, Err(e)),
                        }
                    }
                };
                (idx, Ok(build_method_section(&method, &code, &sdk, &models_path)))
            }));
        }
        let mut indexed: Vec<(usize, Option<DocSection>)> = Vec::new();
        let mut cache_misses = 0usize;
        while let Some(joined) = futures::stream::StreamExt::next(&mut tasks).await {
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
            tracing::warn!(cache_misses, sdk = %ctx.sdk.id, "AI cache misses");
        }
        Ok(GeneratedDocs {
            intro: None,
            conclusion: None,
            sections,
            validation_errors: Vec::new(),
        })
    }
}

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(first) => first.to_uppercase().chain(c).collect(),
        None => String::new(),
    }
}

fn build_method_section(
    method: &Method,
    code_example: &str,
    sdk: &crate::config::SdkConfig,
    models_path_rel: &str,
) -> Option<DocSection> {
    if method.name.is_empty() {
        return None;
    }
    let mut lines: Vec<String> = Vec::new();
    if let Some(desc) = &method.description {
        if !desc.is_empty() {
            lines.push(desc.clone());
            lines.push(String::new());
        }
    }
    if !method.parameters.is_empty() {
        lines.push("## Parameters".to_string());
        lines.push(String::new());
        lines.push("| Name | Type | Required | Description |".to_string());
        lines.push("|------|------|----------|-------------|".to_string());
        for (name, info) in &method.parameters {
            let required = if info.required { "Yes" } else { "No" };
            let type_ = info.type_.replace('|', "\\|");
            lines.push(format!("| {name} | {type_} | {required} |  |"));
        }
        lines.push(String::new());
    }
    if !method.response_type.is_empty() {
        lines.push("## Response".to_string());
        lines.push(String::new());
        let nested = method.nested_types.get(&method.response_type);
        if let Some(n) = nested {
            let url = type_github_url(&n.file_path, sdk, models_path_rel);
            lines.push(format!("Returns: [`{}`]({url})", method.response_type));
        } else {
            lines.push(format!("Returns: `{}`", method.response_type));
        }
        lines.push(String::new());
    }
    if !code_example.is_empty() {
        lines.push("## Example".to_string());
        lines.push(String::new());
        let title = format!("{} Example", method.name);
        lines.push(format!(
            "[inline-code-attrs-start title = '{title}'; type = 'rust'; isFunctional = false; inline-code-attrs-end]"
        ));
        lines.push("[inline-code-start]".to_string());
        lines.push(code_example.to_string());
        lines.push("[inline-code-end]".to_string());
        lines.push(String::new());
    }
    let content = lines.join("\n");
    let sub_cat = format_resource_name(method);
    let filename = format!(
        "{}-api-generated.md",
        super::common::sanitize_filename(&method.name)
    );
    Some(DocSection {
        name: method.name.clone(),
        file: Some(filename),
        content,
        sub_cat: Some(sub_cat),
        type_: Some("api".to_string()),
        sidebar_item_classes: None,
    })
}

fn format_resource_name(method: &Method) -> String {
    let tag = method.tag.as_deref().unwrap_or("api");
    let mut resource = tag.to_string();
    if tag.is_empty() || tag == "api" || tag == "Public" {
        let path = method.path.as_deref().unwrap_or("");
        let inferred = infer_resource_from_path(path);
        if !inferred.is_empty() && inferred != "api" {
            resource = inferred;
        } else if resource == "Public" {
            resource = "Misc Apis".to_string();
        }
    }
    crate::generators::openapi::format_resource_name(&resource)
}

fn infer_resource_from_path(path: &str) -> String {
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

fn type_github_url(file_name: &str, sdk: &crate::config::SdkConfig, models_path_rel: &str) -> String {
    let base = sdk.repo.trim_end_matches(".git").trim_end_matches('/');
    format!("{base}/blob/{branch}/{models_path_rel}{file_name}", branch = sdk.branch)
}

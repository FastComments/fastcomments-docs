//! Rust AI doc generator. Port of
//! `src/sdk-doc-generators/rust-ai-generator.js`.

use anyhow::Result;
use async_trait::async_trait;

use super::common;
use super::prompts;
use super::rust_parser::{snake_to_camel_case, Method, RustParser};
use crate::generators::base::{DocGenerator, DocSection, GeneratedDocs, GeneratorCtx};

pub struct RustAiGenerator;

#[async_trait]
impl DocGenerator for RustAiGenerator {
    async fn generate(&self, ctx: &GeneratorCtx) -> Result<GeneratedDocs> {
        let ai = common::init_ai_context(
            ctx,
            "rust",
            "rustAiConfig",
            "client/src/models/",
        )?;
        let parser = RustParser::new(ctx.repo_path.clone(), &ai.models_path);

        let mut all_methods: Vec<Method> = Vec::new();
        for api_file in &ai.api_files {
            let methods = parser.extract_api_methods(api_file);
            tracing::info!(file = %api_file, count = methods.len(), "parsed");
            for mut m in methods {
                let camel = snake_to_camel_case(&m.name);
                let cap = common::capitalize_first(&camel);
                if let Some(info) = ai.op_map.get(&camel).or_else(|| ai.op_map.get(&cap)) {
                    common::apply_operation_info(&mut m, info);
                } else {
                    tracing::warn!(method = %m.name, "no OpenAPI operation found");
                }
                all_methods.push(m);
            }
        }

        let (sections, _miss) = common::fanout_methods(
            all_methods,
            std::sync::Arc::new(ai.llm),
            std::sync::Arc::new(ctx.sdk.clone()),
            ai.models_path,
            prompts::rust_prompt,
            build_method_section,
        )
        .await;
        Ok(GeneratedDocs {
            intro: None,
            conclusion: None,
            sections,
            validation_errors: Vec::new(),
        })
    }
}

fn build_method_section(
    method: &Method,
    code_example: &str,
    sdk: &crate::config::SdkConfig,
    models_path_rel: &str,
) -> Option<DocSection> {
    let params: Vec<(String, String, bool)> = method
        .parameters
        .iter()
        .map(|(k, v)| (k.clone(), v.type_.clone(), v.required))
        .collect();
    common::render_method_section(
        common::SectionInput {
            name: &method.name,
            description: method.description.as_deref().unwrap_or(""),
            parameters: &params,
            response_type: &method.response_type,
            response_display: &method.response_type,
            nested_file_path: method
                .nested_types
                .get(&method.response_type)
                .map(|n| n.file_path.as_str()),
            code_example,
            lang_tag: "rust",
            prepend_models_path: true,
            tag: method.tag.as_deref(),
            path: method.path.as_deref(),
        },
        sdk,
        models_path_rel,
    )
}

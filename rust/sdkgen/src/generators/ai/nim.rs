//! Nim AI doc generator. Port of
//! `src/sdk-doc-generators/nim-ai-generator.js`.

use anyhow::Result;
use async_trait::async_trait;

use super::common;
use super::nim_parser::{snake_to_camel, Method, NimParser};
use super::prompts;
use crate::generators::base::{DocGenerator, DocSection, GeneratedDocs, GeneratorCtx};

pub struct NimAiGenerator;

#[async_trait]
impl DocGenerator for NimAiGenerator {
    async fn generate(&self, ctx: &GeneratorCtx) -> Result<GeneratedDocs> {
        let ai = common::init_ai_context(
            ctx,
            "nim",
            "nimAiConfig",
            "client/fastcomments/models/",
        )?;
        let parser = NimParser::new(ctx.repo_path.clone(), &ai.models_path);

        let mut all_methods: Vec<Method> = Vec::new();
        for api_file in &ai.api_files {
            let methods = parser.extract_api_methods(api_file);
            tracing::info!(file = %api_file, count = methods.len(), "parsed");
            for mut m in methods {
                // 4 lookup attempts matching nim-ai-generator.js:49-68.
                let lower_first = common::lowercase_first(&m.name);
                let upper_first = common::capitalize_first(&m.name);
                let camel = snake_to_camel(&m.name);
                let info = ai.op_map
                    .get(&m.name)
                    .or_else(|| ai.op_map.get(&lower_first))
                    .or_else(|| ai.op_map.get(&upper_first))
                    .or_else(|| ai.op_map.get(&camel));
                if let Some(info) = info {
                    common::apply_operation_info(&mut m, info);
                }
                all_methods.push(m);
            }
        }

        let (sections, _miss) = common::fanout_methods(
            all_methods,
            std::sync::Arc::new(ai.llm),
            std::sync::Arc::new(ctx.sdk.clone()),
            ai.models_path,
            prompts::nim_prompt,
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
    // Nim-specific quirks vs the shared section builder:
    //   1. Skip the `httpClient` param (nim-ai-generator.js:282 —
    //      "not user-facing").
    //   2. Wrap the response type as `Option[T]` for display.
    //   3. URL builder uses the file path AS-IS (no modelsPath
    //      prepend, unlike cpp/typescript/rust).
    let params: Vec<(String, String, bool)> = method
        .parameters
        .iter()
        .filter(|(name, _)| name.as_str() != "httpClient")
        .map(|(k, v)| (k.clone(), v.type_.clone(), v.required))
        .collect();
    let wrapped_display = format!("Option[{}]", method.response_type);
    common::render_method_section(
        common::SectionInput {
            name: &method.name,
            description: &method.description,
            parameters: &params,
            response_type: &method.response_type,
            response_display: &wrapped_display,
            nested_file_path: method
                .nested_types
                .get(&method.response_type)
                .map(|n| n.file_path.as_str()),
            code_example,
            lang_tag: "nim",
            prepend_models_path: false,
            tag: method.tag.as_deref(),
            path: method.path.as_deref(),
        },
        sdk,
        models_path_rel,
    )
}

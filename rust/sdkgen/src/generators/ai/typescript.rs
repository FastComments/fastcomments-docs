//! TypeScript AI doc generator. Port of
//! `src/sdk-doc-generators/typescript-ai-generator.js`.
//!
//! Parses each configured API file, enriches each method with OpenAPI
//! info, looks up the existing cache file for the (method, prompt,
//! model) tuple, and emits a markdown section.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use super::common;
use super::prompts;
use super::typescript_parser::{Method, TypescriptParser};
use crate::generators::base::{DocGenerator, DocSection, GeneratedDocs, GeneratorCtx};

pub struct TypescriptAiGenerator;

#[async_trait]
impl DocGenerator for TypescriptAiGenerator {
    async fn generate(&self, ctx: &GeneratorCtx) -> Result<GeneratedDocs> {
        let ai = common::init_ai_context(
            ctx,
            "typescript",
            "typescriptAiConfig",
            "src/generated/src/models/",
        )?;
        let parser = TypescriptParser::new(ctx.repo_path.clone(), &ai.models_path);

        // Gather methods across all API files, enriching with OpenAPI.
        let mut all_methods: Vec<Method> = Vec::new();
        for api_file in &ai.api_files {
            let methods = parser.extract_api_methods(api_file);
            tracing::info!(file = %api_file, count = methods.len(), "parsed");
            for mut m in methods {
                if let Some(info) = ai.op_map
                    .get(&m.name)
                    .or_else(|| ai.op_map.get(&common::capitalize_first(&m.name)))
                {
                    common::apply_operation_info(&mut m, info);
                } else {
                    tracing::warn!(method = %m.name, "no OpenAPI operation found");
                }
                all_methods.push(m);
            }
        }

        // Resolve cache + emit sections, in parallel across methods. The
        // cache lookups dominate, but on cache-cold builds the OpenAI
        // calls are the bottleneck — both benefit from parallelism.
        let (sections, _miss) = common::fanout_methods(
            all_methods,
            Arc::new(ai.llm),
            Arc::new(ctx.sdk.clone()),
            ai.models_path,
            prompts::typescript_prompt,
            common::build_method_section::<Method>,
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


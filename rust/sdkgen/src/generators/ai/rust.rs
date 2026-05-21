//! Rust AI doc generator. Port of
//! `src/sdk-doc-generators/rust-ai-generator.js`.

use anyhow::Result;
use async_trait::async_trait;

use super::common;
use super::prompts;
use super::rust_parser::{snake_to_camel_case, Method, RustParser};
use crate::generators::base::{DocGenerator, GeneratedDocs, GeneratorCtx};

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
                let method_name = m.name.clone();
                if !common::enrich_with_first_match(&ai.op_map, &mut m, &[&camel, &cap]) {
                    tracing::warn!(method = %method_name, "no OpenAPI operation found");
                }
                all_methods.push(m);
            }
        }

        Ok(common::run_ai_generator(
            all_methods,
            ai,
            ctx.sdk.clone(),
            prompts::rust_prompt,
            common::build_method_section::<Method>,
        )
        .await)
    }
}


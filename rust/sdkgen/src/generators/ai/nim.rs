//! Nim AI doc generator. Port of
//! `src/sdk-doc-generators/nim-ai-generator.js`.

use anyhow::Result;
use async_trait::async_trait;

use super::common;
use super::nim_parser::{snake_to_camel, Method, NimParser};
use super::prompts;
use crate::generators::base::{DocGenerator, GeneratedDocs, GeneratorCtx};

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
                let name = m.name.clone();
                common::enrich_with_first_match(
                    &ai.op_map,
                    &mut m,
                    &[&name, &lower_first, &upper_first, &camel],
                );
                all_methods.push(m);
            }
        }

        Ok(common::run_ai_generator(
            all_methods,
            ai,
            ctx.sdk.clone(),
            prompts::nim_prompt,
            common::build_method_section::<Method>,
        )
        .await)
    }
}

// nim quirks (httpClient filter, Option[T] response wrap, no
// modelsPath prepend) live on `impl MethodForSection for Method` in
// nim_parser.rs. The shared `common::build_method_section` consumes
// them.

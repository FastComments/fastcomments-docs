//! C++ AI doc generator. Port of
//! `src/sdk-doc-generators/cpp-ai-generator.js`.

use anyhow::Result;
use async_trait::async_trait;

use super::common;
use super::cpp_parser::{CppParser, Method};
use super::prompts;
use crate::generators::base::{DocGenerator, DocSection, GeneratedDocs, GeneratorCtx};

pub struct CppAiGenerator;

#[async_trait]
impl DocGenerator for CppAiGenerator {
    async fn generate(&self, ctx: &GeneratorCtx) -> Result<GeneratedDocs> {
        let ai = common::init_ai_context(
            ctx,
            "cpp",
            "cppAiConfig",
            "client/include/FastCommentsClient/model/",
        )?;
        let parser = CppParser::new(ctx.repo_path.clone(), &ai.models_path);

        let mut all_methods: Vec<Method> = Vec::new();
        for api_file in &ai.api_files {
            let methods = parser.extract_api_methods(api_file);
            tracing::info!(file = %api_file, count = methods.len(), "parsed");
            for mut m in methods {
                // Mirrors cpp-ai-generator.js:48-72: exact, lowercase-first,
                // uppercase-first.
                let lower_first = common::lowercase_first(&m.name);
                let upper_first = common::capitalize_first(&m.name);
                let name = m.name.clone();
                common::enrich_with_first_match(
                    &ai.op_map,
                    &mut m,
                    &[&name, &lower_first, &upper_first],
                );
                all_methods.push(m);
            }
        }

        Ok(common::run_ai_generator(
            all_methods,
            ai,
            ctx.sdk.clone(),
            prompts::cpp_prompt,
            common::build_method_section::<Method>,
        )
        .await)
    }
}


//! OpenAPI doc generator stub.
//!
//! TODO(phase-3-followup): port the full 736-line generator at
//! `src/sdk-doc-generators/openapi-generator.js`. For now we emit a
//! placeholder section per SDK so the meta.json layout stays consistent;
//! the existing Node-generated content under
//! `src/content/guides/sdk-*/items/en/` keeps serving until this is
//! finished.

use anyhow::Result;
use async_trait::async_trait;

use super::base::{DocGenerator, DocSection, GeneratedDocs, GeneratorCtx};

pub struct OpenApiGenerator;

#[async_trait]
impl DocGenerator for OpenApiGenerator {
    async fn generate(&self, ctx: &GeneratorCtx) -> Result<GeneratedDocs> {
        tracing::warn!(
            sdk = %ctx.sdk.id,
            "openapi generator not yet ported — emitting placeholder; existing Node-generated files under src/content/guides/{}/items/en/ remain authoritative",
            ctx.sdk.id,
        );
        Ok(GeneratedDocs {
            intro: None,
            conclusion: None,
            sections: vec![DocSection {
                name: "API Reference".to_string(),
                file: Some("api-reference-readme-generated.md".to_string()),
                content: format!(
                    "The OpenAPI-based API reference for **{name}** is generated from the spec in [{repo}]({repo}/tree/{branch}). The Rust port of the OpenAPI generator is in progress — until it lands, the existing Node-built reference content is preserved.\n",
                    name = ctx.sdk.name,
                    repo = ctx.sdk.repo,
                    branch = ctx.sdk.branch,
                ),
                sub_cat: Some("API Reference".to_string()),
                type_: Some("api".to_string()),
                sidebar_item_classes: None,
            }],
        })
    }
}

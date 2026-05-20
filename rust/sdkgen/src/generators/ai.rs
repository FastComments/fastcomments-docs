//! AI-powered doc generator stubs for typescript / rust / cpp / nim.
//!
//! TODO(phase-3-followup): port the four AI generators in
//! `src/sdk-doc-generators/{typescript,rust,cpp,nim}-ai-generator.js`
//! plus their per-language source parsers. The shared OpenAI client
//! lives in the `fcdocs-llm` crate (already verified for cache-key
//! parity with the Node implementation). For now we emit a placeholder
//! per SDK so the meta.json layout stays consistent.

use anyhow::Result;
use async_trait::async_trait;

use super::base::{DocGenerator, DocSection, GeneratedDocs, GeneratorCtx};

pub struct AiGenerator {
    pub language: &'static str,
}

#[async_trait]
impl DocGenerator for AiGenerator {
    async fn generate(&self, ctx: &GeneratorCtx) -> Result<GeneratedDocs> {
        tracing::warn!(
            sdk = %ctx.sdk.id,
            language = %self.language,
            "AI generator not yet ported — emitting placeholder; existing Node-generated AI examples under src/content/guides/{}/items/en/ remain authoritative",
            ctx.sdk.id,
        );
        Ok(GeneratedDocs {
            intro: None,
            conclusion: None,
            sections: vec![DocSection {
                name: format!("{} AI Examples", capitalize(self.language)),
                file: Some(format!("{}-ai-examples-readme-generated.md", self.language)),
                content: format!(
                    "AI-generated code examples for **{name}** are in progress. The OpenAI integration is already ported to the `fcdocs-llm` crate; the per-language parser and prompt wiring is being finished. Existing Node-generated examples remain authoritative until cutover.\n",
                    name = ctx.sdk.name,
                ),
                sub_cat: Some("Examples".to_string()),
                type_: Some("api".to_string()),
                sidebar_item_classes: None,
            }],
        })
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(first) => first.to_uppercase().chain(c).collect(),
        None => String::new(),
    }
}

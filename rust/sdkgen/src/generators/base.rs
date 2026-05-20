//! Common types and trait for SDK doc generators. Mirrors
//! `src/sdk-doc-generators/base-generator.js`'s contract surface.

use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::config::SdkConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocSection {
    pub name: String,
    /// Optional override for the output filename. If `None`, sdkgen
    /// derives one via `sanitize_filename(name)`.
    #[serde(default)]
    pub file: Option<String>,
    pub content: String,
    #[serde(default, rename = "subCat")]
    pub sub_cat: Option<String>,
    /// Either `"readme"` or `"api"`.
    #[serde(default, rename = "type")]
    pub type_: Option<String>,
    #[serde(default, rename = "sidebarItemClasses")]
    pub sidebar_item_classes: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct GeneratedDocs {
    pub intro: Option<String>,
    pub conclusion: Option<String>,
    pub sections: Vec<DocSection>,
    /// Per-operation problems that did not abort the whole generator
    /// but should fail the build. The OpenAPI generator emits one
    /// entry per "method not found in SDK docs" or "return type not
    /// extractable" case, matching the validation pass in Node's
    /// src/sdk-guide-generator.js:268-309. `guide::generate_all`
    /// aggregates these across SDKs and bails non-zero at end of run.
    pub validation_errors: Vec<String>,
}

pub struct GeneratorCtx {
    pub sdk: SdkConfig,
    pub repo_path: PathBuf,
}

#[async_trait]
pub trait DocGenerator: Send + Sync {
    async fn generate(&self, ctx: &GeneratorCtx) -> Result<GeneratedDocs>;
}

//! SDK config loader. Mirrors `src/content/sdk-repos.json` shape.

use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdkConfig {
    pub id: String,
    pub name: String,
    #[serde(default, rename = "pageHeader")]
    pub page_header: Option<String>,
    pub repo: String,
    pub branch: String,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    /// Legacy single-generator field.
    #[serde(default, rename = "docGenerator")]
    pub doc_generator: Option<String>,
    /// New multi-generator field.
    #[serde(default, rename = "docGenerators")]
    pub doc_generators: Option<Vec<String>>,
    #[serde(default)]
    pub description: Option<String>,
}

impl SdkConfig {
    pub fn generators(&self) -> Vec<String> {
        if let Some(list) = &self.doc_generators {
            return list.clone();
        }
        if let Some(g) = &self.doc_generator {
            return vec![g.clone()];
        }
        vec!["readme".to_string()]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoConfig {
    #[serde(default, rename = "checkoutDirectory")]
    pub checkout_directory: Option<String>,
    #[serde(default, rename = "guidesDirectory")]
    pub guides_directory: Option<String>,
    pub sdks: Vec<SdkConfig>,
}

pub fn load(path: &std::path::Path) -> Result<RepoConfig> {
    let bytes = std::fs::read(path).with_context(|| format!("read {path:?}"))?;
    let cfg: RepoConfig = serde_json::from_slice(&bytes).context("parse sdk-repos.json")?;
    Ok(cfg)
}

pub fn default_config_path(repo_root: &std::path::Path) -> PathBuf {
    repo_root.join("src/content/sdk-repos.json")
}

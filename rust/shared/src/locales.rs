use std::path::Path;

use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Locale {
    pub name: String,
    #[serde(rename = "nativeName")]
    pub native_name: String,
    pub hreflang: String,
    #[serde(default)]
    pub flag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Locales {
    #[serde(rename = "defaultLocale")]
    pub default_locale: String,
    /// Insertion-ordered to preserve `src/locales.json`'s original
    /// declaration order — used by alternate-hreflang link emission to
    /// match Node's `Object.keys(locales)` ordering.
    pub locales: IndexMap<String, Locale>,
}

impl Locales {
    pub fn load_from(path: impl AsRef<Path>) -> Result<Self> {
        let bytes = std::fs::read(path.as_ref())
            .with_context(|| format!("read locales json at {:?}", path.as_ref()))?;
        let parsed: Self = serde_json::from_slice(&bytes).context("parse locales json")?;
        Ok(parsed)
    }

    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.locales.keys().map(|s| s.as_str())
    }
}

/// Convert hreflang format (e.g. "zh-CN") to locale key format ("zh_cn").
///
/// Mirrors `hreflangToLocaleKey` in `src/server-search-engine.js:197-201`.
pub fn hreflang_to_locale_key(hreflang: Option<&str>, default_locale: &str) -> String {
    match hreflang {
        None | Some("") => default_locale.to_string(),
        Some(s) => s.to_ascii_lowercase().replace('-', "_"),
    }
}

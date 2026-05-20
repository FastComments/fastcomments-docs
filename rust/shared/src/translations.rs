//! Loads UI translations from `src/translations.json`. Mirrors the lookup
//! at `src/guides.js:20-23` and `src/app.js:21-24`.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result};
use once_cell::sync::OnceCell;
use serde_json::Value;

/// Per-locale string map: { KEY -> translated_text }.
pub type LocaleMap = HashMap<String, String>;

#[derive(Debug, Clone, Default)]
pub struct Translations {
    by_locale: HashMap<String, LocaleMap>,
    default_locale: String,
}

impl Translations {
    pub fn load_from(path: impl AsRef<Path>, default_locale: impl Into<String>) -> Result<Self> {
        let bytes = std::fs::read(path.as_ref())
            .with_context(|| format!("read translations json {:?}", path.as_ref()))?;
        let parsed: Value = serde_json::from_slice(&bytes).context("parse translations json")?;
        let mut by_locale = HashMap::new();
        if let Value::Object(map) = parsed {
            for (loc, inner) in map {
                if let Value::Object(kv) = inner {
                    let mut m = LocaleMap::new();
                    for (k, v) in kv {
                        if let Some(s) = v.as_str() {
                            m.insert(k, s.to_string());
                        }
                    }
                    by_locale.insert(loc, m);
                }
            }
        }
        Ok(Self {
            by_locale,
            default_locale: default_locale.into(),
        })
    }

    /// Get translations for a locale, falling back to the default locale.
    pub fn for_locale(&self, locale: &str) -> LocaleMap {
        if let Some(m) = self.by_locale.get(locale) {
            return m.clone();
        }
        if let Some(m) = self.by_locale.get(&self.default_locale) {
            return m.clone();
        }
        LocaleMap::new()
    }
}

/// Process-wide singleton — convenient for templates that need `t.KEY`
/// access without threading state everywhere. Populated by sitegen at
/// startup.
static GLOBAL: OnceCell<Arc<Translations>> = OnceCell::new();

pub fn install(translations: Translations) {
    let _ = GLOBAL.set(Arc::new(translations));
}

pub fn global() -> Option<Arc<Translations>> {
    GLOBAL.get().cloned()
}

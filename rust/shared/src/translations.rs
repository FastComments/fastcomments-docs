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

/// Pre-rendered template context for a single locale. Holds an Arc to
/// the underlying HashMap and a pre-serialized `serde_json::Value`
/// view for handlebars rendering. Cheap to clone — both fields are
/// reference-counted.
#[derive(Debug, Clone)]
pub struct LocaleSlot {
    pub map: Arc<LocaleMap>,
    pub value: Arc<Value>,
}

#[derive(Debug, Clone, Default)]
pub struct Translations {
    by_locale: HashMap<String, LocaleSlot>,
    default_locale: String,
    empty_slot: LocaleSlot,
}

impl Translations {
    pub fn load_from(path: impl AsRef<Path>, default_locale: impl Into<String>) -> Result<Self> {
        let bytes = std::fs::read(path.as_ref())
            .with_context(|| format!("read translations json {:?}", path.as_ref()))?;
        let parsed: Value = serde_json::from_slice(&bytes).context("parse translations json")?;
        let mut by_locale: HashMap<String, LocaleSlot> = HashMap::new();
        if let Value::Object(map) = parsed {
            for (loc, inner) in map {
                if let Value::Object(kv) = inner {
                    let mut m = LocaleMap::new();
                    let mut v = serde_json::Map::new();
                    for (k, val) in kv {
                        if let Some(s) = val.as_str() {
                            m.insert(k.clone(), s.to_string());
                            v.insert(k, Value::String(s.to_string()));
                        }
                    }
                    by_locale.insert(
                        loc,
                        LocaleSlot {
                            map: Arc::new(m),
                            value: Arc::new(Value::Object(v)),
                        },
                    );
                }
            }
        }
        Ok(Self {
            by_locale,
            default_locale: default_locale.into(),
            empty_slot: LocaleSlot {
                map: Arc::new(LocaleMap::new()),
                value: Arc::new(Value::Object(serde_json::Map::new())),
            },
        })
    }

    /// Get translations for a locale, falling back to the default locale.
    /// Returns an `Arc<LocaleMap>` so the per-locale data is shared, not
    /// deep-cloned (the old impl rebuilt the full HashMap on every call,
    /// which was ~50 guides × 28 locales × clones per build).
    pub fn for_locale(&self, locale: &str) -> LocaleSlot {
        if let Some(m) = self.by_locale.get(locale) {
            return m.clone();
        }
        if let Some(m) = self.by_locale.get(&self.default_locale) {
            return m.clone();
        }
        self.empty_slot.clone()
    }
}

impl Default for LocaleSlot {
    fn default() -> Self {
        Self {
            map: Arc::new(LocaleMap::new()),
            value: Arc::new(Value::Object(serde_json::Map::new())),
        }
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

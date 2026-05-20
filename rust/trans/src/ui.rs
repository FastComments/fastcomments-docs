//! UI strings translation pipeline. Port of
//! `src/translate-with-gpt.js::processUITranslations` (lines 581-649)
//! and `translateUIStrings` (lines 451-573).
//!
//! For each non-default locale, build a single batch of all missing or
//! stale keys (one OpenAI call per locale instead of per key), translate
//! the batch as a JSON object, merge into `src/translations.json`, then
//! update `src/ui-translation-cache.json` with md5(sourceValue) per
//! translated key.
//!
//! Per-locale processing is sequential (matches Node — `for (const
//! [locale, ...] of Object.entries)` at translate-with-gpt.js:590).
//! That keeps writes to translations.json deterministic since each
//! per-locale batch is one merge + one save.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result};
use fcdocs_shared::locales::Locales;
use serde_json::{json, Value};
use tracing::{info, warn};

use crate::openai::JsonTranslator;
use crate::snapshot::hash_content;

#[derive(Debug, Default)]
pub struct UiStats {
    pub success: usize,
    pub failed: usize,
    pub skipped: usize,
}

pub async fn process_all(
    translator: Arc<JsonTranslator>,
    repo: &Path,
    locales: &Locales,
) -> Result<UiStats> {
    let translations_path = repo.join("src/translations.json");
    let ui_cache_path = repo.join("src/ui-translation-cache.json");
    let mut stats = UiStats::default();

    let mut translations: Value = if translations_path.exists() {
        serde_json::from_slice(&tokio::fs::read(&translations_path).await?)
            .context("parse translations.json")?
    } else {
        json!({})
    };
    let mut ui_cache: BTreeMap<String, String> = if ui_cache_path.exists() {
        serde_json::from_slice(&tokio::fs::read(&ui_cache_path).await?).unwrap_or_default()
    } else {
        BTreeMap::new()
    };

    // Source strings = translations[default_locale]. Clone so we don't
    // borrow `translations` while mutating it below.
    let source_strings: serde_json::Map<String, Value> = translations
        .get(&locales.default_locale)
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default();

    for (locale_key, _) in &locales.locales {
        if locale_key == &locales.default_locale {
            continue;
        }
        let needs = compute_needs(&source_strings, &translations, &ui_cache, locale_key);
        if needs.is_empty() {
            continue;
        }
        let n_missing = needs
            .iter()
            .filter(|n| n.kind == NeedKind::Missing)
            .count();
        let n_stale = needs.len() - n_missing;
        info!(
            locale = %locale_key,
            missing = n_missing,
            stale = n_stale,
            "translating UI batch"
        );

        // Build `toTranslate = { key: source_value }`.
        let mut to_translate = serde_json::Map::new();
        for n in &needs {
            if let Some(v) = source_strings.get(&n.key) {
                to_translate.insert(n.key.clone(), v.clone());
            }
        }

        // en_us shortcut — Node copies source verbatim
        // (translate-with-gpt.js:621). Save the cycle and the cost.
        let translated: serde_json::Map<String, Value> = if locale_key == "en_us" {
            to_translate.clone()
        } else {
            let (system, prompt) = build_system_and_prompt(
                locales,
                locale_key,
                &to_translate,
            );
            match translator
                .translate_to_json(&system, &prompt, &format!("UI strings for {locale_key}"))
                .await
            {
                Ok(t) => t,
                Err(e) => {
                    warn!(locale = %locale_key, error = %e, "UI translation failed");
                    stats.failed += 1;
                    continue;
                }
            }
        };

        // Merge into translations[locale]. Create the locale map if it
        // doesn't exist (Node line 625-627).
        let locale_obj = translations
            .as_object_mut()
            .ok_or_else(|| anyhow::anyhow!("translations.json root must be an object"))?
            .entry(locale_key.clone())
            .or_insert_with(|| Value::Object(serde_json::Map::new()));
        if !locale_obj.is_object() {
            anyhow::bail!("translations[{locale_key}] is not an object");
        }
        let locale_map = locale_obj.as_object_mut().expect("checked above");
        for (k, v) in &translated {
            locale_map.insert(k.clone(), v.clone());
        }

        // Persist translations.json after each locale batch (matches
        // Node's per-batch save at line 629).
        write_translations_atomic(&translations_path, &translations).await?;

        // Update UI cache with md5(sourceValue) for every translated key.
        for k in translated.keys() {
            let Some(src_str) = source_strings.get(k).and_then(|v| v.as_str()) else {
                continue;
            };
            let cache_key = format!("{locale_key}/{k}");
            ui_cache.insert(cache_key, hash_content(src_str));
        }
        write_ui_cache_atomic(&ui_cache_path, &ui_cache).await?;

        stats.success += 1;
    }

    Ok(stats)
}

#[derive(Debug, PartialEq, Eq)]
enum NeedKind {
    Missing,
    Stale,
}

#[derive(Debug)]
struct UiNeed {
    key: String,
    kind: NeedKind,
}

/// Per-locale needs list mirroring
/// src/check-translations.js::getMissingUITranslations: keys that exist
/// in the source but either don't exist in this locale (missing) or
/// have an out-of-date hash in ui-translation-cache.json (stale).
fn compute_needs(
    source_strings: &serde_json::Map<String, Value>,
    translations: &Value,
    ui_cache: &BTreeMap<String, String>,
    locale: &str,
) -> Vec<UiNeed> {
    let locale_obj = translations.get(locale).and_then(|v| v.as_object());
    let mut out = Vec::new();
    for (k, source_val) in source_strings {
        if locale_obj.and_then(|m| m.get(k)).is_none() {
            out.push(UiNeed {
                key: k.clone(),
                kind: NeedKind::Missing,
            });
            continue;
        }
        let Some(src_str) = source_val.as_str() else {
            continue;
        };
        let expected = hash_content(src_str);
        let fresh = ui_cache
            .get(&format!("{locale}/{k}"))
            .map(|cached| cached == &expected)
            .unwrap_or(false);
        if !fresh {
            out.push(UiNeed {
                key: k.clone(),
                kind: NeedKind::Stale,
            });
        }
    }
    out
}

/// Mirrors translate-with-gpt.js:455-463. The system message names
/// the target locale; the user prompt embeds a 2-space-indented JSON
/// dump of the strings to translate.
fn build_system_and_prompt(
    locales: &Locales,
    locale_key: &str,
    to_translate: &serde_json::Map<String, Value>,
) -> (String, String) {
    let native = locales
        .locales
        .get(locale_key)
        .map(|l| l.native_name.clone())
        .unwrap_or_else(|| locale_key.to_string());
    let system = format!(
        "You are an expert translator. Translate UI strings from English to {native} ({locale_key}).\n\
         Return ONLY a valid JSON object with the same keys but translated values.\n\
         Do not include any explanation or markdown formatting - just the raw JSON."
    );
    // JSON.stringify(obj, null, 2) — serde_json::to_string_pretty
    // produces 2-space indent. preserve_order feature on serde_json
    // means insertion-order is preserved (workspace Cargo.toml).
    let payload = serde_json::to_string_pretty(to_translate)
        .unwrap_or_else(|_| "{}".to_string());
    let prompt = format!(
        "Translate these UI strings to {native}:\n\n\
         {payload}\n\n\
         Return ONLY a valid JSON object with the translated values. No explanations."
    );
    (system, prompt)
}

async fn write_translations_atomic(path: &Path, value: &Value) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value)?;
    let tmp = path.with_extension("json.tmp");
    tokio::fs::write(&tmp, &bytes)
        .await
        .with_context(|| format!("write tmp {tmp:?}"))?;
    tokio::fs::rename(&tmp, path)
        .await
        .with_context(|| format!("rename {tmp:?} -> {path:?}"))?;
    Ok(())
}

async fn write_ui_cache_atomic(path: &Path, cache: &BTreeMap<String, String>) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(cache)?;
    let tmp = path.with_extension("json.tmp");
    tokio::fs::write(&tmp, &bytes)
        .await
        .with_context(|| format!("write tmp {tmp:?}"))?;
    tokio::fs::rename(&tmp, path)
        .await
        .with_context(|| format!("rename {tmp:?} -> {path:?}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fcdocs_shared::locales::Locale;
    use indexmap::IndexMap;

    fn locales_en_fr() -> Locales {
        let mut m: IndexMap<String, Locale> = IndexMap::new();
        m.insert(
            "en".into(),
            Locale {
                name: "English".into(),
                native_name: "English".into(),
                hreflang: "en".into(),
                flag: None,
            },
        );
        m.insert(
            "fr_fr".into(),
            Locale {
                name: "French".into(),
                native_name: "Français (France)".into(),
                hreflang: "fr-FR".into(),
                flag: None,
            },
        );
        Locales {
            default_locale: "en".into(),
            locales: m,
        }
    }

    #[test]
    fn needs_includes_missing_and_stale() {
        let mut src = serde_json::Map::new();
        src.insert("A".into(), json!("Hello"));
        src.insert("B".into(), json!("World"));
        src.insert("C".into(), json!("Edited"));
        let t = json!({
            "en": src.clone(),
            "fr_fr": {
                // A: missing entirely
                "B": "Monde", // fresh — matches cache
                "C": "Édité"  // stale — cache has old hash
            }
        });
        let mut cache = BTreeMap::new();
        cache.insert("fr_fr/B".into(), hash_content("World"));
        cache.insert("fr_fr/C".into(), hash_content("OldValue")); // mismatch
        let needs = compute_needs(&src, &t, &cache, "fr_fr");
        let keys: Vec<&str> = needs.iter().map(|n| n.key.as_str()).collect();
        assert!(keys.contains(&"A"));
        assert!(keys.contains(&"C"));
        assert!(!keys.contains(&"B"), "fresh B should not be in needs");
    }

    #[test]
    fn prompt_format_matches_node() {
        let mut to_translate = serde_json::Map::new();
        to_translate.insert("GREETING".into(), json!("Hello"));
        to_translate.insert("FAREWELL".into(), json!("Goodbye"));
        let (sys, prompt) = build_system_and_prompt(
            &locales_en_fr(),
            "fr_fr",
            &to_translate,
        );
        // System message text matches Node line 455-457 verbatim
        // (English -> Français (France) (fr_fr) + format directive).
        assert!(sys.contains("Français (France) (fr_fr)"));
        assert!(sys.contains("Return ONLY a valid JSON object"));
        // User prompt embeds 2-space-indented JSON of the strings.
        assert!(prompt.contains("Translate these UI strings to Français (France):"));
        assert!(prompt.contains("\"GREETING\": \"Hello\""));
        assert!(prompt.contains("\"FAREWELL\": \"Goodbye\""));
        // 2-space indent
        assert!(prompt.contains("\n  \""));
    }
}

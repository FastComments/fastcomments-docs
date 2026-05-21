//! `trans check` — port of `src/check-translations.js`.
//!
//! Walks `src/content/guides/`, identifies missing locale translations
//! by comparing against `src/translation-cache.json`, and reports gaps.
//! Exits non-zero when gaps are found so build.sh can branch into
//! `trans run`. Skips guides that haven't been migrated to the
//! per-locale `items/<locale>/` structure (matches Node behavior).

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;
use fcdocs_shared::locales::Locales;
use tracing::{info, warn};

use crate::discover::default_locale_files;
use crate::snapshot::hash_content;

// Same shape as run::CacheMap. BTreeMap serializes to a JSON object
// matching translation-cache.json's on-disk layout.
type CacheMap = BTreeMap<String, String>;

pub async fn run() -> Result<()> {
    let repo = repo_root()?;
    let guides_dir = repo.join("src/content/guides");
    let translations_path = repo.join("src/translations.json");
    let cache_path = repo.join("src/translation-cache.json");
    let ui_cache_path = repo.join("src/ui-translation-cache.json");
    let locales_path = repo.join("src/locales.json");

    let locales = Locales::load_from(&locales_path)?;
    let cache: CacheMap = if cache_path.exists() {
        serde_json::from_slice(&std::fs::read(&cache_path)?).unwrap_or_default()
    } else {
        CacheMap::default()
    };
    // UI cache: `{locale}/{key}` -> md5(default-locale source value).
    // Stale = cache entry differs from current source hash (the EN
    // copy was edited but the per-locale translation wasn't refreshed).
    // Matches Node src/check-translations.js:283-296.
    let ui_cache: CacheMap = if ui_cache_path.exists() {
        serde_json::from_slice(&std::fs::read(&ui_cache_path)?).unwrap_or_default()
    } else {
        CacheMap::default()
    };
    let translations_json: serde_json::Value = if translations_path.exists() {
        serde_json::from_slice(&std::fs::read(&translations_path)?).unwrap_or(serde_json::Value::Null)
    } else {
        serde_json::Value::Null
    };

    let mut gaps = 0usize;
    let mut needs_migration = Vec::new();
    let mut missing_files: Vec<(String, String, String)> = Vec::new(); // (guide, locale, file)
    let mut inline_code_errors: Vec<(String, String, String, (usize, usize), (usize, usize))> =
        Vec::new(); // (guide, locale, file, expected_start_end, actual_start_end)

    for entry in std::fs::read_dir(&guides_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let guide_id = entry.file_name().to_string_lossy().into_owned();
        let meta_path = entry.path().join("meta.json");
        if !meta_path.exists() {
            continue;
        }
        let items_path = entry.path().join("items");
        if !items_path.exists() {
            continue;
        }
        let default_items = items_path.join(&locales.default_locale);
        if !default_items.exists()
            && !entry.path().join("intro.md").exists()
            && !entry.path().join("conclusion.md").exists()
        {
            // No locale-structured items and no root-level
            // intro/conclusion either — pre-locale flat structure
            // needing `migrate-to-locale-structure.js`.
            needs_migration.push(guide_id);
            continue;
        }
        // Use the same discovery helper Node's getDefaultLocaleFiles
        // mirrors (check-translations.js:141-161). It picks up every
        // .md under items/<defaultLocale>/ AND falls back to root-level
        // intro.md / conclusion.md when items/<defaultLocale>/ lacks
        // them. Without that fallback, ~157 root-level files across
        // ~80 guides were silently excluded from both staleness checks
        // and translation runs.
        for src in default_locale_files(&entry.path(), &locales.default_locale) {
            let filename = src.filename.clone();
            let raw = std::fs::read_to_string(&src.source_path)?;
            // Source too small to translate (e.g. empty intro.md /
            // conclusion.md placeholders). run.rs would just skip
            // these with no work and no target write, so check
            // shouldn't pretend they're missing — that creates an
            // infinite re-enqueue loop where each run "translates"
            // the same set without ever satisfying check.
            if crate::snapshot::source_is_too_small_to_translate(&raw) {
                continue;
            }
            let source_hash = hash_content(&raw);
            let default_counts = count_inline_code(&raw);

            for (locale_key, _) in &locales.locales {
                if locale_key == &locales.default_locale {
                    continue;
                }
                let target = items_path.join(locale_key).join(&filename);
                let cache_key = format!("{guide_id}/{locale_key}/{filename}");
                let cached_hash = cache.get(&cache_key);
                let needs = !target.exists() || cached_hash != Some(&source_hash);
                if needs {
                    gaps += 1;
                    missing_files.push((guide_id.clone(), locale_key.clone(), filename.clone()));
                    continue;
                }
                // File exists and source hash matches cache — also verify
                // inline-code token counts match the source. Mirrors
                // src/check-translations.js:442-464.
                if let Ok(translated) = std::fs::read_to_string(&target) {
                    let locale_counts = count_inline_code(&translated);
                    if locale_counts != default_counts {
                        inline_code_errors.push((
                            guide_id.clone(),
                            locale_key.clone(),
                            filename.clone(),
                            default_counts,
                            locale_counts,
                        ));
                    }
                }
            }
        }
    }

    // UI translations gap check. See audit_ui_translations.
    let ui_audit = audit_ui_translations(&translations_json, &ui_cache, &locales);
    let ui_missing = ui_audit.missing;
    let ui_stale = ui_audit.stale_count;
    let stale_sample = ui_audit.stale_sample;

    // meta.json gap check. See audit_meta_translations.
    let meta_audit = audit_meta_translations(&guides_dir, &cache, &locales);
    let meta_missing = meta_audit.missing_count;
    let meta_stale = meta_audit.stale_count;

    info!(
        missing_translations = gaps,
        missing_ui_strings = ui_missing,
        stale_ui_strings = ui_stale,
        missing_meta_json = meta_missing,
        stale_meta_json = meta_stale,
        inline_code_mismatches = inline_code_errors.len(),
    );
    if !stale_sample.is_empty() {
        info!("first 10 stale UI strings (source value edited since translation):");
        for (l, k) in &stale_sample {
            info!("  {l}/{k}");
        }
    }
    if !meta_audit.missing_sample.is_empty() {
        info!("first 10 missing meta.json translations:");
        for (g, l) in &meta_audit.missing_sample {
            info!("  {g}/{l}");
        }
    }
    if !meta_audit.stale_sample.is_empty() {
        info!("first 10 stale meta.json translations (source edited):");
        for (g, l) in &meta_audit.stale_sample {
            info!("  {g}/{l}");
        }
    }
    // Combine for the final exit gate. Both classes block the build.
    let ui_gaps = ui_missing + ui_stale;
    let meta_gaps = meta_missing + meta_stale;
    if !inline_code_errors.is_empty() {
        info!("first 10 inline-code mismatches:");
        for (g, l, f, (es, ee), (as_, ae)) in inline_code_errors.iter().take(10) {
            info!(
                "  {g}/{l}/{f}  expected {es} start / {ee} end, actual {as_} start / {ae} end"
            );
        }
    }
    if !needs_migration.is_empty() {
        warn!(
            count = needs_migration.len(),
            "guides need migration to per-locale structure (run migrate-to-locale-structure.js)"
        );
    }
    if !missing_files.is_empty() {
        info!("first 10 missing translations:");
        for (g, l, f) in missing_files.iter().take(10) {
            info!("  {g}/{l}/{f}");
        }
    }
    if gaps > 0
        || ui_gaps > 0
        || meta_gaps > 0
        || !needs_migration.is_empty()
        || !inline_code_errors.is_empty()
    {
        std::process::exit(1);
    }
    info!("all translations up to date");
    Ok(())
}

/// Outcome of a UI-translation audit: counts of each failure mode +
/// a bounded sample of stale (locale, key) pairs for log output.
struct UiAudit {
    missing: usize,
    stale_count: usize,
    stale_sample: Vec<(String, String)>,
}

/// Walk `translations_json` (the `src/translations.json` shape) and
/// flag two failure modes:
///
///   * missing — key exists in the default locale but not in the target.
///   * stale  — key exists in both, but md5(default-locale source value)
///     differs from `ui_cache["{locale}/{key}"]`. That's the case the
///     prior Rust port silently passed because it only checked presence.
///     Mirrors Node `src/check-translations.js:283-296`.
///
/// Factored out of `run` so tests can drive it without
/// `std::process::exit`.
fn audit_ui_translations(
    translations_json: &serde_json::Value,
    ui_cache: &CacheMap,
    locales: &Locales,
) -> UiAudit {
    const SAMPLE_CAP: usize = 10;
    let mut missing = 0usize;
    let mut stale_count = 0usize;
    let mut stale_sample: Vec<(String, String)> = Vec::new();
    let Some(default_obj) = translations_json
        .get(&locales.default_locale)
        .and_then(|v| v.as_object())
    else {
        return UiAudit {
            missing,
            stale_count,
            stale_sample,
        };
    };
    for (locale_key, _) in &locales.locales {
        if locale_key == &locales.default_locale {
            continue;
        }
        let locale_obj = translations_json
            .get(locale_key)
            .and_then(|v| v.as_object());
        for (default_key, source_val) in default_obj {
            match locale_obj.and_then(|m| m.get(default_key)) {
                None => missing += 1,
                Some(_) => {
                    let Some(src_str) = source_val.as_str() else {
                        continue;
                    };
                    let expected = hash_content(src_str);
                    let cache_key = format!("{locale_key}/{default_key}");
                    let fresh = ui_cache
                        .get(&cache_key)
                        .map(|cached| cached == &expected)
                        .unwrap_or(false);
                    if !fresh {
                        stale_count += 1;
                        if stale_sample.len() < SAMPLE_CAP {
                            stale_sample.push((locale_key.clone(), default_key.clone()));
                        }
                    }
                }
            }
        }
    }
    UiAudit {
        missing,
        stale_count,
        stale_sample,
    }
}

/// Outcome of a meta.json translation audit. `missing_count` covers
/// the case Node's check-translations.js:226-244 catches (no
/// `meta_translated/meta_<locale>.json` file at all). `stale_count`
/// extends that with hash-based staleness — the same check
/// `translate-with-gpt.js:842` (`isCached`) uses to decide whether
/// `trans run` would re-translate. Surfacing it here means `trans
/// check` exits non-zero so build.sh branches into the translator,
/// instead of reporting "all translations up to date" while
/// meta.json edits go untranslated.
///
/// The cache key matches `getCacheKey(guideId, locale, "meta.json")`
/// in Node — `{guideId}/{locale}/meta.json` — so the existing
/// `src/translation-cache.json` entries remain valid hits.
struct MetaAudit {
    missing_count: usize,
    missing_sample: Vec<(String, String)>, // (guide_id, locale)
    stale_count: usize,
    stale_sample: Vec<(String, String)>,
}

fn audit_meta_translations(
    guides_dir: &Path,
    cache: &CacheMap,
    locales: &Locales,
) -> MetaAudit {
    const SAMPLE_CAP: usize = 10;
    let mut missing_count = 0usize;
    let mut missing_sample: Vec<(String, String)> = Vec::new();
    let mut stale_count = 0usize;
    let mut stale_sample: Vec<(String, String)> = Vec::new();

    let Ok(entries) = std::fs::read_dir(guides_dir) else {
        return MetaAudit {
            missing_count,
            missing_sample,
            stale_count,
            stale_sample,
        };
    };

    for entry in entries.flatten() {
        let Ok(ft) = entry.file_type() else { continue };
        if !ft.is_dir() {
            continue;
        }
        let guide_id = entry.file_name().to_string_lossy().into_owned();
        let meta_path = entry.path().join("meta.json");
        if !meta_path.exists() {
            continue;
        }
        // Hash the source meta.json once per guide. Mirrors
        // translate-with-gpt.js:833-834.
        let source_hash = match std::fs::read_to_string(&meta_path) {
            Ok(s) => Some(hash_content(&s)),
            Err(_) => None,
        };
        for (locale_key, _) in &locales.locales {
            if locale_key == &locales.default_locale {
                continue;
            }
            let translated = entry
                .path()
                .join("meta_translated")
                .join(format!("meta_{locale_key}.json"));
            if !translated.exists() {
                missing_count += 1;
                if missing_sample.len() < SAMPLE_CAP {
                    missing_sample.push((guide_id.clone(), locale_key.clone()));
                }
                continue;
            }
            // File present. Compare cache entry against current source
            // hash — same shape as translate-with-gpt.js's isCached.
            let Some(expected) = source_hash.as_deref() else {
                continue;
            };
            let cache_key = format!("{guide_id}/{locale_key}/meta.json");
            let fresh = cache
                .get(&cache_key)
                .map(|cached| cached == expected)
                .unwrap_or(false);
            if !fresh {
                stale_count += 1;
                if stale_sample.len() < SAMPLE_CAP {
                    stale_sample.push((guide_id.clone(), locale_key.clone()));
                }
            }
        }
    }
    MetaAudit {
        missing_count,
        missing_sample,
        stale_count,
        stale_sample,
    }
}

fn count_inline_code(content: &str) -> (usize, usize) {
    // Mirrors `countInlineCode` in src/check-translations.js:72-76.
    let start = content.matches("[inline-code-start]").count();
    let end = content.matches("[inline-code-end]").count();
    (start, end)
}

use fcdocs_shared::repo::repo_root;

#[cfg(test)]
mod test_fixtures {
    use super::*;
    use fcdocs_shared::locales::Locale;
    use indexmap::IndexMap;

    /// Build a `Locales` fixture with the given keys (first key is the
    /// default). Each `Locale` gets stub names/hreflang derived from
    /// the key. Hoisted out of the per-test-mod helpers so the two
    /// audit-test modules don't carry their own copies.
    pub fn locales_with(keys: &[&str]) -> Locales {
        let mut m: IndexMap<String, Locale> = IndexMap::new();
        for k in keys {
            m.insert(
                (*k).to_string(),
                Locale {
                    name: (*k).into(),
                    native_name: (*k).into(),
                    hreflang: (*k).into(),
                    flag: None,
                },
            );
        }
        Locales {
            default_locale: keys
                .first()
                .copied()
                .expect("locales_with requires at least one key")
                .to_string(),
            locales: m,
        }
    }
}

#[cfg(test)]
mod ui_audit_tests {
    use super::*;
    use super::test_fixtures::locales_with;
    use serde_json::json;

    fn locales_en_fr() -> Locales {
        locales_with(&["en", "fr_fr"])
    }

    fn cache_map<I, K, V>(entries: I) -> CacheMap
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let mut b = BTreeMap::new();
        for (k, v) in entries {
            b.insert(k.into(), v.into());
        }
        b
    }

    #[test]
    fn missing_key_is_flagged() {
        let t = json!({
            "en": {"GREETING": "Hello"},
            "fr_fr": {}
        });
        let r = audit_ui_translations(&t, &cache_map([] as [(String, String); 0]), &locales_en_fr());
        assert_eq!(r.missing, 1);
        assert_eq!(r.stale_count, 0);
    }

    #[test]
    fn stale_translation_is_flagged_when_source_value_edited() {
        // EN source: "Hello, world" — translated to fr_fr.
        // The translation was cached against MD5("Hello, world").
        // Now EN changed to "Hello, world!" — cache hash no longer
        // matches → stale (and the prior Rust port would have silently
        // returned "all translations up to date").
        let original = "Hello, world";
        let original_hash = hash_content(original);
        let edited = "Hello, world!";

        let t = json!({
            "en": {"GREETING": edited},
            "fr_fr": {"GREETING": "Bonjour le monde"}
        });
        let cache = cache_map([(format!("fr_fr/GREETING"), original_hash)]);
        let r = audit_ui_translations(&t, &cache, &locales_en_fr());
        assert_eq!(r.missing, 0, "no missing — key exists in both locales");
        assert_eq!(r.stale_count, 1, "stale: fr_fr/GREETING source value changed");
        assert_eq!(r.stale_sample, vec![("fr_fr".into(), "GREETING".into())]);
        // Sanity: pinning the expected stale hash is unnecessary; the
        // staleness criterion is just `cached != md5(source)`.
        let _ = edited;
    }

    #[test]
    fn fresh_translation_is_not_flagged() {
        let value = "Documentation";
        let h = hash_content(value);
        let t = json!({
            "en": {"TITLE": value},
            "fr_fr": {"TITLE": "Documentation"}
        });
        let cache = cache_map([(format!("fr_fr/TITLE"), h)]);
        let r = audit_ui_translations(&t, &cache, &locales_en_fr());
        assert_eq!(r.missing, 0);
        assert_eq!(r.stale_count, 0);
    }

    #[test]
    fn translation_present_but_no_cache_entry_is_stale() {
        // Translation file exists for a key but the UI cache has no
        // entry. Node treats this as stale (the translation was
        // hand-written without going through the cache-aware tooling).
        let t = json!({
            "en": {"K": "X"},
            "fr_fr": {"K": "Y"}
        });
        let r = audit_ui_translations(&t, &cache_map([] as [(String, String); 0]), &locales_en_fr());
        assert_eq!(r.missing, 0);
        assert_eq!(r.stale_count, 1);
    }

    #[test]
    fn non_string_source_values_are_skipped() {
        // Defensive: if translations.json ever contains a non-string
        // value (nested object, number), don't crash and don't flag —
        // matches Node hashContent which only operates on strings.
        let t = json!({
            "en": {"NUMERIC": 42, "NESTED": {"x": "y"}, "OK": "fine"},
            "fr_fr": {"NUMERIC": 42, "NESTED": {"x": "y"}, "OK": "bien"}
        });
        let h = hash_content("fine");
        let cache = cache_map([(format!("fr_fr/OK"), h)]);
        let r = audit_ui_translations(&t, &cache, &locales_en_fr());
        assert_eq!(r.missing, 0);
        assert_eq!(r.stale_count, 0);
    }

    #[test]
    fn sample_caps_at_10_but_count_does_not() {
        // 12 stale entries — sample should hold 10, count should be 12.
        let mut en = serde_json::Map::new();
        let mut fr = serde_json::Map::new();
        for i in 0..12 {
            en.insert(format!("KEY{i}"), json!(format!("v{i}")));
            fr.insert(format!("KEY{i}"), json!(format!("t{i}")));
        }
        let t = json!({"en": en, "fr_fr": fr});
        let r = audit_ui_translations(&t, &cache_map([] as [(String, String); 0]), &locales_en_fr());
        assert_eq!(r.missing, 0);
        assert_eq!(r.stale_count, 12);
        assert_eq!(r.stale_sample.len(), 10);
    }
}

#[cfg(test)]
mod meta_audit_tests {
    use super::*;
    use super::test_fixtures::locales_with;

    fn locales_en_fr_de() -> Locales {
        locales_with(&["en", "fr_fr", "de_de"])
    }

    fn make_guide_with_meta(
        guides_dir: &Path,
        guide_id: &str,
        meta_body: &str,
        translated: &[(&str, &str)],
    ) {
        let g = guides_dir.join(guide_id);
        std::fs::create_dir_all(&g).unwrap();
        std::fs::write(g.join("meta.json"), meta_body).unwrap();
        if !translated.is_empty() {
            let td = g.join("meta_translated");
            std::fs::create_dir_all(&td).unwrap();
            for (locale, body) in translated {
                std::fs::write(td.join(format!("meta_{locale}.json")), body).unwrap();
            }
        }
    }

    fn cache_with<I: IntoIterator<Item = (String, String)>>(entries: I) -> CacheMap {
        let mut b = BTreeMap::new();
        for (k, v) in entries {
            b.insert(k, v);
        }
        b
    }

    #[test]
    fn missing_meta_translated_file_is_flagged() {
        let tmp = tempfile::tempdir().unwrap();
        // Source meta.json exists; neither locale has a translated copy.
        make_guide_with_meta(tmp.path(), "guide-a", "{\"name\":\"A\"}", &[]);
        let r = audit_meta_translations(tmp.path(), &cache_with([]), &locales_en_fr_de());
        assert_eq!(r.missing_count, 2, "fr_fr + de_de both missing");
        assert_eq!(r.stale_count, 0);
        assert!(r.missing_sample.iter().any(|(g, l)| g == "guide-a" && l == "fr_fr"));
        assert!(r.missing_sample.iter().any(|(g, l)| g == "guide-a" && l == "de_de"));
    }

    #[test]
    fn fresh_meta_translation_is_not_flagged() {
        let tmp = tempfile::tempdir().unwrap();
        let body = "{\"name\":\"A\"}";
        make_guide_with_meta(
            tmp.path(),
            "guide-a",
            body,
            &[
                ("fr_fr", "{\"name\":\"Le A\"}"),
                ("de_de", "{\"name\":\"Das A\"}"),
            ],
        );
        let h = hash_content(body);
        let cache = cache_with([
            ("guide-a/fr_fr/meta.json".into(), h.clone()),
            ("guide-a/de_de/meta.json".into(), h.clone()),
        ]);
        let r = audit_meta_translations(tmp.path(), &cache, &locales_en_fr_de());
        assert_eq!(r.missing_count, 0);
        assert_eq!(r.stale_count, 0);
    }

    #[test]
    fn stale_meta_translation_is_flagged() {
        // Source has been edited since the translation was cached.
        let tmp = tempfile::tempdir().unwrap();
        make_guide_with_meta(
            tmp.path(),
            "guide-a",
            "{\"name\":\"Edited A\"}",
            &[
                ("fr_fr", "{\"name\":\"Le A\"}"),
                ("de_de", "{\"name\":\"Das A\"}"),
            ],
        );
        let old_hash = hash_content("{\"name\":\"A\"}");
        let cache = cache_with([
            ("guide-a/fr_fr/meta.json".into(), old_hash.clone()),
            ("guide-a/de_de/meta.json".into(), old_hash),
        ]);
        let r = audit_meta_translations(tmp.path(), &cache, &locales_en_fr_de());
        assert_eq!(r.missing_count, 0);
        assert_eq!(r.stale_count, 2);
    }

    #[test]
    fn translation_present_without_cache_entry_is_stale() {
        // File written by hand, no cache entry — matches Node's
        // isCached semantics (no entry means treat as needing
        // translation).
        let tmp = tempfile::tempdir().unwrap();
        make_guide_with_meta(
            tmp.path(),
            "guide-a",
            "{\"name\":\"A\"}",
            &[("fr_fr", "{\"name\":\"Le A\"}")],
        );
        let r = audit_meta_translations(tmp.path(), &cache_with([]), &locales_en_fr_de());
        // de_de missing (no file), fr_fr present-but-no-cache -> stale.
        assert_eq!(r.missing_count, 1);
        assert_eq!(r.stale_count, 1);
    }

    #[test]
    fn guide_without_source_meta_is_ignored() {
        // Some directories under guides/ are empty / unrelated; if
        // there's no meta.json we shouldn't fabricate gaps.
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("guide-empty")).unwrap();
        let r = audit_meta_translations(tmp.path(), &cache_with([]), &locales_en_fr_de());
        assert_eq!(r.missing_count, 0);
        assert_eq!(r.stale_count, 0);
    }

    #[test]
    fn nonexistent_guides_dir_returns_zero() {
        // Defensive — early call before scaffolding shouldn't panic.
        let r = audit_meta_translations(
            Path::new("/definitely/does/not/exist/nowhere"),
            &cache_with([]),
            &locales_en_fr_de(),
        );
        assert_eq!(r.missing_count, 0);
        assert_eq!(r.stale_count, 0);
    }
}

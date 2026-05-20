//! `trans check` — port of `src/check-translations.js`.
//!
//! Walks `src/content/guides/`, identifies missing locale translations
//! by comparing against `src/translation-cache.json`, and reports gaps.
//! Exits non-zero when gaps are found so build.sh can branch into
//! `trans run`. Skips guides that haven't been migrated to the
//! per-locale `items/<locale>/` structure (matches Node behavior).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use fcdocs_shared::locales::Locales;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::snapshot::hash_content;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
struct CacheMap(BTreeMap<String, String>);

pub async fn run() -> Result<()> {
    let repo = repo_root()?;
    let guides_dir = repo.join("src/content/guides");
    let translations_path = repo.join("src/translations.json");
    let cache_path = repo.join("src/translation-cache.json");
    let locales_path = repo.join("src/locales.json");

    let locales = Locales::load_from(&locales_path)?;
    let cache: CacheMap = if cache_path.exists() {
        serde_json::from_slice(&std::fs::read(&cache_path)?).unwrap_or_default()
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
        if !default_items.exists() {
            // Pre-locale flat structure — needs `migrate-to-locale-structure.js`.
            needs_migration.push(guide_id);
            continue;
        }
        // For each .md in the default locale, check every non-default locale.
        let default_files: Vec<PathBuf> = std::fs::read_dir(&default_items)?
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
            .collect();

        for src_path in &default_files {
            let filename = src_path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_string();
            let raw = std::fs::read_to_string(src_path)?;
            let source_hash = hash_content(&raw);

            let default_counts = count_inline_code(&raw);

            for (locale_key, _) in &locales.locales {
                if locale_key == &locales.default_locale {
                    continue;
                }
                let target = items_path.join(locale_key).join(&filename);
                let cache_key = format!("{guide_id}/{locale_key}/{filename}");
                let cached_hash = cache.0.get(&cache_key);
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

    // UI translations gap check.
    let mut ui_gaps = 0usize;
    if let Some(default_obj) = translations_json
        .get(&locales.default_locale)
        .and_then(|v| v.as_object())
    {
        for (locale_key, _) in &locales.locales {
            if locale_key == &locales.default_locale {
                continue;
            }
            let locale_obj = translations_json
                .get(locale_key)
                .and_then(|v| v.as_object());
            for default_key in default_obj.keys() {
                if locale_obj
                    .and_then(|m| m.get(default_key))
                    .is_none()
                {
                    ui_gaps += 1;
                }
            }
        }
    }

    info!(
        missing_translations = gaps,
        missing_ui_strings = ui_gaps,
        inline_code_mismatches = inline_code_errors.len(),
    );
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
    if gaps > 0 || ui_gaps > 0 || !needs_migration.is_empty() || !inline_code_errors.is_empty() {
        std::process::exit(1);
    }
    info!("all translations up to date");
    Ok(())
}

fn count_inline_code(content: &str) -> (usize, usize) {
    // Mirrors `countInlineCode` in src/check-translations.js:72-76.
    let start = content.matches("[inline-code-start]").count();
    let end = content.matches("[inline-code-end]").count();
    (start, end)
}

fn repo_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let mut cur: &Path = cwd.as_path();
    loop {
        if cur.join("package.json").exists() && cur.join("src/locales.json").exists() {
            return Ok(cur.to_path_buf());
        }
        match cur.parent() {
            Some(p) => cur = p,
            None => anyhow::bail!("could not locate repo root from {cwd:?}"),
        }
    }
}

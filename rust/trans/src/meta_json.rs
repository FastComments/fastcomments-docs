//! meta.json translation pipeline. Port of
//! `src/translate-with-gpt.js::processMetaJsonTranslations`
//! (lines 857-942) + `translateMetaJson` (lines 659-813).
//!
//! For each guide × non-default-locale where the cached source hash
//! doesn't match the current source meta.json: build a flat map of
//! translatable strings (guideName / pageHeader / item_<i>_name /
//! item_<i>_subCat), translate them as one JSON object, reassemble
//! the meta object with translated values, and write to
//! `meta_translated/meta_<locale>.json`. Then update the shared
//! `translation-cache.json` keyed by `{guideId}/{locale}/meta.json`.
//!
//! Per-(guide × locale) tasks run in parallel up to `concurrency`,
//! matching Node's worker pool at translate-with-gpt.js:934-937.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::{Context, Result};
use fcdocs_shared::locales::Locales;
use futures::stream::{FuturesUnordered, StreamExt};
use serde_json::{json, Value};
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::openai::JsonTranslator;
use crate::snapshot::hash_content;

#[derive(Debug, Default)]
pub struct MetaStats {
    pub success: usize,
    pub failed: usize,
    pub skipped: usize,
}

/// CLI-derived options for the meta.json phase. Mirrors the subset of
/// trans::Options that applies to per-guide meta translation.
#[derive(Debug, Default, Clone)]
pub struct Options {
    pub filter_locale: Option<String>,
    pub filter_guide: Option<String>,
    pub dry_run: bool,
    pub force: bool,
}

#[derive(Debug, Clone)]
struct Task {
    guide_id: String,
    locale: String,
    source_hash: String,
}

#[allow(clippy::too_many_arguments)]
pub async fn process_all(
    translator: Arc<JsonTranslator>,
    repo: &Path,
    locales: Arc<Locales>,
    cache: Arc<Mutex<BTreeMap<String, String>>>,
    dirty: Arc<AtomicBool>,
    concurrency: usize,
    opts: Options,
) -> Result<MetaStats> {
    let guides_dir = Arc::new(repo.join("src/content/guides"));
    let tasks = build_task_list(
        &guides_dir,
        &locales,
        &*cache.lock().await,
        opts.filter_locale.as_deref(),
        opts.filter_guide.as_deref(),
        opts.force,
    )
    .await;
    if tasks.is_empty() {
        return Ok(MetaStats::default());
    }
    info!(
        count = tasks.len(),
        concurrency,
        force = opts.force,
        dry_run = opts.dry_run,
        locale = opts.filter_locale.as_deref().unwrap_or("*"),
        guide = opts.filter_guide.as_deref().unwrap_or("*"),
        "meta.json translation batch"
    );
    let dry_run = opts.dry_run;

    let success = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let failed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let skipped = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let mut in_flight = FuturesUnordered::new();
    let mut iter = tasks.into_iter();

    let spawn = |task: Task, in_flight: &mut FuturesUnordered<_>| {
        let translator = translator.clone();
        let locales = locales.clone();
        let guides_dir = guides_dir.clone();
        let cache = cache.clone();
        let dirty = dirty.clone();
        let success = success.clone();
        let failed = failed.clone();
        let skipped = skipped.clone();
        in_flight.push(tokio::spawn(async move {
            match process_one(
                &task,
                &guides_dir,
                &locales,
                &translator,
                &cache,
                &dirty,
                dry_run,
            )
            .await
            {
                Ok(Outcome::Translated) => {
                    success.fetch_add(1, Ordering::Relaxed);
                }
                Ok(Outcome::Skipped) => {
                    skipped.fetch_add(1, Ordering::Relaxed);
                }
                Err(e) => {
                    warn!(
                        guide = %task.guide_id,
                        locale = %task.locale,
                        error = %format!("{e:#}"),
                        "meta.json translation failed"
                    );
                    failed.fetch_add(1, Ordering::Relaxed);
                }
            }
        }));
    };

    while in_flight.len() < concurrency {
        let Some(t) = iter.next() else { break };
        spawn(t, &mut in_flight);
    }
    while let Some(joined) = in_flight.next().await {
        if let Err(e) = joined {
            warn!(error = %e, "meta task panicked");
            failed.fetch_add(1, Ordering::Relaxed);
        }
        if let Some(t) = iter.next() {
            spawn(t, &mut in_flight);
        }
    }

    Ok(MetaStats {
        success: success.load(Ordering::Relaxed),
        failed: failed.load(Ordering::Relaxed),
        skipped: skipped.load(Ordering::Relaxed),
    })
}

enum Outcome {
    Translated,
    Skipped,
}

#[allow(clippy::too_many_arguments)]
async fn process_one(
    task: &Task,
    guides_dir: &Path,
    locales: &Locales,
    translator: &JsonTranslator,
    cache: &Mutex<BTreeMap<String, String>>,
    dirty: &AtomicBool,
    dry_run: bool,
) -> Result<Outcome> {
    if dry_run {
        // Matches Node line 890-894 — log + skip + don't touch cache.
        info!(
            guide = %task.guide_id,
            locale = %task.locale,
            "[dry-run] would translate meta.json"
        );
        return Ok(Outcome::Skipped);
    }
    let meta_path = guides_dir.join(&task.guide_id).join("meta.json");
    let meta_bytes = tokio::fs::read(&meta_path)
        .await
        .with_context(|| format!("read source meta {meta_path:?}"))?;
    let meta: Value =
        serde_json::from_slice(&meta_bytes).context("parse source meta.json")?;

    // en_us is a verbatim copy of the source (Node line 900-901).
    let translated_meta = if task.locale == "en_us" {
        meta.clone()
    } else {
        let to_translate = build_to_translate(&meta);
        if to_translate.is_empty() {
            return Ok(Outcome::Skipped);
        }
        let (system, prompt) = build_system_and_prompt(locales, &task.locale, &to_translate);
        let label = format!("meta.json for {}/{}", task.guide_id, task.locale);
        let translated = translator
            .translate_to_json(&system, &prompt, &label)
            .await
            .context("translate meta.json")?;
        merge_translated(&meta, &translated)
    };

    // Write meta_translated/meta_<locale>.json
    let out_dir = guides_dir.join(&task.guide_id).join("meta_translated");
    tokio::fs::create_dir_all(&out_dir).await?;
    let out_path = out_dir.join(format!("meta_{}.json", task.locale));
    let pretty = serde_json::to_vec_pretty(&translated_meta)?;
    let tmp = out_path.with_extension("json.tmp");
    tokio::fs::write(&tmp, &pretty)
        .await
        .with_context(|| format!("write tmp {tmp:?}"))?;
    tokio::fs::rename(&tmp, &out_path)
        .await
        .with_context(|| format!("rename {tmp:?} -> {out_path:?}"))?;

    // Update cache. The cache flusher in run.rs handles persistence.
    {
        let mut c = cache.lock().await;
        c.insert(
            format!("{}/{}/meta.json", task.guide_id, task.locale),
            task.source_hash.clone(),
        );
        dirty.store(true, Ordering::Release);
    }
    Ok(Outcome::Translated)
}

async fn build_task_list(
    guides_dir: &Path,
    locales: &Locales,
    cache: &BTreeMap<String, String>,
    filter_locale: Option<&str>,
    filter_guide: Option<&str>,
    force: bool,
) -> Vec<Task> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(guides_dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let Ok(ft) = entry.file_type() else { continue };
        if !ft.is_dir() {
            continue;
        }
        let guide_id = entry.file_name().to_string_lossy().into_owned();
        if let Some(filter) = filter_guide {
            if guide_id != filter {
                continue;
            }
        }
        let meta_path = entry.path().join("meta.json");
        let Ok(meta_str) = std::fs::read_to_string(&meta_path) else {
            continue;
        };
        let source_hash = hash_content(&meta_str);
        for (locale_key, _) in &locales.locales {
            if locale_key == &locales.default_locale {
                continue;
            }
            if let Some(filter) = filter_locale {
                if locale_key != filter {
                    continue;
                }
            }
            let cache_key = format!("{guide_id}/{locale_key}/meta.json");
            let cached = cache.get(&cache_key);
            if force || cached != Some(&source_hash) {
                out.push(Task {
                    guide_id: guide_id.clone(),
                    locale: locale_key.clone(),
                    source_hash: source_hash.clone(),
                });
            }
        }
    }
    out
}

/// Mirrors translate-with-gpt.js:663-676 — flatten the strings the
/// model needs to translate into a single map. Item index ordering
/// must be preserved across the round-trip, so we use insertion-
/// ordered Map (preserve_order on serde_json).
fn build_to_translate(meta: &Value) -> serde_json::Map<String, Value> {
    let mut to = serde_json::Map::new();
    if let Some(name) = meta.get("name").and_then(|v| v.as_str()) {
        to.insert("guideName".into(), Value::String(name.to_string()));
    }
    if let Some(ph) = meta.get("pageHeader").and_then(|v| v.as_str()) {
        if !ph.is_empty() {
            to.insert("pageHeader".into(), Value::String(ph.to_string()));
        }
    }
    if let Some(items) = meta.get("itemsOrdered").and_then(|v| v.as_array()) {
        for (idx, item) in items.iter().enumerate() {
            if let Some(name) = item.get("name").and_then(|v| v.as_str()) {
                to.insert(format!("item_{idx}_name"), Value::String(name.to_string()));
            }
            if let Some(sc) = item.get("subCat").and_then(|v| v.as_str()) {
                to.insert(
                    format!("item_{idx}_subCat"),
                    Value::String(sc.to_string()),
                );
            }
        }
    }
    to
}

/// Mirrors translate-with-gpt.js:763-778. Returns a fully-rebuilt
/// meta object with the same shape as the source, but with the
/// translated `name` / `pageHeader` and item `name`/`subCat`.
/// Per-item fields fall back to the original if a key is missing
/// from the LLM response (matches `item_<i>_name || item.name`).
fn merge_translated(
    source: &Value,
    translated: &serde_json::Map<String, Value>,
) -> Value {
    let mut out = source.clone();
    let out_obj = match out.as_object_mut() {
        Some(o) => o,
        None => return out,
    };
    if let Some(guide_name) = translated.get("guideName").and_then(|v| v.as_str()) {
        out_obj.insert("name".into(), Value::String(guide_name.to_string()));
    }
    if source.get("pageHeader").is_some() {
        if let Some(ph) = translated.get("pageHeader").and_then(|v| v.as_str()) {
            out_obj.insert("pageHeader".into(), Value::String(ph.to_string()));
        }
    }
    if let Some(items_val) = out_obj.get_mut("itemsOrdered") {
        if let Some(items) = items_val.as_array_mut() {
            for (idx, item_val) in items.iter_mut().enumerate() {
                let Some(item_obj) = item_val.as_object_mut() else {
                    continue;
                };
                if let Some(n) = translated
                    .get(&format!("item_{idx}_name"))
                    .and_then(|v| v.as_str())
                {
                    item_obj.insert("name".into(), Value::String(n.to_string()));
                }
                if let Some(sc) = translated
                    .get(&format!("item_{idx}_subCat"))
                    .and_then(|v| v.as_str())
                {
                    item_obj.insert("subCat".into(), Value::String(sc.to_string()));
                }
            }
        }
    }
    out
}

/// Mirrors translate-with-gpt.js:678-686.
fn build_system_and_prompt(
    locales: &Locales,
    locale_key: &str,
    to_translate: &serde_json::Map<String, Value>,
) -> (String, String) {
    let native = locales.native_name_or_key(locale_key);
    let system = format!(
        "You are an expert translator. Translate metadata from English to {native} ({locale_key}).\n\
         Return ONLY a valid JSON object with the same keys but translated values.\n\
         Do not include any explanation or markdown formatting - just the raw JSON."
    );
    let payload =
        serde_json::to_string_pretty(to_translate).unwrap_or_else(|_| "{}".to_string());
    let prompt = format!(
        "Translate these guide metadata strings to {native}:\n\n\
         {payload}\n\n\
         Return ONLY a valid JSON object with the translated values. No explanations."
    );
    (system, prompt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_to_translate_flat_keys() {
        let meta = json!({
            "name": "My Guide",
            "pageHeader": "A header",
            "itemsOrdered": [
                {"name": "Intro", "subCat": "Getting Started", "file": "intro.md"},
                {"name": "Setup", "subCat": "Getting Started", "file": "setup.md"}
            ]
        });
        let to = build_to_translate(&meta);
        assert_eq!(to.get("guideName").and_then(|v| v.as_str()), Some("My Guide"));
        assert_eq!(to.get("pageHeader").and_then(|v| v.as_str()), Some("A header"));
        assert_eq!(to.get("item_0_name").and_then(|v| v.as_str()), Some("Intro"));
        assert_eq!(to.get("item_0_subCat").and_then(|v| v.as_str()), Some("Getting Started"));
        assert_eq!(to.get("item_1_name").and_then(|v| v.as_str()), Some("Setup"));
        // Non-string fields aren't in toTranslate.
        assert!(to.get("file").is_none());
    }

    #[test]
    fn build_to_translate_omits_empty_pageheader() {
        // Node check `if (meta.pageHeader)` at line 668 — empty string
        // is falsy in JS. Match that.
        let meta = json!({"name": "G", "pageHeader": "", "itemsOrdered": []});
        let to = build_to_translate(&meta);
        assert!(to.get("pageHeader").is_none());
    }

    #[test]
    fn merge_translated_preserves_shape() {
        let source = json!({
            "name": "Source",
            "icon": "x.png",
            "itemsOrdered": [
                {"name": "Old", "subCat": "Old Cat", "file": "a.md", "type": "html"}
            ]
        });
        let mut t = serde_json::Map::new();
        t.insert("guideName".into(), json!("Translated"));
        t.insert("item_0_name".into(), json!("New"));
        t.insert("item_0_subCat".into(), json!("New Cat"));
        let merged = merge_translated(&source, &t);
        assert_eq!(merged["name"], "Translated");
        assert_eq!(merged["icon"], "x.png");
        let item = &merged["itemsOrdered"][0];
        assert_eq!(item["name"], "New");
        assert_eq!(item["subCat"], "New Cat");
        assert_eq!(item["file"], "a.md");
        assert_eq!(item["type"], "html");
    }

    #[test]
    fn merge_translated_falls_back_per_item_field() {
        // Node line 776-777 — if LLM omitted a key, fall back to the
        // source value. Required for partial response robustness.
        let source = json!({
            "name": "S",
            "itemsOrdered": [{"name": "Keep", "subCat": "KeepCat"}]
        });
        let mut t = serde_json::Map::new();
        t.insert("guideName".into(), json!("T"));
        // No item_0_name; LLM dropped it.
        t.insert("item_0_subCat".into(), json!("NewCat"));
        let merged = merge_translated(&source, &t);
        assert_eq!(merged["itemsOrdered"][0]["name"], "Keep");
        assert_eq!(merged["itemsOrdered"][0]["subCat"], "NewCat");
    }

    #[test]
    fn prompt_mentions_locale_and_payload() {
        let mut to = serde_json::Map::new();
        to.insert("guideName".into(), json!("My Guide"));
        let locales = test_locales();
        let (sys, prompt) = build_system_and_prompt(&locales, "fr_fr", &to);
        assert!(sys.contains("Translate metadata from English to Français (France) (fr_fr)"));
        assert!(prompt.contains("Translate these guide metadata strings to Français (France):"));
        assert!(prompt.contains("\"guideName\": \"My Guide\""));
    }

    fn locales_from_pairs(pairs: &[(&str, &str)]) -> Locales {
        use fcdocs_shared::locales::Locale;
        use indexmap::IndexMap;
        let mut m: IndexMap<String, Locale> = IndexMap::new();
        for (k, n) in pairs {
            m.insert(
                (*k).into(),
                Locale {
                    name: (*k).into(),
                    native_name: (*n).into(),
                    hreflang: (*k).into(),
                    flag: None,
                },
            );
        }
        Locales {
            default_locale: pairs
                .first()
                .map(|(k, _)| (*k).into())
                .unwrap_or_else(|| "en".into()),
            locales: m,
        }
    }

    fn test_locales() -> Locales {
        locales_from_pairs(&[("en", "English"), ("fr_fr", "Français (France)")])
    }

    // --- filter + force behavior on build_task_list ---

    fn write_guide(root: &Path, guide_id: &str, body: &str) {
        let g = root.join(guide_id);
        std::fs::create_dir_all(&g).unwrap();
        std::fs::write(g.join("meta.json"), body).unwrap();
    }

    fn locales_en_fr_de() -> Locales {
        locales_from_pairs(&[
            ("en", "English"),
            ("fr_fr", "Français (France)"),
            ("de_de", "Deutsch"),
        ])
    }

    #[tokio::test]
    async fn build_task_list_no_filter_yields_all_locales() {
        let tmp = tempfile::tempdir().unwrap();
        write_guide(tmp.path(), "g", "{\"name\":\"G\"}");
        let cache = BTreeMap::new();
        let tasks = build_task_list(tmp.path(), &locales_en_fr_de(), &cache, None, None, false).await;
        assert_eq!(tasks.len(), 2); // fr_fr + de_de
    }

    #[tokio::test]
    async fn build_task_list_locale_filter() {
        let tmp = tempfile::tempdir().unwrap();
        write_guide(tmp.path(), "g", "{\"name\":\"G\"}");
        let cache = BTreeMap::new();
        let tasks = build_task_list(
            tmp.path(),
            &locales_en_fr_de(),
            &cache,
            Some("fr_fr"),
            None,
            false,
        )
        .await;
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].locale, "fr_fr");
    }

    #[tokio::test]
    async fn build_task_list_guide_filter() {
        let tmp = tempfile::tempdir().unwrap();
        write_guide(tmp.path(), "guide-a", "{\"name\":\"A\"}");
        write_guide(tmp.path(), "guide-b", "{\"name\":\"B\"}");
        let cache = BTreeMap::new();
        let tasks = build_task_list(
            tmp.path(),
            &locales_en_fr_de(),
            &cache,
            None,
            Some("guide-a"),
            false,
        )
        .await;
        assert_eq!(tasks.len(), 2);
        assert!(tasks.iter().all(|t| t.guide_id == "guide-a"));
    }

    #[tokio::test]
    async fn build_task_list_force_ignores_cache() {
        let tmp = tempfile::tempdir().unwrap();
        let body = "{\"name\":\"G\"}";
        write_guide(tmp.path(), "g", body);
        let h = hash_content(body);
        let mut cache = BTreeMap::new();
        cache.insert("g/fr_fr/meta.json".into(), h.clone());
        cache.insert("g/de_de/meta.json".into(), h);

        let normal =
            build_task_list(tmp.path(), &locales_en_fr_de(), &cache, None, None, false).await;
        assert_eq!(normal.len(), 0, "cache is fresh for both locales");

        let forced =
            build_task_list(tmp.path(), &locales_en_fr_de(), &cache, None, None, true).await;
        assert_eq!(forced.len(), 2, "--force re-includes every (guide × locale)");
    }
}

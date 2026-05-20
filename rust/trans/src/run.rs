//! `trans run` — port of `src/translate-with-gpt.js`.
//!
//! Walks the same per-locale items tree as `trans check`, identifies
//! missing or stale translations, and calls OpenAI for each. Prompts
//! and cache keys mirror the Node implementation byte-for-byte so
//! existing `translation-cache.json` entries remain valid hits.
//!
//! Scope of the port:
//! - Markdown file translations (the main use case — `processTranslations`).
//! - Cache load/save in the same JSON shape Node uses.
//! - Inline-code-attrs sanitization (`sanitizeInlineCodeAttrs`).
//!
//! Out of scope here (covered separately if/when needed):
//! - UI strings translation (`translations.json`) — covered by the
//!   `processUITranslations` path in the Node version.
//! - meta.json translation — covered by `processMetaJsonTranslations`.
//! Both are mechanical to port using the same LlmClient; deferred to
//! a follow-up.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use fcdocs_shared::locales::Locales;
use futures::stream::{FuturesUnordered, StreamExt};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::meta_json;
use crate::openai::JsonTranslator;
use crate::snapshot::hash_content;
use crate::ui;

const DEFAULT_MODEL: &str = "gpt-5-mini";

/// Matches Node's translate-with-gpt.js CLI default
/// (`parseArgs() options.concurrency = 20` at line 1001). The
/// in-process function defaults (lines 322, 858) are 5, but the
/// CLI overrides them when called via `node src/translate-with-gpt.js`,
/// which is what build.sh invokes. Mirroring the CLI default so a
/// production trans run finishes in the same wall time.
const DEFAULT_CONCURRENCY: usize = 20;

/// How often the background flusher writes a dirty cache to disk.
/// Bounded data loss on crash; total I/O is O(run_duration / interval)
/// instead of O(tasks).
const CACHE_FLUSH_INTERVAL: Duration = Duration::from_secs(15);

// Cache shape on disk: { "guideId/locale/filename": "md5hex", ... }.
// BTreeMap serializes directly to a sorted-key JSON object via
// serde_json, matching Node's translation-cache.json layout. Shared by
// the markdown items path AND the meta.json path (Node's
// translate-with-gpt.js uses one cache for both, keyed differently).
type CacheMap = BTreeMap<String, String>;

pub async fn run() -> Result<()> {
    let repo = repo_root()?;
    let guides_dir = Arc::new(repo.join("src/content/guides"));
    let cache_path = Arc::new(repo.join("src/translation-cache.json"));
    let locales_path = repo.join("src/locales.json");
    let locales = Arc::new(Locales::load_from(&locales_path)?);
    let cache_initial: CacheMap = if cache_path.exists() {
        serde_json::from_slice(&tokio::fs::read(&*cache_path).await?).unwrap_or_default()
    } else {
        CacheMap::default()
    };

    let tasks = build_task_list(&guides_dir, &locales, &cache_initial)?;
    info!(count = tasks.len(), "missing markdown translation tasks");

    let api_key = std::env::var("OPENAI_API_KEY").ok();
    if api_key.is_none() {
        warn!("OPENAI_API_KEY not set — `trans run` cannot call OpenAI.");
        anyhow::bail!("OPENAI_API_KEY required");
    }

    let model = Arc::new(std::env::var("OPENAI_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string()));
    // 120s per request is generous for chat/completions (Node default
    // was 600s, but that just hides hung connections). Connect timeout
    // is shorter so we fail fast on routing problems.
    let client = Arc::new(
        reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(120))
            .build()?,
    );
    let api_key = Arc::new(api_key.unwrap());
    let cache = Arc::new(Mutex::new(cache_initial));
    let dirty = Arc::new(AtomicBool::new(false));

    // Background cache flusher. Writes the cache JSON at most once per
    // CACHE_FLUSH_INTERVAL, and only when entries have been added since
    // the last write. Used to be: full-file rewrite after every
    // successful task, which made cache I/O O(N^2) for N tasks.
    let flusher_handle = {
        let cache = cache.clone();
        let dirty = dirty.clone();
        let cache_path = cache_path.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(CACHE_FLUSH_INTERVAL);
            ticker.tick().await; // skip first immediate tick
            loop {
                ticker.tick().await;
                if let Err(e) = flush_cache_if_dirty(&cache, &dirty, &cache_path).await {
                    warn!(error = %format!("{e:#}"), "cache flush failed (will retry on next tick)");
                }
            }
        })
    };

    let concurrency: usize = std::env::var("TRANS_CONCURRENCY")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|n: &usize| *n > 0)
        .unwrap_or(DEFAULT_CONCURRENCY);
    info!(concurrency, "translation concurrency");

    let success = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let failed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let skipped = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let mut in_flight = FuturesUnordered::new();
    let mut iter = tasks.into_iter();

    let drive = |task: Task, in_flight: &mut FuturesUnordered<_>| {
        let guides_dir = guides_dir.clone();
        let cache = cache.clone();
        let dirty = dirty.clone();
        let locales = locales.clone();
        let client = client.clone();
        let api_key = api_key.clone();
        let model = model.clone();
        let success = success.clone();
        let failed = failed.clone();
        let skipped = skipped.clone();
        in_flight.push(tokio::spawn(async move {
            let res = process_one_task(
                &task,
                &guides_dir,
                &cache,
                &dirty,
                &locales,
                &client,
                &api_key,
                &model,
            )
            .await;
            match res {
                Ok(Outcome::Success) => {
                    success.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
                Ok(Outcome::Skipped) => {
                    skipped.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
                Err(e) => {
                    warn!(
                        "[error] {}/{}/{}: {e:#}",
                        task.guide_id, task.locale, task.filename
                    );
                    failed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            }
        }));
    };

    while in_flight.len() < concurrency {
        let Some(t) = iter.next() else { break };
        drive(t, &mut in_flight);
    }
    while let Some(joined) = in_flight.next().await {
        if let Err(e) = joined {
            warn!(error = %e, "translation task panicked");
            failed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        if let Some(t) = iter.next() {
            drive(t, &mut in_flight);
        }
    }

    let s = success.load(std::sync::atomic::Ordering::Relaxed);
    let f = failed.load(std::sync::atomic::Ordering::Relaxed);
    let k = skipped.load(std::sync::atomic::Ordering::Relaxed);
    info!(success = s, failed = f, skipped = k, "markdown items phase complete");

    // Phase 2: UI strings. Per-locale batches, sequential
    // (translate-with-gpt.js processes them with a single for-of loop
    // at line 590). Has its own cache (src/ui-translation-cache.json).
    let translator = Arc::new(JsonTranslator {
        client: client.clone(),
        api_key: api_key.clone(),
        primary_model: (*model).clone(),
    });
    let ui_stats = ui::process_all(translator.clone(), &repo, &locales).await?;
    info!(
        success = ui_stats.success,
        failed = ui_stats.failed,
        skipped = ui_stats.skipped,
        "UI strings phase complete"
    );

    // Phase 3: meta.json. Parallel up to `concurrency`; shares the
    // markdown cache file (translation-cache.json) keyed by
    // `{guideId}/{locale}/meta.json`, so the existing flusher persists
    // updates without extra plumbing.
    let meta_stats = meta_json::process_all(
        translator,
        &repo,
        locales.clone(),
        cache.clone(),
        dirty.clone(),
        concurrency,
    )
    .await?;
    info!(
        success = meta_stats.success,
        failed = meta_stats.failed,
        skipped = meta_stats.skipped,
        "meta.json phase complete"
    );

    // Stop the periodic flusher and do one final write so the cache
    // reflects every successful task (markdown + meta) even if the
    // last flush tick hadn't fired yet. UI writes go through ui.rs's
    // own atomic-rename path, not this flusher.
    flusher_handle.abort();
    let _ = flusher_handle.await; // ignore JoinError(Cancelled)
    if let Err(e) = flush_cache_if_dirty(&cache, &dirty, &cache_path).await {
        warn!(error = %format!("{e:#}"), "final cache flush failed");
    }

    let total_failed = f + ui_stats.failed + meta_stats.failed;
    info!(
        markdown_success = s,
        markdown_failed = f,
        markdown_skipped = k,
        ui_success = ui_stats.success,
        ui_failed = ui_stats.failed,
        meta_success = meta_stats.success,
        meta_failed = meta_stats.failed,
        "trans run complete"
    );
    if total_failed > 0 {
        anyhow::bail!(
            "{total_failed} translation task(s) failed (markdown={f}, ui={ui_f}, meta={meta_f})",
            ui_f = ui_stats.failed,
            meta_f = meta_stats.failed,
        );
    }
    Ok(())
}

enum Outcome {
    Success,
    Skipped,
}

#[allow(clippy::too_many_arguments)]
async fn process_one_task(
    task: &Task,
    guides_dir: &Path,
    cache: &Mutex<CacheMap>,
    dirty: &AtomicBool,
    locales: &Locales,
    client: &reqwest::Client,
    api_key: &str,
    model: &str,
) -> Result<Outcome> {
    let source_path = guides_dir
        .join(&task.guide_id)
        .join("items")
        .join(&locales.default_locale)
        .join(&task.filename);
    let source = tokio::fs::read_to_string(&source_path)
        .await
        .with_context(|| format!("read source {source_path:?}"))?;

    if source.trim().len() < 10 {
        info!(
            "[skipped] {}/{}/{} (too small)",
            task.guide_id, task.locale, task.filename
        );
        record_cache(cache, dirty, task).await;
        return Ok(Outcome::Skipped);
    }

    // en_us special-case: copy source verbatim. Mirrors
    // translate-with-gpt.js:362-366.
    let translation = if task.locale == "en_us" {
        source.clone()
    } else {
        let prompt = build_prompt(&source, &task.locale, locales);
        let system = system_message(&task.locale, locales);
        let raw = call_openai(client, api_key, model, &system, &prompt, &task.filename)
            .await
            .context("OpenAI call")?;
        sanitize_inline_code_attrs(&raw)
    };

    // Validate inline-code count parity. Mirrors translate-with-gpt.js:299-311.
    if task.locale != "en_us" {
        let src_counts = count_inline_code(&source);
        let tr_counts = count_inline_code(&translation);
        if src_counts != tr_counts {
            warn!(
                "[warning] inline-code mismatch in {}/{}/{}: expected {:?}, got {:?}",
                task.guide_id, task.locale, task.filename, src_counts, tr_counts
            );
        }
    }

    // Save translated file.
    let target_dir = guides_dir
        .join(&task.guide_id)
        .join("items")
        .join(&task.locale);
    tokio::fs::create_dir_all(&target_dir).await?;
    let target_path = target_dir.join(&task.filename);
    tokio::fs::write(&target_path, &translation)
        .await
        .with_context(|| format!("write {target_path:?}"))?;

    record_cache(cache, dirty, task).await;
    Ok(Outcome::Success)
}

/// Mark a task as cached in memory. The background flusher
/// (`flush_cache_if_dirty`) will write to disk on its next tick. The
/// final flush at end-of-run guarantees no data loss when the process
/// exits normally; a crash mid-run loses at most one CACHE_FLUSH_INTERVAL
/// worth of completions, which the next run will redo.
async fn record_cache(cache: &Mutex<CacheMap>, dirty: &AtomicBool, task: &Task) {
    let mut c = cache.lock().await;
    c.insert(
        cache_key(&task.guide_id, &task.locale, &task.filename),
        task.source_hash.clone(),
    );
    dirty.store(true, Ordering::Release);
}

/// Snapshot + atomically write the cache, but only if anything is
/// dirty. Atomic via tmp file + rename so a crash mid-write doesn't
/// leave a half-written JSON that fails to parse on the next run.
async fn flush_cache_if_dirty(
    cache: &Mutex<CacheMap>,
    dirty: &AtomicBool,
    cache_path: &Path,
) -> Result<()> {
    // Cheap check first to avoid taking the lock when there's nothing
    // to do (common case on a fully-cached run).
    if !dirty.load(Ordering::Acquire) {
        return Ok(());
    }
    let bytes = {
        let c = cache.lock().await;
        // Clear the flag while we still hold the lock — any writer who
        // sneaks in *after* this point will set it again, so we'll
        // write again on the next tick. Without the lock, a writer
        // could set dirty=true and we'd clear it without persisting
        // their change.
        dirty.store(false, Ordering::Release);
        serde_json::to_vec_pretty(&*c)?
    };
    let tmp = cache_path.with_extension("json.tmp");
    tokio::fs::write(&tmp, &bytes)
        .await
        .with_context(|| format!("write tmp {tmp:?}"))?;
    tokio::fs::rename(&tmp, cache_path)
        .await
        .with_context(|| format!("rename {tmp:?} -> {cache_path:?}"))?;
    Ok(())
}

#[derive(Debug, Clone)]
struct Task {
    guide_id: String,
    locale: String,
    filename: String,
    source_hash: String,
}

fn build_task_list(
    guides_dir: &Path,
    locales: &Locales,
    cache: &CacheMap,
) -> Result<Vec<Task>> {
    let mut tasks = Vec::new();
    for entry in std::fs::read_dir(guides_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let guide_id = entry.file_name().to_string_lossy().into_owned();
        let items_path = entry.path().join("items");
        if !items_path.exists() {
            continue;
        }
        let default_items = items_path.join(&locales.default_locale);
        if !default_items.exists() {
            continue;
        }
        for src_entry in std::fs::read_dir(&default_items)?.flatten() {
            let p = src_entry.path();
            if p.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            let filename = p
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_string();
            let Ok(content) = std::fs::read_to_string(&p) else {
                continue;
            };
            let source_hash = hash_content(&content);

            for (locale, _) in &locales.locales {
                if locale == &locales.default_locale {
                    continue;
                }
                let target = items_path.join(locale).join(&filename);
                let cache_key = cache_key(&guide_id, locale, &filename);
                let cached_hash = cache.get(&cache_key);
                let needs = !target.exists() || cached_hash != Some(&source_hash);
                if needs {
                    tasks.push(Task {
                        guide_id: guide_id.clone(),
                        locale: locale.clone(),
                        filename: filename.clone(),
                        source_hash: source_hash.clone(),
                    });
                }
            }
        }
    }
    Ok(tasks)
}

fn cache_key(guide_id: &str, locale: &str, filename: &str) -> String {
    format!("{guide_id}/{locale}/{filename}")
}

/// Verbatim port of `getSystemMessage` at translate-with-gpt.js:139-148.
fn system_message(locale: &str, locales: &Locales) -> String {
    let native = locales
        .locales
        .get(locale)
        .map(|l| l.native_name.clone())
        .unwrap_or_else(|| locale.to_string());
    format!(
        "You are an expert technical translator specializing in software documentation.\n\
         You translate from English to {native} ({locale}).\n\
         You maintain the exact same formatting, structure, and technical accuracy.\n\
         You NEVER translate code, variable names, API endpoints, or technical identifiers.\n\
         You preserve all markdown formatting and special tags exactly as they appear."
    )
}

/// Verbatim port of `buildPrompt` at translate-with-gpt.js:156-188.
fn build_prompt(content: &str, locale: &str, locales: &Locales) -> String {
    let native = locales
        .locales
        .get(locale)
        .map(|l| l.native_name.clone())
        .unwrap_or_else(|| locale.to_string());
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "Translate the following FastComments documentation from English to {native}."
    ));
    lines.push(String::new());
    lines.push("CRITICAL RULES:".to_string());
    lines.push("1. Retain code and logic in [inline-code-start] and [inline-code-end] blocks exactly, just translate comments.".to_string());
    lines.push("2. DO NOT translate anything inside [inline-code-attrs-start ...] tags - preserve them exactly".to_string());
    lines.push("3. DO NOT translate [api-resource-header-start ...] tags - preserve them exactly".to_string());
    lines.push("4. DO NOT translate code blocks (```...```) or inline code (`...`) except comments.".to_string());
    lines.push("5. DO NOT translate URLs, API endpoints, variable names, or technical identifiers".to_string());
    lines.push("6. DO NOT translate property names in TypeScript/JavaScript interfaces".to_string());
    lines.push("7. PRESERVE all special tags and their attributes exactly as written".to_string());
    lines.push("8. PRESERVE all markdown formatting (headers, lists, bold, links, etc.)".to_string());
    lines.push("9. Translate ONLY the natural language text (descriptions, explanations)".to_string());
    lines.push("10. Keep the same line structure and paragraph breaks".to_string());
    lines.push(String::new());
    lines.push("The title attributes in [inline-code-attrs-start] tags SHOULD be translated.".to_string());
    lines.push("For example: title = 'Example cURL Request' should become title = 'Exemple de requête cURL' in French.".to_string());
    lines.push("If a translated title contains an apostrophe inside the single-quoted value, escape it with a backslash. For example, French d'utilisation must be written as title = 'Exemple d\\'utilisation' (the attrs body is parsed as JavaScript and an unescaped apostrophe will break the build).".to_string());
    lines.push(String::new());
    lines.push("SOURCE CONTENT:".to_string());
    lines.push("---".to_string());
    lines.push(content.to_string());
    lines.push("---".to_string());
    lines.push(String::new());
    lines.push("Return ONLY the translated content, nothing else. No explanations or notes.".to_string());
    lines.join("\n")
}

/// Verbatim port of `sanitizeInlineCodeAttrs` at translate-with-gpt.js:45-62.
fn sanitize_inline_code_attrs(text: &str) -> String {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?s)\[inline-code-attrs-start ([\s\S]*?) inline-code-attrs-end\]")
            .expect("regex")
    });
    static TOK_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?s)^(\s*\w+\s*=\s*)'([\s\S]*)'(\s*)$").expect("regex")
    });
    RE.replace_all(text, |caps: &regex::Captures| {
        let body = &caps[1];
        let tokens: Vec<&str> = body.split("; ").collect();
        let fixed: Vec<String> = tokens
            .iter()
            .map(|tok| {
                if let Some(m) = TOK_RE.captures(tok) {
                    let prefix = m[1].to_string();
                    let value = m[2].to_string();
                    let suffix = m[3].to_string();
                    // Replace unescaped `'` with `\'`, leaving already-escaped
                    // `\'` untouched. Use NUL as a placeholder during the swap.
                    let escaped = value
                        .replace("\\'", "\u{0000}")
                        .replace('\'', "\\'")
                        .replace('\u{0000}', "\\'");
                    format!("{prefix}'{escaped}'{suffix}")
                } else {
                    tok.to_string()
                }
            })
            .collect();
        format!(
            "[inline-code-attrs-start {} inline-code-attrs-end]",
            fixed.join("; ")
        )
    })
    .into_owned()
}

fn count_inline_code(content: &str) -> (usize, usize) {
    (
        content.matches("[inline-code-start]").count(),
        content.matches("[inline-code-end]").count(),
    )
}

async fn call_openai(
    client: &reqwest::Client,
    api_key: &str,
    model: &str,
    system: &str,
    prompt: &str,
    filename: &str,
) -> Result<String> {
    let body = serde_json::json!({
        "model": model,
        "messages": [
            {"role": "system", "content": system},
            {"role": "user", "content": prompt},
        ],
        "max_completion_tokens": 16000,
    });
    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .with_context(|| format!("POST OpenAI for {filename}"))?;
    let status = resp.status();
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("OpenAI API error {status}: {text}");
    }
    let value: serde_json::Value = resp.json().await?;
    let text = value
        .pointer("/choices/0/message/content")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .trim()
        .to_string();
    if text.is_empty() {
        anyhow::bail!("OpenAI returned empty content");
    }
    Ok(text)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_replaces_unescaped_apostrophes() {
        let input = "[inline-code-attrs-start title = 'Exemple d'utilisation'; type = 'html' inline-code-attrs-end]";
        let out = sanitize_inline_code_attrs(input);
        assert_eq!(
            out,
            "[inline-code-attrs-start title = 'Exemple d\\'utilisation'; type = 'html' inline-code-attrs-end]"
        );
    }

    #[test]
    fn sanitize_preserves_already_escaped() {
        let input = "[inline-code-attrs-start title = 'Already \\'escaped\\'' inline-code-attrs-end]";
        let out = sanitize_inline_code_attrs(input);
        assert_eq!(
            out,
            "[inline-code-attrs-start title = 'Already \\'escaped\\'' inline-code-attrs-end]"
        );
    }

    #[test]
    fn system_message_format() {
        let locales = Locales {
            default_locale: "en".into(),
            locales: indexmap::IndexMap::from([(
                "fr_fr".to_string(),
                fcdocs_shared::locales::Locale {
                    name: "French (France)".into(),
                    native_name: "Français (France)".into(),
                    hreflang: "fr-FR".into(),
                    flag: Some("🇫🇷".into()),
                },
            )]),
        };
        let sm = system_message("fr_fr", &locales);
        assert!(sm.contains("Français (France) (fr_fr)"));
    }

    #[tokio::test]
    async fn record_cache_marks_dirty_without_writing() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let cache_path = tmp.path().to_path_buf();
        let cache = Mutex::new(CacheMap::default());
        let dirty = AtomicBool::new(false);
        let task = Task {
            guide_id: "g".into(),
            locale: "fr_fr".into(),
            filename: "f.md".into(),
            source_hash: "abc".into(),
        };
        record_cache(&cache, &dirty, &task).await;
        assert!(dirty.load(Ordering::Acquire));
        // The file on disk hasn't been touched yet — record_cache only
        // marks dirty.
        let on_disk = tokio::fs::read_to_string(&cache_path).await.unwrap();
        assert_eq!(on_disk, "");
    }

    #[tokio::test]
    async fn flush_writes_and_clears_dirty() {
        let dir = tempfile::tempdir().unwrap();
        let cache_path = dir.path().join("cache.json");
        let cache = Mutex::new(CacheMap::default());
        let dirty = AtomicBool::new(false);
        let task = Task {
            guide_id: "g".into(),
            locale: "fr_fr".into(),
            filename: "f.md".into(),
            source_hash: "abc".into(),
        };
        record_cache(&cache, &dirty, &task).await;

        flush_cache_if_dirty(&cache, &dirty, &cache_path).await.unwrap();
        assert!(!dirty.load(Ordering::Acquire));
        let on_disk: serde_json::Value =
            serde_json::from_slice(&tokio::fs::read(&cache_path).await.unwrap()).unwrap();
        assert_eq!(on_disk["g/fr_fr/f.md"], "abc");
    }

    #[tokio::test]
    async fn flush_is_noop_when_clean() {
        let dir = tempfile::tempdir().unwrap();
        let cache_path = dir.path().join("cache.json");
        let cache = Mutex::new(CacheMap::default());
        let dirty = AtomicBool::new(false);
        // No record_cache call → still clean. Flush must not create the
        // file, so a fresh `trans run` with no work doesn't truncate an
        // existing on-disk cache to `{}`.
        flush_cache_if_dirty(&cache, &dirty, &cache_path).await.unwrap();
        assert!(!cache_path.exists());
    }

    #[tokio::test]
    async fn many_records_one_flush() {
        // The whole point of the refactor: 100 records + 1 flush = 1
        // file write, not 100.
        let dir = tempfile::tempdir().unwrap();
        let cache_path = dir.path().join("cache.json");
        let cache = Mutex::new(CacheMap::default());
        let dirty = AtomicBool::new(false);
        for i in 0..100 {
            let task = Task {
                guide_id: format!("g{i}"),
                locale: "fr_fr".into(),
                filename: format!("f{i}.md"),
                source_hash: format!("h{i}"),
            };
            record_cache(&cache, &dirty, &task).await;
        }
        flush_cache_if_dirty(&cache, &dirty, &cache_path).await.unwrap();
        let on_disk: serde_json::Value =
            serde_json::from_slice(&tokio::fs::read(&cache_path).await.unwrap()).unwrap();
        assert_eq!(on_disk.as_object().unwrap().len(), 100);
    }
}

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

use crate::discover::default_locale_files;
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

/// CLI options for `trans run`. Mirror of Node parseArgs() at
/// src/translate-with-gpt.js:996-1024.
#[derive(Debug, Default, Clone)]
pub struct Options {
    /// `--locale <code>`: limit to one locale across all three phases.
    pub locale: Option<String>,
    /// `--guide <id>`: limit to one guide for markdown items + meta.json.
    /// UI strings aren't per-guide so this filter has no effect there.
    pub guide: Option<String>,
    /// `--concurrency <n>`: caps both markdown and meta.json fan-out.
    /// UI strings remain sequential per-locale (matches Node).
    pub concurrency: Option<usize>,
    /// `--dry-run`: skip OpenAI calls + skip writes; just log what
    /// would happen. Useful for verifying scope before paying tokens.
    pub dry_run: bool,
    /// `--force`: re-translate even when the cache says fresh.
    /// Useful when the prompt template or system message changed
    /// and the existing translations no longer reflect intent.
    pub force: bool,
}

/// Parse CLI args after the `run` subcommand. Mirrors Node parseArgs()
/// at src/translate-with-gpt.js:996-1024 byte-for-byte: same flag
/// names, same `--help` text shape, same defaults.
pub fn parse_options<I: IntoIterator<Item = String>>(args: I) -> Result<Options> {
    let mut opts = Options::default();
    let mut iter = args.into_iter().peekable();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--locale" => {
                opts.locale = Some(
                    iter.next()
                        .context("--locale requires a value (e.g. --locale fr_fr)")?,
                );
            }
            "--guide" => {
                opts.guide = Some(
                    iter.next()
                        .context("--guide requires a value (e.g. --guide api)")?,
                );
            }
            "--concurrency" => {
                let v = iter
                    .next()
                    .context("--concurrency requires a value (e.g. --concurrency 20)")?;
                let n: usize = v
                    .parse()
                    .with_context(|| format!("--concurrency must be a positive integer, got {v:?}"))?;
                if n == 0 {
                    anyhow::bail!("--concurrency must be > 0");
                }
                opts.concurrency = Some(n);
            }
            "--dry-run" => opts.dry_run = true,
            "--force" => opts.force = true,
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => anyhow::bail!("unknown arg: {other:?} (try --help)"),
        }
    }
    Ok(opts)
}

fn print_help() {
    // Indent column 25 (after "  --concurrency <n>  ") for clean
    // continuation alignment. Written as discrete lines instead of one
    // long multi-line string so the column doesn't drift under refactor.
    let lines: &[&str] = &[
        "Usage: trans run [options]",
        "",
        "Options:",
        "  --locale <code>        Only translate for this locale (e.g. fr_fr).",
        "  --guide <id>           Only translate for this guide. Applies to markdown",
        "                         items + meta.json; UI strings are global, so the",
        "                         filter has no effect there.",
        "  --concurrency <n>      Concurrent OpenAI calls. Also overridable via the",
        "                         TRANS_CONCURRENCY env var.",
        "  --dry-run              Report what would translate; do NOT call OpenAI or",
        "                         write files. Skips the OPENAI_API_KEY requirement",
        "                         so it's safe for scope-check runs in CI.",
        "  --force                Re-translate even when the cache says fresh (e.g.",
        "                         after a prompt or system-message edit).",
        "  --help, -h             Show this message.",
        "",
        "Environment:",
        "  OPENAI_API_KEY         Required unless --dry-run.",
        "  OPENAI_MODEL           Model to use.",
        "  TRANS_CONCURRENCY      Default concurrency (see --concurrency).",
    ];
    for l in lines {
        println!("{l}");
    }
    println!();
    println!("Defaults: concurrency = {DEFAULT_CONCURRENCY}, model = {DEFAULT_MODEL}");
}

pub async fn run_with(opts: Options) -> Result<()> {
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

    // Validate --locale early so a typo doesn't silently translate zero
    // files (which would look like success).
    if let Some(loc) = &opts.locale {
        if !locales.locales.contains_key(loc) {
            anyhow::bail!(
                "--locale {loc:?} is not a known locale. \
                 Known: {known}",
                known = locales
                    .locales
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if loc == &locales.default_locale {
            anyhow::bail!(
                "--locale {loc:?} is the default locale; nothing to translate."
            );
        }
    }

    let tasks = build_task_list_filtered(
        &guides_dir,
        &locales,
        &cache_initial,
        opts.locale.as_deref(),
        opts.guide.as_deref(),
        opts.force,
    )?;
    info!(
        count = tasks.len(),
        force = opts.force,
        dry_run = opts.dry_run,
        locale = opts.locale.as_deref().unwrap_or("*"),
        guide = opts.guide.as_deref().unwrap_or("*"),
        "markdown translation tasks"
    );

    // OPENAI_API_KEY only required when we'll actually call out. Dry
    // runs (Node line 891: `if (dryRun) { skip }`) need to work
    // unattended in CI even without the secret.
    let api_key = std::env::var("OPENAI_API_KEY").ok();
    if api_key.is_none() && !opts.dry_run {
        warn!("OPENAI_API_KEY not set — `trans run` cannot call OpenAI (use --dry-run to scope-check without a key).");
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
    // Empty string is fine for dry-run — the markdown path's
    // process_one_task short-circuits before consulting it.
    let api_key = Arc::new(api_key.unwrap_or_default());
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

    // Priority: --concurrency flag > TRANS_CONCURRENCY env > DEFAULT.
    let concurrency: usize = opts
        .concurrency
        .or_else(|| {
            std::env::var("TRANS_CONCURRENCY")
                .ok()
                .and_then(|s| s.parse().ok())
                .filter(|n: &usize| *n > 0)
        })
        .unwrap_or(DEFAULT_CONCURRENCY);
    info!(concurrency, "translation concurrency");

    let success = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let failed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let skipped = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let mut in_flight = FuturesUnordered::new();
    let mut iter = tasks.into_iter();

    let dry_run = opts.dry_run;
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
                dry_run,
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
    let ui_stats = ui::process_all(
        translator.clone(),
        &repo,
        &locales,
        ui::Options {
            filter_locale: opts.locale.clone(),
            dry_run: opts.dry_run,
            force: opts.force,
        },
    )
    .await?;
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
        meta_json::Options {
            filter_locale: opts.locale.clone(),
            filter_guide: opts.guide.clone(),
            dry_run: opts.dry_run,
            force: opts.force,
        },
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
    dry_run: bool,
) -> Result<Outcome> {
    if dry_run {
        // Matches Node's processTranslations dry-run branch (it logs
        // "Would translate ..." and increments skipped). We log + skip
        // without touching the cache so a subsequent real run still
        // sees the same gap.
        info!(
            "[dry-run] would translate {}/{}/{}",
            task.guide_id, task.locale, task.filename
        );
        return Ok(Outcome::Skipped);
    }
    // task.source_path was recorded at discovery time. For
    // root-level intro.md / conclusion.md it points at the guide
    // root, not items/<default_locale>/. Re-constructing the path
    // here from (guide_id, filename) would silently re-introduce the
    // "intro.md missing from items/<default_locale>/" bug — read
    // would fail and the task would be marked failed even though
    // discovery saw the file at the root.
    let _ = guides_dir; // retained in signature for symmetry
    let _ = locales;    // ditto
    let source = tokio::fs::read_to_string(&task.source_path)
        .await
        .with_context(|| format!("read source {:?}", task.source_path))?;

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
    /// Absolute path of the source-locale file. Usually
    /// `<guide>/items/<default_locale>/<filename>`, but for root-level
    /// intro.md / conclusion.md (per Node check-translations.js:147-158)
    /// it points at the guide root. The translated output always
    /// lands under `items/<locale>/<filename>` regardless.
    source_path: PathBuf,
}

/// Filtered task discovery. Mirrors translate-with-gpt.js::buildTaskList
/// (line 950-994):
///   - `filter_locale = Some(loc)` restricts to that locale.
///   - `filter_guide = Some(id)` restricts to that guide.
///   - `force = true` includes every (guide, locale, file), ignoring
///     the cache freshness check. The file-exists check is also
///     bypassed so a real `--force` regenerates even files already on
///     disk with the correct content.
fn build_task_list_filtered(
    guides_dir: &Path,
    locales: &Locales,
    cache: &CacheMap,
    filter_locale: Option<&str>,
    filter_guide: Option<&str>,
    force: bool,
) -> Result<Vec<Task>> {
    let mut tasks = Vec::new();
    for entry in std::fs::read_dir(guides_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let guide_id = entry.file_name().to_string_lossy().into_owned();
        if let Some(filter) = filter_guide {
            if guide_id != filter {
                continue;
            }
        }
        let guide_path = entry.path();
        let items_path = guide_path.join("items");
        // discover::default_locale_files handles the root-level
        // intro.md / conclusion.md fallback that Node's
        // getDefaultLocaleFiles applies (check-translations.js:141-161).
        // Without that, ~157 root-level intro/conclusion files were
        // silently excluded from every translation run.
        let default_files = default_locale_files(&guide_path, &locales.default_locale);
        if default_files.is_empty() {
            continue;
        }
        for src in default_files {
            let Ok(content) = std::fs::read_to_string(&src.source_path) else {
                continue;
            };
            let source_hash = hash_content(&content);

            for (locale, _) in &locales.locales {
                if locale == &locales.default_locale {
                    continue;
                }
                if let Some(filter) = filter_locale {
                    if locale != filter {
                        continue;
                    }
                }
                let target = items_path.join(locale).join(&src.filename);
                let cache_key = cache_key(&guide_id, locale, &src.filename);
                let cached_hash = cache.get(&cache_key);
                let needs = force
                    || !target.exists()
                    || cached_hash != Some(&source_hash);
                if needs {
                    tasks.push(Task {
                        guide_id: guide_id.clone(),
                        locale: locale.clone(),
                        filename: src.filename.clone(),
                        source_hash: source_hash.clone(),
                        source_path: src.source_path.clone(),
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
            source_path: PathBuf::from("/tmp/unused"),
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
            source_path: PathBuf::from("/tmp/unused"),
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
                source_path: PathBuf::from("/tmp/unused"),
            };
            record_cache(&cache, &dirty, &task).await;
        }
        flush_cache_if_dirty(&cache, &dirty, &cache_path).await.unwrap();
        let on_disk: serde_json::Value =
            serde_json::from_slice(&tokio::fs::read(&cache_path).await.unwrap()).unwrap();
        assert_eq!(on_disk.as_object().unwrap().len(), 100);
    }
}

#[cfg(test)]
mod cli_tests {
    //! Argument parsing + filter wiring for `trans run`. The Node CLI
    //! contract at src/translate-with-gpt.js:996-1024 is the spec.

    use super::*;
    use fcdocs_shared::locales::Locale;
    use indexmap::IndexMap;

    fn locales_en_fr_de() -> Locales {
        let mut m: IndexMap<String, Locale> = IndexMap::new();
        for k in ["en", "fr_fr", "de_de"] {
            m.insert(
                k.into(),
                Locale {
                    name: k.into(),
                    native_name: k.into(),
                    hreflang: k.into(),
                    flag: None,
                },
            );
        }
        Locales {
            default_locale: "en".into(),
            locales: m,
        }
    }

    fn args(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn defaults_when_no_flags() {
        let o = parse_options(args(&[])).unwrap();
        assert_eq!(o.locale, None);
        assert_eq!(o.guide, None);
        assert_eq!(o.concurrency, None);
        assert!(!o.dry_run);
        assert!(!o.force);
    }

    #[test]
    fn all_flags_parse() {
        let o = parse_options(args(&[
            "--locale",
            "fr_fr",
            "--guide",
            "api",
            "--concurrency",
            "8",
            "--dry-run",
            "--force",
        ]))
        .unwrap();
        assert_eq!(o.locale.as_deref(), Some("fr_fr"));
        assert_eq!(o.guide.as_deref(), Some("api"));
        assert_eq!(o.concurrency, Some(8));
        assert!(o.dry_run);
        assert!(o.force);
    }

    #[test]
    fn locale_requires_value() {
        let e = parse_options(args(&["--locale"])).unwrap_err();
        assert!(e.to_string().contains("--locale requires a value"));
    }

    #[test]
    fn concurrency_must_be_positive_integer() {
        assert!(parse_options(args(&["--concurrency", "0"])).is_err());
        assert!(parse_options(args(&["--concurrency", "abc"])).is_err());
    }

    #[test]
    fn unknown_arg_rejected() {
        let e = parse_options(args(&["--frobnicate"])).unwrap_err();
        assert!(e.to_string().contains("unknown arg"));
    }

    // --- filter behavior on build_task_list_filtered ---

    fn write_md(root: &Path, guide: &str, locale: &str, name: &str, body: &str) {
        let dir = root.join(guide).join("items").join(locale);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(name), body).unwrap();
    }

    #[test]
    fn build_task_list_locale_filter() {
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        // Source files exist in en/; no fr_fr or de_de translations.
        write_md(g, "a", "en", "x.md", "Source content for X file translation.");
        write_md(g, "a", "en", "y.md", "Source content for Y file translation.");
        let cache = CacheMap::new();
        let all = build_task_list_filtered(g, &locales_en_fr_de(), &cache, None, None, false)
            .unwrap();
        // 2 files * 2 non-default locales = 4 tasks
        assert_eq!(all.len(), 4);
        let only_fr = build_task_list_filtered(
            g,
            &locales_en_fr_de(),
            &cache,
            Some("fr_fr"),
            None,
            false,
        )
        .unwrap();
        assert_eq!(only_fr.len(), 2);
        assert!(only_fr.iter().all(|t| t.locale == "fr_fr"));
    }

    #[test]
    fn build_task_list_guide_filter() {
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write_md(g, "guide-a", "en", "x.md", "Source content for X file translation.");
        write_md(g, "guide-b", "en", "y.md", "Source content for Y file translation.");
        let cache = CacheMap::new();
        let only_a = build_task_list_filtered(
            g,
            &locales_en_fr_de(),
            &cache,
            None,
            Some("guide-a"),
            false,
        )
        .unwrap();
        assert_eq!(only_a.len(), 2); // x.md × 2 non-default locales
        assert!(only_a.iter().all(|t| t.guide_id == "guide-a"));
    }

    #[test]
    fn build_task_list_picks_up_root_level_intro_and_conclusion() {
        // Regression: ~157 root-level intro.md/conclusion.md files
        // across ~80 guides were silently excluded from the previous
        // walk because it only scanned items/<default>/. Discovery
        // must surface them so they hit the cache + translation path.
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write_md(g, "g", "en", "howto.md", "Body content for howto.");
        // intro.md at guide ROOT (mirrors src/content/guides/<id>/intro.md).
        std::fs::write(g.join("g").join("intro.md"), "Welcome to the guide.").unwrap();
        std::fs::write(g.join("g").join("conclusion.md"), "Thanks for reading.").unwrap();

        let cache = CacheMap::new();
        let tasks =
            build_task_list_filtered(g, &locales_en_fr_de(), &cache, None, None, false).unwrap();
        // 3 source files (howto + intro + conclusion) × 2 non-default locales = 6 tasks
        assert_eq!(tasks.len(), 6, "tasks: {tasks:#?}");
        let names: std::collections::HashSet<&str> =
            tasks.iter().map(|t| t.filename.as_str()).collect();
        assert!(names.contains("intro.md"), "intro.md missing from tasks");
        assert!(names.contains("conclusion.md"));

        // For the root-level files, source_path MUST point at the
        // guide root (not items/en/), otherwise process_one_task can't
        // read the source.
        let intro = tasks.iter().find(|t| t.filename == "intro.md").unwrap();
        assert_eq!(intro.source_path, g.join("g").join("intro.md"));
        let conclusion = tasks.iter().find(|t| t.filename == "conclusion.md").unwrap();
        assert_eq!(conclusion.source_path, g.join("g").join("conclusion.md"));
        // And for the regular item, source_path stays under items/en/.
        let howto = tasks.iter().find(|t| t.filename == "howto.md").unwrap();
        assert_eq!(howto.source_path, g.join("g/items/en/howto.md"));
    }

    #[test]
    fn build_task_list_force_ignores_cache() {
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write_md(g, "g", "en", "x.md", "Source content for X file translation.");
        // Translation already exists for fr_fr AND the cache says it's
        // fresh — no work needed in normal mode.
        write_md(g, "g", "fr_fr", "x.md", "Contenu source pour la traduction du fichier X.");
        let hash = hash_content("Source content for X file translation.");
        let mut cache = CacheMap::new();
        cache.insert("g/fr_fr/x.md".into(), hash.clone());
        cache.insert("g/de_de/x.md".into(), hash.clone()); // de_de also fresh
        write_md(g, "g", "de_de", "x.md", "Quelle für die Übersetzung der X-Datei.");

        let normal = build_task_list_filtered(
            g,
            &locales_en_fr_de(),
            &cache,
            None,
            None,
            false,
        )
        .unwrap();
        assert_eq!(normal.len(), 0, "everything is fresh");

        let forced = build_task_list_filtered(
            g,
            &locales_en_fr_de(),
            &cache,
            None,
            None,
            true,
        )
        .unwrap();
        assert_eq!(forced.len(), 2, "--force re-includes every (file × locale)");
    }
}

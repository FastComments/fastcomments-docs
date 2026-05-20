//! Replaces `src/build-search-index-parallel.js` +
//! `src/build-search-index-worker.js`.
//!
//! Per-locale, walks `src/content/guides/`, applies the content pipeline
//! (handlebars -> markers -> markdown -> snippets -> html-to-text), and writes
//! a Tantivy index to `index/<locale>/`.
//!
//! Behavior reference: the field shapes and special-case branches mirror
//! `src/build-search-index-worker.js` line-for-line so a regression harness
//! can diff text content one-to-one.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use anyhow::{Context, Result};
use fcdocs_shared::guides::{Guide, GuidesRoot, MetaItem};
use fcdocs_shared::locales::Locales;
use fcdocs_shared::pipeline;
use fcdocs_shared::sidecar::SidecarClient;
use tantivy::schema::{
    IndexRecordOption, Schema, SchemaBuilder, TextFieldIndexing, TextOptions, STORED, STRING,
};
use tantivy::tokenizer::{
    Language, LowerCaser, RemoveLongFilter, SimpleTokenizer, Stemmer, TextAnalyzer,
};
use tantivy::{Index, TantivyDocument};
use tokio::sync::Semaphore;
use tracing::{info, warn};

use fcdocs_shared::sidecar_supervisor::Sidecar;

const DEFAULT_INDEX_DIR: &str = "index";
const DEFAULT_GUIDES_DIR: &str = "src/content/guides";
const DEFAULT_SNIPPETS_DIR: &str = "src/snippets";
const DEFAULT_LOCALES_JSON: &str = "src/locales.json";
const SIDECAR_SCRIPT: &str = "src/content-sidecar.js";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let repo_root = repo_root()?;
    let guides_dir = repo_root.join(DEFAULT_GUIDES_DIR);
    let snippets_dir = repo_root.join(DEFAULT_SNIPPETS_DIR);
    let locales_path = repo_root.join(DEFAULT_LOCALES_JSON);
    let out_dir = repo_root.join(DEFAULT_INDEX_DIR);

    if !guides_dir.exists() {
        anyhow::bail!("guides dir missing: {:?}", guides_dir);
    }
    if !locales_path.exists() {
        anyhow::bail!(
            "locales.json missing: {:?}. Run `node scripts/dump-locales.js` first.",
            locales_path
        );
    }
    std::fs::create_dir_all(&out_dir)?;

    let locales = Locales::load_from(&locales_path)?;
    let default_locale = locales.default_locale.clone();
    // Optional comma-separated locale filter (`--locale en` or
    // `INDEXER_LOCALES=en,fr_fr`) for focused dev/test runs.
    let locale_filter: Option<Vec<String>> = parse_locale_filter(std::env::args().skip(1))
        .or_else(|| {
            std::env::var("INDEXER_LOCALES").ok().map(|v| {
                v.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
        });
    let locale_keys: Vec<String> = locales
        .keys()
        .filter(|k| locale_filter.as_ref().map_or(true, |f| f.iter().any(|x| x == k)))
        .map(|s| s.to_string())
        .collect();
    if locale_keys.is_empty() {
        anyhow::bail!("locale filter matched zero locales");
    }

    info!(
        locales = locale_keys.len(),
        out_dir = %out_dir.display(),
        "starting indexer"
    );

    // Spawn the Node sidecar and wait for it to be ready.
    let sidecar_proc = Sidecar::spawn(&repo_root, &repo_root.join(SIDECAR_SCRIPT)).await?;
    info!(url = %sidecar_proc.url, "content sidecar up");
    let sidecar = Arc::new(SidecarClient::new(sidecar_proc.url.clone()));

    let root = Arc::new(GuidesRoot::new(guides_dir, default_locale));
    let snippets_dir = Arc::new(snippets_dir);
    let out_dir = Arc::new(out_dir);

    let parallelism = ((num_cpus::get() as f64) * 1.5).ceil() as usize;
    let parallelism = parallelism.max(1);
    info!(parallelism, "spawning per-locale workers");

    let permits = Arc::new(Semaphore::new(parallelism));
    let mut handles = Vec::with_capacity(locale_keys.len());
    for locale in locale_keys.clone() {
        let root = Arc::clone(&root);
        let snippets_dir = Arc::clone(&snippets_dir);
        let out_dir = Arc::clone(&out_dir);
        let sidecar = Arc::clone(&sidecar);
        let permits = Arc::clone(&permits);
        handles.push(tokio::spawn(async move {
            let _permit = permits.acquire().await.expect("semaphore alive");
            let started = Instant::now();
            let result = build_locale_index(
                &locale,
                &root,
                &snippets_dir,
                &out_dir,
                &sidecar,
            )
            .await;
            (locale, result, started.elapsed())
        }));
    }

    let total = handles.len();
    let mut total_indexed: u64 = 0;
    let mut total_skipped: u64 = 0;
    let mut failed = 0usize;
    let mut completed = 0usize;
    for h in handles {
        let (locale, result, elapsed) = h.await?;
        completed += 1;
        match result {
            Ok(stats) => {
                total_indexed += stats.indexed;
                total_skipped += stats.skipped;
                let size_mb = stats.size_bytes as f64 / 1024.0 / 1024.0;
                info!(
                    "[{}/{}] {}: {} indexed, {} skipped, {:.2} MB, {:.2?}",
                    completed, total, locale, stats.indexed, stats.skipped, size_mb, elapsed
                );
            }
            Err(e) => {
                failed += 1;
                warn!(
                    "[{}/{}] {}: FAILED — {:#}",
                    completed, total, locale, e
                );
            }
        }
    }

    info!(
        total_indexed,
        total_skipped,
        failed,
        "indexer done"
    );

    // Shut down the sidecar.
    drop(sidecar);
    sidecar_proc.shutdown().await;

    if failed > 0 {
        anyhow::bail!("{} locale(s) failed", failed);
    }
    Ok(())
}

#[derive(Debug, Default, Clone, Copy)]
struct LocaleStats {
    indexed: u64,
    skipped: u64,
    size_bytes: u64,
}

async fn build_locale_index(
    locale: &str,
    root: &GuidesRoot,
    snippets_dir: &Path,
    out_dir: &Path,
    sidecar: &SidecarClient,
) -> Result<LocaleStats> {
    let dir = out_dir.join(locale);
    if dir.exists() {
        std::fs::remove_dir_all(&dir)
            .with_context(|| format!("wipe stale index dir {dir:?}"))?;
    }
    std::fs::create_dir_all(&dir)?;

    let schema = build_schema();
    let index = Index::create_in_dir(&dir, schema.clone())?;
    register_tokenizers(&index, locale);
    let mut writer = index.writer(50_000_000)?;

    let f_doc_id = schema.get_field("doc_id").unwrap();
    let f_title = schema.get_field("title").unwrap();
    let f_parent_title = schema.get_field("parent_title").unwrap();
    let f_url = schema.get_field("url").unwrap();
    let f_parent_url = schema.get_field("parent_url").unwrap();
    let f_icon = schema.get_field("icon").unwrap();
    let f_search_text = schema.get_field("search_text").unwrap();

    let mut stats = LocaleStats::default();
    let guides = root.walk(locale)?;

    for guide in &guides {
        let guide_title = guide
            .meta
            .page_header
            .clone()
            .or_else(|| guide.meta.name.clone())
            .unwrap_or_else(|| guide.id.clone());

        // Skip non-default locales that have no translated content for this guide.
        if locale != root.default_locale && !root.has_locale_content(&guide.id, locale) {
            continue;
        }

        let guide_link = root.guide_link(&guide.id, locale);
        let icon = guide
            .meta
            .icon
            .as_deref()
            .map(|i| format!("/images/guide-icons/{i}"));

        if guide.id.starts_with("installation-") && guide.id != "installation" {
            match build_installation_aggregate(guide, locale, root, snippets_dir, sidecar).await {
                Ok(text) => {
                    let mut d = TantivyDocument::default();
                    d.add_text(f_doc_id, &guide.id);
                    d.add_text(f_title, guide_title.as_str());
                    d.add_text(f_url, &format!("/{guide_link}"));
                    if let Some(icon) = &icon {
                        d.add_text(f_icon, icon);
                    }
                    d.add_text(f_search_text, &text);
                    // parent_title/parent_url omitted for top-level docs.
                    writer.add_document(d)?;
                    stats.indexed += 1;
                }
                Err(e) => {
                    warn!(guide = %guide.id, locale, error = %e, "installation aggregate failed");
                    stats.skipped += 1;
                }
            }
            continue;
        }

        // Standard per-item indexing.
        for item in &guide.meta.items_ordered {
            match build_item(guide, item, locale, root, snippets_dir, sidecar).await {
                Ok(Some((item_id, full_url, text))) => {
                    let mut d = TantivyDocument::default();
                    d.add_text(f_doc_id, &format!("{}>{item_id}", guide.id));
                    d.add_text(f_title, &item.name);
                    d.add_text(f_parent_title, &guide_title);
                    d.add_text(f_url, &full_url);
                    d.add_text(f_parent_url, &guide_link);
                    if let Some(icon) = &icon {
                        d.add_text(f_icon, icon);
                    }
                    d.add_text(f_search_text, &text);
                    writer.add_document(d)?;
                    stats.indexed += 1;
                }
                Ok(None) => {
                    stats.skipped += 1;
                }
                Err(e) => {
                    warn!(
                        guide = %guide.id,
                        item = %item.file,
                        locale,
                        error = %e,
                        "item indexing failed"
                    );
                    stats.skipped += 1;
                }
            }
        }
    }

    writer.commit()?;
    drop(writer);

    stats.size_bytes = dir_size_bytes(&dir).unwrap_or(0);
    Ok(stats)
}

/// Build the `installation-*` aggregate: intro.md + every item concatenated.
/// Mirrors `src/build-search-index-worker.js:96-132`.
async fn build_installation_aggregate(
    guide: &Guide,
    locale: &str,
    root: &GuidesRoot,
    snippets_dir: &Path,
    sidecar: &SidecarClient,
) -> Result<String> {
    let mut combined = String::new();

    if let Some(intro_path) = root.intro_path(&guide.id, locale) {
        let raw = std::fs::read_to_string(&intro_path)
            .with_context(|| format!("read intro {intro_path:?}"))?;
        let processed =
            pipeline::process_markdown(&intro_path, &raw, snippets_dir, sidecar).await?;
        combined.push_str(&processed.indexable_text);
    }

    for item in &guide.meta.items_ordered {
        match build_item(guide, item, locale, root, snippets_dir, sidecar).await {
            Ok(Some((_id, _url, text))) => {
                if !combined.is_empty() {
                    combined.push('\n');
                }
                combined.push_str(&text);
            }
            Ok(None) => {}
            Err(e) => {
                warn!(
                    guide = %guide.id,
                    item = %item.file,
                    locale,
                    error = %e,
                    "item indexing failed in aggregate"
                );
            }
        }
    }

    Ok(combined)
}

/// Returns `(item_id, full_url, indexable_text)` or `None` for missing
/// `-generated.md` files (mirrors src/guides.js:127-130).
async fn build_item(
    guide: &Guide,
    item: &MetaItem,
    locale: &str,
    root: &GuidesRoot,
    snippets_dir: &Path,
    sidecar: &SidecarClient,
) -> Result<Option<(String, String, String)>> {
    let (path, _fallback) = root.resolve_item_path(&guide.id, &item.file, locale);
    if !path.exists() {
        if item.file.ends_with("-generated.md") {
            warn!(file = %item.file, "skipping missing generated file");
            return Ok(None);
        }
        anyhow::bail!("required file not found: {path:?}");
    }

    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("read markdown {path:?}"))?;
    let processed = pipeline::process_markdown(&path, &raw, snippets_dir, sidecar)
        .await
        .with_context(|| format!("pipeline on {path:?}"))?;

    let item_id = item.file.trim_end_matches(".md").to_string();
    let guide_link = root.guide_link(&guide.id, locale);
    let full_url = format!("/{guide_link}#{item_id}");

    Ok(Some((item_id, full_url, processed.indexable_text)))
}

fn build_schema() -> Schema {
    let mut b: SchemaBuilder = Schema::builder();
    b.add_text_field("doc_id", STRING | STORED);
    let text_idx = TextFieldIndexing::default()
        .set_tokenizer("docs_text")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let text_opts = TextOptions::default()
        .set_indexing_options(text_idx)
        .set_stored();
    b.add_text_field("title", text_opts.clone());
    b.add_text_field("parent_title", text_opts.clone());
    b.add_text_field("url", text_opts.clone());
    b.add_text_field("parent_url", text_opts.clone());
    b.add_text_field("icon", STRING | STORED);
    b.add_text_field("search_text", text_opts);
    b.build()
}

/// Register the per-locale tokenizer named `docs_text`.
///
/// English-family locales get Porter stemming + lowercasing, matching the
/// FTS5 `tokenize='porter unicode61'` setting in
/// `src/build-search-index-worker.js:70`. Other locales get lowercasing
/// without stemming (safer default — CJK locales may want a dedicated
/// tokenizer like lindera/jieba in a follow-up).
fn register_tokenizers(index: &Index, locale: &str) {
    let analyzer: TextAnalyzer = if locale == "en" || locale == "en_us" {
        TextAnalyzer::builder(SimpleTokenizer::default())
            .filter(RemoveLongFilter::limit(40))
            .filter(LowerCaser)
            .filter(Stemmer::new(Language::English))
            .build()
    } else {
        // Future: extend with per-locale stemmers (Russian, German, etc.) once
        // we've validated they don't regress the FTS5 result set.
        TextAnalyzer::builder(SimpleTokenizer::default())
            .filter(RemoveLongFilter::limit(40))
            .filter(LowerCaser)
            .build()
    };

    index.tokenizers().register("docs_text", analyzer);
}

fn dir_size_bytes(path: &Path) -> Result<u64> {
    let mut total: u64 = 0;
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let md = entry.metadata()?;
        if md.is_file() {
            total += md.len();
        } else if md.is_dir() {
            total += dir_size_bytes(&entry.path())?;
        }
    }
    Ok(total)
}

fn parse_locale_filter(args: impl Iterator<Item = String>) -> Option<Vec<String>> {
    let mut iter = args.peekable();
    while let Some(arg) = iter.next() {
        if arg == "--locale" || arg == "--locales" {
            return iter.next().map(|v| {
                v.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            });
        } else if let Some(rest) = arg.strip_prefix("--locale=") {
            return Some(
                rest.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect(),
            );
        }
    }
    None
}

fn repo_root() -> Result<PathBuf> {
    // The binary should be invoked from the repo root or via build.sh. We walk
    // up from the current working directory until we find `package.json` so
    // it works either way.
    let cwd = std::env::current_dir()?;
    let mut cur: &Path = cwd.as_path();
    loop {
        if cur.join("package.json").exists() && cur.join("src").join("locales.json").exists() {
            return Ok(cur.to_path_buf());
        }
        match cur.parent() {
            Some(p) => cur = p,
            None => anyhow::bail!("could not locate repo root from {:?}", cwd),
        }
    }
}


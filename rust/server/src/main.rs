//! Replaces `src/server-search-engine.js`. Axum on port 5001, reads
//! per-locale Tantivy indexes under `index/<locale>/`, mirrors the existing
//! /search HTTP contract verbatim so the frontend at
//! `src/static/js/search.js` keeps working unchanged.
//!
//! Behavior reference (line-for-line below):
//!
//! * Query sanitization + prefix-OR matching:
//!   `src/server-search-engine.js:234-241`.
//! * BM25 per-field boosts (title 50x, parent_title 10x, search_text 1x):
//!   `src/server-search-engine.js:248-264`.
//! * Locale fallback to default locale when DB missing:
//!   `src/server-search-engine.js:206-228`.
//! * OpenAI rerank + prompt-injection guard:
//!   `src/server-search-engine.js:37-130`.
//! * Search telemetry batch flush:
//!   `src/server-search-engine.js:138-194`.
//! * CORS for /search and OPTIONS:
//!   `src/server-search-engine.js:301-353`.

use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use axum::{
    extract::{ConnectInfo, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use dashmap::DashMap;
use fcdocs_shared::locales::{hreflang_to_locale_key, Locales};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Value};
use tantivy::{Index, IndexReader, ReloadPolicy, TantivyDocument};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

mod ratelimit;
mod telemetry;

use ratelimit::RateLimiter;

const DEFAULT_PORT: u16 = 5001;
const TELEMETRY_FLUSH_SECS: u64 = 10;
const TOP_K: usize = 15;
const BOOST_TITLE: f32 = 50.0;
const BOOST_PARENT_TITLE: f32 = 10.0;
const BOOST_SEARCH_TEXT: f32 = 1.0;

/// Hard cap on the `query` string. Anything longer is rejected at the
/// edge: it can't help recall, but it can drive OpenAI tokens and CPU.
const MAX_QUERY_LEN: usize = 256;
/// Tokens-per-second + burst, per remote IP. Matches "a human typing
/// search queries" rather than a scraper.
const RATE_PER_SEC: f64 = 5.0;
const RATE_BURST: f64 = 20.0;

#[derive(Clone)]
struct AppState {
    locales: Arc<Locales>,
    index_dir: Arc<PathBuf>,
    cache: Arc<DashMap<String, LocaleSlot>>,
    telemetry: Arc<Mutex<telemetry::Collector>>,
    openai_api_key: Option<Arc<String>>,
    openai_model: Arc<String>,
    search_api_key: Option<Arc<String>>,
    http: reqwest::Client,
    limiter: Arc<RateLimiter>,
}

#[derive(Clone)]
struct LocaleSlot {
    index: Arc<Index>,
    reader: Arc<IndexReader>,
    f_doc_id: Field,
    f_title: Field,
    f_parent_title: Field,
    f_url: Field,
    f_parent_url: Field,
    f_icon: Field,
    f_search_text: Field,
}

#[derive(Debug, Deserialize)]
struct SearchParams {
    #[serde(default)]
    query: String,
    #[serde(default)]
    locale: Option<String>,
    #[serde(rename = "tenantId", default)]
    tenant_id: Option<String>,
    #[serde(default, deserialize_with = "deserialize_bool")]
    full: bool,
    #[serde(default, deserialize_with = "deserialize_bool")]
    nollm: bool,
}

fn deserialize_bool<'de, D>(de: D) -> std::result::Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(de)?;
    Ok(matches!(s.as_deref(), Some("true") | Some("1")))
}

#[derive(Debug, Serialize, Clone)]
struct SearchResult {
    id: String,
    title: String,
    #[serde(rename = "parentTitle")]
    parent_title: Option<String>,
    url: String,
    #[serde(rename = "parentUrl")]
    parent_url: Option<String>,
    icon: Option<String>,
    score: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip)]
    search_text: String,
}

#[derive(Debug, Serialize)]
struct SearchResponse {
    status: &'static str,
    results: Vec<SearchResult>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: &'static str,
    error: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let repo_root = repo_root()?;
    let index_dir = repo_root.join("index");
    let locales_path = repo_root.join("src/locales.json");
    if !index_dir.exists() {
        anyhow::bail!(
            "index dir missing: {:?}. Run the indexer first: ./indexer",
            index_dir
        );
    }
    if !locales_path.exists() {
        anyhow::bail!("locales.json missing: {:?}", locales_path);
    }

    let locales = Arc::new(Locales::load_from(&locales_path)?);
    let available: Vec<String> = locales
        .keys()
        .filter(|loc| index_dir.join(loc).join("meta.json").exists())
        .map(|s| s.to_string())
        .collect();
    if available.is_empty() {
        anyhow::bail!(
            "no Tantivy indexes found under {:?} — run the indexer first",
            index_dir
        );
    }

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_PORT);

    info!(
        port,
        available_locales = available.len(),
        "starting search server"
    );

    let http = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(20))
        .pool_idle_timeout(Duration::from_secs(60))
        .build()
        .context("build shared reqwest client")?;

    let state = AppState {
        locales,
        index_dir: Arc::new(index_dir),
        cache: Arc::new(DashMap::new()),
        telemetry: Arc::new(Mutex::new(telemetry::Collector::default())),
        openai_api_key: std::env::var("OPENAI_API_KEY").ok().map(Arc::new),
        openai_model: Arc::new(
            std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-5-mini".to_string()),
        ),
        search_api_key: std::env::var("SEARCH_API_KEY").ok().map(Arc::new),
        http,
        limiter: Arc::new(RateLimiter::new(RATE_PER_SEC, RATE_BURST)),
    };

    // Background telemetry flusher.
    {
        let st = state.clone();
        tokio::spawn(async move {
            telemetry::flush_loop(st).await;
        });
    }
    // Background limiter GC so idle IPs don't pin memory.
    {
        let limiter = state.limiter.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(60));
            ticker.tick().await;
            loop {
                ticker.tick().await;
                limiter.gc(Duration::from_secs(600));
            }
        });
    }

    // CORS: docs site is public, no credentials are exchanged. Allow any
    // origin for GET/OPTIONS, never set Access-Control-Allow-Credentials.
    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([axum::http::Method::GET, axum::http::Method::OPTIONS])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/search", get(handle_search).options(handle_options))
        .route("/health", get(|| async { Json(serde_json::json!({"ok": true})) }))
        .with_state(state)
        .layer(cors);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(%addr, "search server listening");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;
    Ok(())
}

async fn handle_options() -> impl IntoResponse {
    StatusCode::OK
}

async fn handle_search(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    Query(params): Query<SearchParams>,
) -> impl IntoResponse {
    let ip = client_ip(&headers, addr.ip());
    if !state.limiter.allow(ip) {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(serde_json::to_value(ErrorResponse {
                status: "failed",
                error: "rate limit exceeded".into(),
            })
            .unwrap()),
        );
    }

    if params.query.len() > MAX_QUERY_LEN {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::to_value(ErrorResponse {
                status: "failed",
                error: format!("query exceeds {MAX_QUERY_LEN}-char limit"),
            })
            .unwrap()),
        );
    }

    let locale = hreflang_to_locale_key(params.locale.as_deref(), &state.locales.default_locale);
    let query = params.query;
    info!(query = %query, locale = %locale, "search");

    let mut results = match run_search(&state, &locale, &query) {
        Ok(r) => r,
        Err(e) => {
            error!("search error: {e:#}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::to_value(ErrorResponse {
                    status: "failed",
                    error: e.to_string(),
                })
                .unwrap()),
            );
        }
    };

    info!(count = results.len(), query = %query, "results");

    if !params.nollm {
        if let Some(key) = state.openai_api_key.clone() {
            results = reorder_with_openai(
                &state.http,
                &query,
                results,
                &key,
                &state.openai_model,
            )
            .await;
        }
    }

    // Strip or expose search_text per `full` flag.
    if params.full {
        for r in &mut results {
            r.body = Some(std::mem::take(&mut r.search_text));
        }
    } else {
        for r in &mut results {
            r.search_text.clear();
        }
    }

    let body = SearchResponse {
        status: "success",
        results,
    };

    // Record telemetry (capped + length-truncated in record()).
    let tenant_id = params.tenant_id.unwrap_or_else(|| "default".to_string());
    {
        let mut t = state.telemetry.lock().await;
        t.record(tenant_id, query);
    }

    (StatusCode::OK, Json(serde_json::to_value(body).unwrap()))
}

/// Prefer the first X-Forwarded-For hop if we're behind a reverse proxy;
/// fall back to the socket peer. Mainly so rate-limiting works correctly
/// when the server sits behind nginx/cloudflare.
fn client_ip(headers: &HeaderMap, peer: IpAddr) -> IpAddr {
    if let Some(xff) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(first) = xff.split(',').next() {
            if let Ok(ip) = first.trim().parse::<IpAddr>() {
                return ip;
            }
        }
    }
    peer
}


fn run_search(state: &AppState, locale: &str, query: &str) -> Result<Vec<SearchResult>> {
    let slot = get_locale_slot(state, locale)?;
    let terms = sanitize_query(query);
    if terms.is_empty() {
        return Ok(Vec::new());
    }

    // Use Tantivy's QueryParser so each `term*` becomes a proper BM25-scored
    // prefix query against title + parent_title + search_text. Per-field
    // boosts replicate the FTS5 column weights at
    // `src/server-search-engine.js:260`: title 50x, parent_title 10x,
    // search_text 1x. QueryParser also handles the symmetric tokenization
    // (lowercase + stemming) so a query like "installation" stems through
    // the analyzer and matches the stemmed "install" terms in the index.
    let mut qp = QueryParser::for_index(
        &slot.index,
        vec![slot.f_title, slot.f_parent_title, slot.f_search_text],
    );
    qp.set_field_boost(slot.f_title, BOOST_TITLE);
    qp.set_field_boost(slot.f_parent_title, BOOST_PARENT_TITLE);
    qp.set_field_boost(slot.f_search_text, BOOST_SEARCH_TEXT);

    let parts: Vec<String> = terms
        .iter()
        .map(|t| format!("{}*", escape_for_query_parser(t)))
        .collect();
    let q_str = parts.join(" OR ");
    let q = match qp.parse_query(&q_str) {
        Ok(q) => q,
        Err(e) => {
            warn!(query = %q_str, error = %e, "query parser failed; returning empty");
            return Ok(Vec::new());
        }
    };

    let searcher = slot.reader.searcher();
    let top = searcher.search(&q, &TopDocs::with_limit(TOP_K))?;
    let mut out = Vec::with_capacity(top.len());
    for (score, addr) in top {
        let doc: TantivyDocument = searcher.doc(addr)?;
        let id = field_str(&doc, slot.f_doc_id).unwrap_or_default();
        let title = field_str(&doc, slot.f_title).unwrap_or_default();
        let parent_title = field_str(&doc, slot.f_parent_title);
        let url = field_str(&doc, slot.f_url).unwrap_or_default();
        let parent_url = field_str(&doc, slot.f_parent_url);
        let icon = field_str(&doc, slot.f_icon);
        let search_text = field_str(&doc, slot.f_search_text).unwrap_or_default();
        out.push(SearchResult {
            id,
            title,
            parent_title,
            url,
            parent_url,
            icon,
            score,
            body: None,
            search_text,
        });
    }
    Ok(out)
}

fn field_str(doc: &TantivyDocument, field: Field) -> Option<String> {
    doc.get_first(field).and_then(|v| v.as_str().map(|s| s.to_string()))
}

/// Escape characters that Tantivy's QueryParser treats as syntax
/// (`:`, `*`, `?`, `\`, `+`, `-`, `(`, `)`, `[`, `]`, `{`, `}`, `^`, `~`,
/// `"`, AND/OR keywords) — needed because we append our own trailing `*`
/// for the prefix wildcard and don't want user input to inject extra
/// operators. Most of these are already removed by `sanitize_query`, but
/// some pass-through characters (`/`, `.`, `,`, `\`, `?`) still warrant a
/// belt-and-braces escape.
fn escape_for_query_parser(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 4);
    for ch in input.chars() {
        match ch {
            '\\' | '"' | ':' | '*' | '?' | '+' | '-' | '(' | ')' | '[' | ']' | '{' | '}'
            | '^' | '~' | '/' | '!' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}

fn get_locale_slot(state: &AppState, locale: &str) -> Result<LocaleSlot> {
    if let Some(slot) = state.cache.get(locale) {
        return Ok(slot.clone());
    }
    // Compute the EFFECTIVE locale ONCE and use it for both the on-disk
    // index dir and the analyzer choice. The previous code keyed the
    // analyzer off the requested locale even when we'd just fallen back
    // to default — so a request for `fr_fr` (no fr_fr index) would open
    // `index/en/` (English Porter-stemmer corpus) and register a
    // *non*-stemming analyzer for "docs_text", because `fr_fr` isn't en
    // or en_us. Query "installation" then tokenized to "installation"
    // and missed the "install"-stemmed terms in the index. Silent recall
    // degradation for every fallback locale.
    let requested_dir = state.index_dir.join(locale);
    let (effective_locale, dir) = if requested_dir.exists() {
        (locale.to_string(), requested_dir)
    } else {
        warn!(locale, "no index for locale, falling back to default");
        (
            state.locales.default_locale.clone(),
            state.index_dir.join(&state.locales.default_locale),
        )
    };
    let index = Index::open_in_dir(&dir).with_context(|| format!("open index {dir:?}"))?;
    // Use the effective-locale's analyzer so query tokenization matches
    // the analyzer the indexer used when it wrote this dir.
    index
        .tokenizers()
        .register("docs_text", build_docs_text_analyzer(&effective_locale));
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;
    let schema = index.schema();
    let slot = LocaleSlot {
        f_doc_id: schema.get_field("doc_id")?,
        f_title: schema.get_field("title")?,
        f_parent_title: schema.get_field("parent_title")?,
        f_url: schema.get_field("url")?,
        f_parent_url: schema.get_field("parent_url")?,
        f_icon: schema.get_field("icon")?,
        f_search_text: schema.get_field("search_text")?,
        index: Arc::new(index),
        reader: Arc::new(reader),
    };
    // Cache under the requested key (cheap hit path) AND under the
    // effective key (so the next request for the default locale doesn't
    // re-open + re-register).
    state.cache.insert(locale.to_string(), slot.clone());
    if effective_locale != locale {
        state.cache.insert(effective_locale, slot.clone());
    }
    Ok(slot)
}

/// Pick the `docs_text` analyzer for a locale's index. Must mirror the
/// indexer's `register_tokenizers` in `rust/indexer/src/main.rs` —
/// drift between the two would cause asymmetric tokenization and
/// recall holes.
///
/// English Porter applied to EVERY locale (including non-English),
/// matching Node's `tokenize='porter unicode61'` at
/// `src/build-search-index-worker.js:70`. See the longer rationale
/// in the indexer's register_tokenizers doc-comment.
fn build_docs_text_analyzer(_locale: &str) -> tantivy::tokenizer::TextAnalyzer {
    use tantivy::tokenizer::{
        Language, LowerCaser, RemoveLongFilter, SimpleTokenizer, Stemmer, TextAnalyzer,
    };
    TextAnalyzer::builder(SimpleTokenizer::default())
        .filter(RemoveLongFilter::limit(40))
        .filter(LowerCaser)
        .filter(Stemmer::new(Language::English))
        .build()
}

/// Mirrors src/server-search-engine.js:234-241.
fn sanitize_query(q: &str) -> Vec<String> {
    static QUOTES: Lazy<Regex> = Lazy::new(|| Regex::new(r#"['"]"#).expect("re"));
    static SPECIAL: Lazy<Regex> = Lazy::new(|| Regex::new(r"[-+*(){}\[\]^~:]").expect("re"));
    let no_quotes = QUOTES.replace_all(q, "").into_owned();
    let spaced = SPECIAL.replace_all(&no_quotes, " ").into_owned();
    spaced
        .split_whitespace()
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect()
}

/// Prompt-injection patterns ported verbatim from
/// `src/server-search-engine.js:17-30`.
static INJECTION_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    let pats: &[&str] = &[
        r"(?i)ignore\s+(all\s+)?(previous|prior|above|earlier)\s+(instructions?|prompts?|rules?|context)",
        r"(?i)disregard\s+(all\s+)?(previous|prior|above|earlier)\s+(instructions?|prompts?|rules?|context)",
        r"(?i)forget\s+(all\s+)?(previous|prior|above|earlier)\s+(instructions?|prompts?|rules?|context)",
        r"(?i)override\s+(all\s+)?(previous|prior|above|earlier)\s+(instructions?|prompts?|rules?|context)",
        r"(?i)you\s+are\s+(now\s+)?(a|an)\s+",
        r"(?i)new\s+instructions?:",
        r"(?i)system\s*prompt",
        r"(?i)\bact\s+as\b",
        r"(?i)\brole\s*play\b",
        r"(?i)pretend\s+(you('re|are)|to\s+be)",
        r"(?i)do\s+not\s+follow",
        r"(?i)bypass\s+(the\s+)?(rules?|restrictions?|filters?)",
    ];
    pats.iter().map(|p| Regex::new(p).expect("regex")).collect()
});

fn contains_prompt_injection(text: &str) -> bool {
    INJECTION_PATTERNS.iter().any(|re| re.is_match(text))
}

/// Port of reorderResultsWithOpenAI from
/// src/server-search-engine.js:37-130.
async fn reorder_with_openai(
    client: &reqwest::Client,
    query: &str,
    results: Vec<SearchResult>,
    api_key: &str,
    model: &str,
) -> Vec<SearchResult> {
    if results.len() <= 1 {
        info!("OpenAI reranking skipped: 0-1 results");
        return results;
    }
    if contains_prompt_injection(query) {
        info!("Prompt injection detected in query, skipping OpenAI reranking");
        return results;
    }

    let results_list = results
        .iter()
        .map(|r| {
            let parent = r.parent_title.as_deref().unwrap_or("none");
            format!("[{id}] \"{title}\" (parent: \"{parent}\")", id = r.id, title = r.title, parent = parent)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let body = serde_json::json!({
        "model": model,
        "messages": [
            {"role": "system", "content": "You are a search result ranker for FastComments documentation. Given a search query and a list of results with IDs in brackets, return ONLY the IDs in order of relevance (most relevant first). Output only comma-separated IDs, nothing else. Example output: guide-auth,api-sso,ref-users\n\nImportant ranking hints:\n- For queries about installing, adding, or setting up FastComments on a website (e.g., \"install\", \"add to site\", \"setup\", \"getting started\", \"how to add\"), the guide-installation result should be ranked first if present."},
            {"role": "user", "content": format!("Query: \"{query}\"\n\nResults:\n{results_list}")}
        ],
        "max_completion_tokens": 4000,
    });

    info!(model, n = results.len(), "OpenAI reranking");
    let resp = match client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            error!("OpenAI request failed: {e}");
            return results;
        }
    };

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        error!("OpenAI API error {status}: {text}");
        return results;
    }
    let data: serde_json::Value = match resp.json().await {
        Ok(v) => v,
        Err(e) => {
            error!("OpenAI parse failed: {e}");
            return results;
        }
    };

    let ranking_text = data
        .pointer("/choices/0/message/content")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    if ranking_text.is_empty() {
        warn!("OpenAI reranking: empty ranking text");
        return results;
    }
    let ranked: Vec<String> = ranking_text
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if ranked.is_empty() {
        warn!("OpenAI reranking: failed to parse ids");
        return results;
    }

    let mut by_id: HashMap<String, SearchResult> =
        results.iter().map(|r| (r.id.clone(), r.clone())).collect();
    let mut out: Vec<SearchResult> = Vec::with_capacity(results.len());
    let mut used: HashSet<String> = HashSet::new();
    for id in &ranked {
        if used.contains(id) {
            continue;
        }
        if let Some(r) = by_id.remove(id) {
            out.push(r);
            used.insert(id.clone());
        }
    }
    for r in results {
        if !used.contains(&r.id) {
            out.push(r);
        }
    }
    info!(n = out.len(), "OpenAI reranking ok");
    out
}

async fn shutdown_signal() {
    use tokio::signal;
    let ctrl_c = async {
        signal::ctrl_c().await.expect("install ctrl_c handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("install SIGTERM handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }
    info!("shutdown signal received");
}

fn repo_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let mut cur: &std::path::Path = cwd.as_path();
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

#[cfg(test)]
mod fallback_tests {
    //! End-to-end test that pins the locale-fallback analyzer choice.
    //! Builds a minimal en index with stem-able content, requests a
    //! search for a fr_fr locale (no fr_fr dir on disk), and asserts
    //! the English-stemmer analyzer is registered against the
    //! default-locale index so query "installation" still hits a
    //! document containing "install".

    use super::*;
    use fcdocs_shared::locales::Locale;
    use indexmap::IndexMap;
    use tantivy::doc;
    use tantivy::schema::{
        IndexRecordOption, Schema, TextFieldIndexing, TextOptions, STORED, STRING,
    };

    fn build_test_schema() -> Schema {
        // Must match what the indexer writes (see rust/indexer/src/main.rs
        // build_schema). Drift here would mean the test indexes a doc
        // the server can't read.
        let mut b = Schema::builder();
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

    fn write_en_index_with_install_doc(en_dir: &std::path::Path) {
        std::fs::create_dir_all(en_dir).unwrap();
        let schema = build_test_schema();
        let index = Index::create_in_dir(en_dir, schema.clone()).unwrap();
        // Use the same analyzer the indexer would have registered when
        // writing en/.
        index
            .tokenizers()
            .register("docs_text", build_docs_text_analyzer("en"));
        let mut writer = index.writer(50_000_000).unwrap();
        let title = schema.get_field("title").unwrap();
        let url = schema.get_field("url").unwrap();
        let search_text = schema.get_field("search_text").unwrap();
        let doc_id = schema.get_field("doc_id").unwrap();
        writer
            .add_document(doc!(
                doc_id => "guide-install>quickstart",
                title => "Install FastComments",
                url => "/guide-install.html#quickstart",
                search_text => "How to install FastComments on your site quickly.",
            ))
            .unwrap();
        writer.commit().unwrap();
    }

    fn fake_state(index_dir: PathBuf) -> AppState {
        // Minimal locales with en as default + fr_fr defined but with
        // no on-disk index dir, so the fallback path fires.
        let mut locales_map: IndexMap<String, Locale> = IndexMap::new();
        locales_map.insert(
            "en".into(),
            Locale {
                name: "English".into(),
                native_name: "English".into(),
                hreflang: "en".into(),
                flag: Some("🇬🇧".into()),
            },
        );
        locales_map.insert(
            "fr_fr".into(),
            Locale {
                name: "French".into(),
                native_name: "Français".into(),
                hreflang: "fr-FR".into(),
                flag: Some("🇫🇷".into()),
            },
        );
        let locales = Arc::new(Locales {
            default_locale: "en".into(),
            locales: locales_map,
        });
        AppState {
            locales,
            index_dir: Arc::new(index_dir),
            cache: Arc::new(DashMap::new()),
            telemetry: Arc::new(Mutex::new(telemetry::Collector::default())),
            openai_api_key: None,
            openai_model: Arc::new("test".into()),
            search_api_key: None,
            http: reqwest::Client::new(),
            limiter: Arc::new(RateLimiter::new(1000.0, 1000.0)),
        }
    }

    #[test]
    fn fallback_to_default_uses_default_locale_analyzer() {
        let tmp = tempfile::tempdir().unwrap();
        let index_dir = tmp.path().to_path_buf();
        // Only en/ exists; fr_fr/ is intentionally missing.
        write_en_index_with_install_doc(&index_dir.join("en"));

        let state = fake_state(index_dir);

        // Request fr_fr (no index dir) -> server must open en/ AND
        // register the en analyzer. Without that, "installation" tokenizes
        // unstemmed and misses the "install"-stemmed term in the index.
        let results = run_search(&state, "fr_fr", "installation").unwrap();
        assert!(
            !results.is_empty(),
            "fallback path missed a document the English-stemmed index should match \
             (analyzer drift between index and query)"
        );
        assert_eq!(results[0].id, "guide-install>quickstart");
    }

    #[test]
    fn requested_locale_with_index_uses_its_own_analyzer() {
        // Sanity check the non-fallback path still works.
        let tmp = tempfile::tempdir().unwrap();
        let index_dir = tmp.path().to_path_buf();
        write_en_index_with_install_doc(&index_dir.join("en"));
        let state = fake_state(index_dir);
        let results = run_search(&state, "en", "installation").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "guide-install>quickstart");
    }

    /// Pins parity with Node's `tokenize='porter unicode61'` setting
    /// applied to EVERY locale. A fr_fr index containing "installing"
    /// must match a fr_fr query for "installation" because both
    /// stem to "instal" via English Porter. Before this fix,
    /// non-English locales registered a no-stemmer analyzer and the
    /// match would silently miss — the existing search-regression.js
    /// harness only tested en, so the drift hid behind a green CI.
    #[test]
    fn non_english_locale_still_stems_per_node_parity() {
        let tmp = tempfile::tempdir().unwrap();
        let index_dir = tmp.path().to_path_buf();
        let fr_dir = index_dir.join("fr_fr");
        std::fs::create_dir_all(&fr_dir).unwrap();
        let schema = build_test_schema();
        let index = Index::create_in_dir(&fr_dir, schema.clone()).unwrap();
        // Indexer would have registered the (now-Porter-everywhere)
        // analyzer for fr_fr. We mirror that here.
        index
            .tokenizers()
            .register("docs_text", build_docs_text_analyzer("fr_fr"));
        let mut writer = index.writer(50_000_000).unwrap();
        let title = schema.get_field("title").unwrap();
        let url = schema.get_field("url").unwrap();
        let search_text = schema.get_field("search_text").unwrap();
        let doc_id = schema.get_field("doc_id").unwrap();
        writer
            .add_document(doc!(
                doc_id => "guide-fr>quickstart",
                title => "Installing FastComments",
                url => "/guide-fr.html#quickstart",
                search_text => "How to start installing FastComments on your site.",
            ))
            .unwrap();
        writer.commit().unwrap();

        let state = fake_state(index_dir);
        // Query for a different surface form of "install" — only matches
        // when the indexer + query analyzer both stem.
        let results = run_search(&state, "fr_fr", "installation").unwrap();
        assert!(
            !results.is_empty(),
            "fr_fr query 'installation' missed a doc indexed as 'installing' — \
             non-English locales need the same Porter stemmer Node applied via \
             tokenize='porter unicode61'."
        );
        assert_eq!(results[0].id, "guide-fr>quickstart");
    }
}


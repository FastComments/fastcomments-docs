//! LLM chat-completions client with disk-backed cache + model fallback.
//!
//! Mirrors `src/sdk-doc-generators/openai-client.js`:
//! - Cache directory laid out the same way (`<cache_dir>/<sha256>.json`).
//! - Cache file body matches the Node shape `{ method, codeExample, timestamp, model }`.
//! - Retries the primary model, then any `LLM_FALLBACK_MODELS`, for
//!   oversized/transient requests.

use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use thiserror::Error;

use crate::cache_key::sha256_hex;

/// Per-request read timeout. Node's default was 600s, which mostly
/// papered over hung connections. 120s is generous for LLM chat
/// completions but fails fast when the network melts.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
/// How many times to retry the SAME model on transient failure (429,
/// 5xx, connection error) before moving to the next fallback.
const RETRIES_PER_MODEL: u32 = 3;
const RETRY_BASE_DELAY: Duration = Duration::from_millis(500);
const RETRY_MAX_DELAY: Duration = Duration::from_secs(30);

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("LLM API returned status {status}: {body}")]
    Api { status: u16, body: String },
    #[error("LLM response missing choices/message/content")]
    BadResponse,
    #[error("cache directory not writable: {0}")]
    Cache(PathBuf),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub text: String,
    pub model_used: String,
    pub cached: bool,
}

#[derive(Debug, Clone)]
pub struct LlmClient {
    pub cache_dir: PathBuf,
    pub api_key: Option<String>,
    pub model: String,
    pub language: String,
    http: reqwest::Client,
    /// Models tried in order. First is the configured `model`, then fallbacks.
    pub fallback_models: Vec<String>,
    /// Full chat-completions URL (`{LLM_BASE_URL}/chat/completions`).
    endpoint_url: String,
}

impl LlmClient {
    pub fn new(
        cache_dir: impl Into<PathBuf>,
        model: impl Into<String>,
        language: impl Into<String>,
    ) -> Result<Self> {
        let cache_dir = cache_dir.into();
        // Cache dir creation is rare (called from sync builder); leave
        // as std::fs since the alternative is an async constructor and
        // the callers all live in async fn already.
        std::fs::create_dir_all(&cache_dir)
            .with_context(|| format!("create cache dir {cache_dir:?}"))?;
        let model = model.into();
        let language = language.into();
        let fallback_models = default_fallbacks(&model);
        Ok(Self {
            cache_dir,
            api_key: std::env::var("LLM_API_KEY").ok(),
            model,
            language,
            http: reqwest::Client::builder()
                .connect_timeout(CONNECT_TIMEOUT)
                .timeout(REQUEST_TIMEOUT)
                .build()
                .expect("reqwest client"),
            fallback_models,
            endpoint_url: chat_completions_url(),
        })
    }

    /// Compute the cache key for a (method, prompt, model) tuple,
    /// matching Node's `openai-client.js:58-72`.
    ///
    /// Critical detail: Node does
    ///     const data = { methodName: method.name, ..., httpMethod: method.httpMethod, ... };
    /// then `JSON.stringify(sortObjectKeys(data))`. When `method.httpMethod`
    /// is `undefined`, `JSON.stringify` drops the key entirely. The previous
    /// Rust port serialized `Option::None` as `Value::Null`, so we hashed
    /// `{"httpMethod":null,...}` while Node hashed `{...}` without the key.
    /// That produced two different SHA256s for the same logical input and
    /// missed every Node-shaped cache file — every cache-cold method was
    /// re-generated against the LLM, often with different (sometimes wrong)
    /// output than the cached Node version.
    ///
    /// We restore parity by building the JSON object manually and skipping
    /// any field where `method_meta.get(...)` returns `None`.
    pub fn cache_key(&self, method_meta: &Value, prompt: &str) -> String {
        let mut data = serde_json::Map::with_capacity(8);
        // (out_key, source_key) — out_key is the JSON property the cache
        // key sees; source_key matches whatever `to_value(&method)` produced
        // (Method struct uses #[serde(rename = ...)]).
        for (out_key, src_key) in [
            ("methodName", "name"),
            ("parameters", "parameters"),
            ("responseType", "responseType"),
            ("nestedTypes", "nestedTypes"),
            ("httpMethod", "httpMethod"),
            ("path", "path"),
        ] {
            if let Some(v) = method_meta.get(src_key) {
                // Node note: an explicit JSON null in method_meta is
                // preserved as `"key":null` by JSON.stringify. We mirror
                // that — only true `undefined` / missing keys drop out.
                data.insert(out_key.to_string(), v.clone());
            }
        }
        data.insert("prompt".to_string(), Value::String(prompt.to_string()));
        data.insert("model".to_string(), Value::String(self.model.clone()));
        sha256_hex(&Value::Object(data))
    }

    /// Cache-only lookup. Returns `Some(code_example)` if the cache
    /// file exists, else `None`. Use this when an API key isn't
    /// available (parity validation, CI, etc.) to surface only the
    /// cache hits.
    pub async fn get_cached(&self, method_meta: &Value, prompt: &str) -> Option<String> {
        let key = self.cache_key(method_meta, prompt);
        let path = self.cache_dir.join(format!("{key}.json"));
        read_cache(&path).await.map(|c| c.code_example)
    }

    /// Generate a completion, with cache lookup keyed by
    /// SHA256(stable_stringify({ method, prompt, model })).
    /// `method_meta` should mirror the Node call site at
    /// `src/sdk-doc-generators/openai-client.js:58-72`.
    pub async fn generate(&self, method_meta: &Value, prompt: &str) -> Result<LlmResponse> {
        let cache_key = self.cache_key(method_meta, prompt);
        let cache_file = self.cache_dir.join(format!("{cache_key}.json"));

        if let Some(cached) = read_cache(&cache_file).await {
            tracing::debug!(method = ?method_meta.get("name"), "llm cache hit");
            return Ok(LlmResponse {
                text: cached.code_example,
                model_used: cached.model.unwrap_or_else(|| self.model.clone()),
                cached: true,
            });
        }

        let api_key = self
            .api_key
            .as_deref()
            .context("LLM_API_KEY not set")?;

        let mut last_err: Option<anyhow::Error> = None;
        for model in &self.fallback_models {
            match self.call_llm_retrying(api_key, model, prompt).await {
                Ok(text) => {
                    let _ = write_cache(
                        &cache_file,
                        &CacheRecord {
                            method: method_meta
                                .get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            signature_hash: cache_key.clone(),
                            generated_at: now_iso8601(),
                            code_example: text.clone(),
                            model: Some(model.clone()),
                            prompt_tokens: None,
                            completion_tokens: None,
                        },
                    )
                    .await;
                    return Ok(LlmResponse {
                        text,
                        model_used: model.clone(),
                        cached: false,
                    });
                }
                Err(e) => {
                    tracing::warn!(model = %model, error = %e, "LLM call failed; trying next fallback");
                    last_err = Some(e);
                }
            }
        }
        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("no models attempted")))
    }

    /// Retry the same `model` on transient errors before bubbling up
    /// to the fallback chain. Without this, a single 429 immediately
    /// promotes to the (more expensive) next-tier model.
    async fn call_llm_retrying(
        &self,
        api_key: &str,
        model: &str,
        prompt: &str,
    ) -> Result<String> {
        let mut last_err: Option<anyhow::Error> = None;
        for attempt in 0..RETRIES_PER_MODEL {
            match self.call_llm(api_key, model, prompt).await {
                Ok(text) => return Ok(text),
                Err(e) => {
                    if !is_transient(&e) || attempt == RETRIES_PER_MODEL - 1 {
                        return Err(e);
                    }
                    let delay = backoff_for(attempt);
                    tracing::warn!(
                        model,
                        attempt = attempt + 1,
                        delay_ms = delay.as_millis() as u64,
                        error = %e,
                        "transient LLM failure; backing off"
                    );
                    last_err = Some(e);
                    tokio::time::sleep(delay).await;
                }
            }
        }
        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("retry loop fell through")))
    }

    async fn call_llm(&self, api_key: &str, model: &str, prompt: &str) -> Result<String> {
        // Three things to keep in lockstep with Node
        // (src/sdk-doc-generators/openai-client.js:115-126, 386-416):
        //
        // 1. Send a language-specific system persona, not just the user
        //    prompt. The persona materially affects the LLM's tone and
        //    idiomaticity — without it, code examples drift to
        //    generic-style output that doesn't match Node's cache.
        // 2. max_tokens = 8000 (Node used 8000), not 4000. Halving the
        //    budget truncates longer snippets mid-function.
        // 3. Strip the leading/trailing markdown code fence the model
        //    sometimes wraps the answer in (```typescript / ```rust /
        //    plain ```). Otherwise the cached output starts with a
        //    fence that the rendered markdown will display literally.
        let body = json!({
            "model": model,
            "messages": [
                {"role": "system", "content": self.system_message()},
                {"role": "user", "content": prompt},
            ],
            "max_tokens": 8000,
        });
        let resp = self
            .http
            .post(&self.endpoint_url)
            .bearer_auth(api_key)
            .json(&body)
            .send()
            .await
            .context("LLM POST")?;
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(LlmError::Api {
                status: status.as_u16(),
                body: text,
            }
            .into());
        }
        let value: Value = resp.json().await.context("parse LLM response")?;
        let raw = value
            .pointer("/choices/0/message/content")
            .and_then(|v| v.as_str())
            .ok_or(LlmError::BadResponse)?;
        Ok(strip_code_fences(raw))
    }

    /// Mirrors Node's `getSystemMessage` at openai-client.js:115-126.
    /// Language-specific persona; falls back to TypeScript for any
    /// language we haven't seen (matches Node's else branch).
    fn system_message(&self) -> &'static str {
        match self.language.as_str() {
            "rust" => {
                "You are an expert Rust developer generating realistic, idiomatic API usage examples for the FastComments API."
            }
            "cpp" => {
                "You are an expert C++ developer generating realistic, idiomatic API usage examples for the FastComments API."
            }
            "nim" => {
                "You are an expert Nim developer generating realistic, idiomatic API usage examples for the FastComments API."
            }
            _ => {
                "You are an expert TypeScript developer generating realistic API usage examples for the FastComments API."
            }
        }
    }
}

/// Strip leading ` ```typescript\n` / ` ```ts\n` / ` ```rust\n` / ` ```\n`
/// and trailing `\n``` ` from the model's reply, matching Node's
/// `cleanCode` chain at openai-client.js:411-416. Also `.trim()`s,
/// which Node does up-front on line 408.
fn strip_code_fences(s: &str) -> String {
    let trimmed = s.trim();
    let mut body = trimmed;
    for prefix in ["```typescript\n", "```ts\n", "```rust\n", "```cpp\n", "```nim\n", "```\n"] {
        if let Some(rest) = body.strip_prefix(prefix) {
            body = rest;
            break;
        }
    }
    if let Some(rest) = body.strip_suffix("\n```") {
        body = rest;
    }
    body.to_string()
}

fn is_transient(err: &anyhow::Error) -> bool {
    if let Some(api) = err.downcast_ref::<LlmError>() {
        return matches!(api, LlmError::Api { status, .. } if *status == 429 || (*status >= 500 && *status < 600));
    }
    // reqwest connection / timeout errors land here.
    if let Some(re) = err.downcast_ref::<reqwest::Error>() {
        return re.is_timeout() || re.is_connect() || re.is_request();
    }
    false
}

fn backoff_for(attempt: u32) -> Duration {
    // Exponential backoff with jitter: 500ms, 1s, 2s, 4s, ... capped at
    // RETRY_MAX_DELAY. Jitter (±25%) avoids thundering-herd on shared
    // 429s.
    let base = RETRY_BASE_DELAY.as_millis() as u64;
    let pow = 1u64 << attempt.min(10);
    let raw = base.saturating_mul(pow).min(RETRY_MAX_DELAY.as_millis() as u64);
    // Cheap deterministic jitter from monotonic clock; no rand crate.
    let now_nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64)
        .unwrap_or(0);
    let jitter = (now_nanos % (raw / 2 + 1)) as i64 - (raw as i64 / 4);
    Duration::from_millis((raw as i64 + jitter).max(1) as u64)
}

/// Default LLM API base (no trailing `/chat/completions`): DeepInfra's
/// chat endpoint. Override with `LLM_BASE_URL`.
pub const DEFAULT_BASE_URL: &str = "https://api.deepinfra.com/v1/openai";

/// Build the chat-completions endpoint from `LLM_BASE_URL` (falling back
/// to [`DEFAULT_BASE_URL`]). Trailing slashes on the base are tolerated.
fn chat_completions_url() -> String {
    let base = std::env::var("LLM_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
    format!("{}/chat/completions", base.trim_end_matches('/'))
}

fn default_fallbacks(primary: &str) -> Vec<String> {
    // Primary model first, then any models from `LLM_FALLBACK_MODELS`
    // (comma-separated). Empty by default so no provider-specific model
    // names are baked into the binary.
    let mut out = vec![primary.to_string()];
    let fallbacks = std::env::var("LLM_FALLBACK_MODELS").unwrap_or_default();
    for m in fallbacks.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        if !out.iter().any(|x| x == m) {
            out.push(m.to_string());
        }
    }
    out
}

/// Matches the on-disk cache shape written by
/// `src/sdk-doc-generators/openai-client.js:419-427`:
///
/// ```json
/// {
///   "method": "...",
///   "signatureHash": "...",
///   "generatedAt": "ISO-8601 timestamp",
///   "codeExample": "...",
///   "model": "...",
///   "promptTokens": N,
///   "completionTokens": N
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
struct CacheRecord {
    method: String,
    #[serde(rename = "signatureHash", default)]
    signature_hash: String,
    #[serde(rename = "generatedAt", default)]
    generated_at: String,
    #[serde(rename = "codeExample")]
    code_example: String,
    #[serde(default)]
    model: Option<String>,
    #[serde(rename = "promptTokens", default, skip_serializing_if = "Option::is_none")]
    prompt_tokens: Option<u64>,
    #[serde(rename = "completionTokens", default, skip_serializing_if = "Option::is_none")]
    completion_tokens: Option<u64>,
}

async fn read_cache(path: &Path) -> Option<CacheRecord> {
    let bytes = tokio::fs::read(path).await.ok()?;
    serde_json::from_slice(&bytes).ok()
}

async fn write_cache(path: &Path, record: &CacheRecord) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(record)?;
    tokio::fs::write(path, bytes).await?;
    Ok(())
}

fn now_iso8601() -> String {
    // Matches `new Date().toISOString()` in Node (YYYY-MM-DDTHH:MM:SS.sssZ).
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);
    let secs = ts_millis / 1000;
    let millis = ts_millis % 1000;
    let tm = chrono::DateTime::<chrono::Utc>::from_timestamp(secs, (millis as u32) * 1_000_000)
        .unwrap_or_else(chrono::Utc::now);
    tm.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

#[cfg(test)]
mod cache_key_tests {
    //! These tests pin the cache-key construction byte-for-byte against
    //! Node's behavior in `src/sdk-doc-generators/openai-client.js`.
    //!
    //! Regenerate the expected hashes with the snippet at the top of
    //! `client.rs`'s `cache_key` docstring if Node's `generateCacheKey`
    //! ever changes — but treat any drift as a real incident, because
    //! it invalidates every committed cache file under
    //! `src/sdk-ai-cache/`.
    use super::*;

    fn make_client() -> LlmClient {
        let tmp = std::env::temp_dir().join("fcdocs-cache-key-tests");
        let _ = std::fs::create_dir_all(&tmp);
        LlmClient {
            cache_dir: tmp,
            api_key: None,
            model: "gpt-5-mini".to_string(),
            language: "typescript".to_string(),
            http: reqwest::Client::new(),
            fallback_models: vec!["gpt-5-mini".to_string()],
            endpoint_url: chat_completions_url(),
        }
    }

    #[test]
    fn matches_node_when_http_method_present() {
        let client = make_client();
        let method = json!({
            "name": "addDomainConfig",
            "parameters": {"tenantId": {"type": "string", "required": true}},
            "responseType": "DomainConfig",
            "nestedTypes": {},
            "httpMethod": "POST",
            "path": "/api/v1/domain-config",
        });
        assert_eq!(
            client.cache_key(&method, "TEST PROMPT"),
            "5febcc618c0d8ef55592f33ef320532a0554f336a98325e83b30e3664b05b311"
        );
    }

    #[test]
    fn matches_node_when_http_method_absent() {
        // This is the regression: Method has #[serde(skip_serializing_if =
        // "Option::is_none")] on http_method, so to_value omits the key.
        // Node's JSON.stringify drops undefined; ours must too.
        let client = make_client();
        let method = json!({
            "name": "addDomainConfig",
            "parameters": {"tenantId": {"type": "string", "required": true}},
            "responseType": "DomainConfig",
            "nestedTypes": {},
            // no httpMethod, no path
        });
        assert_eq!(
            client.cache_key(&method, "TEST PROMPT"),
            "c6faa321a52fc23cd8abc9373c1079e7e3f16fb8a662da5a1192f7a3b2af1ae6"
        );
    }

    #[test]
    fn matches_node_when_http_method_is_explicit_null() {
        // Defensive case: if some upstream emits an explicit null
        // (rather than skip-serializing), it should hash differently
        // from `absent` and match Node's behavior with `httpMethod:
        // null` left in the object.
        let client = make_client();
        let method = json!({
            "name": "addDomainConfig",
            "parameters": {"tenantId": {"type": "string", "required": true}},
            "responseType": "DomainConfig",
            "nestedTypes": {},
            "httpMethod": null,
            "path": null,
        });
        assert_eq!(
            client.cache_key(&method, "TEST PROMPT"),
            "f78cedb63356648c100ea7de881e1ffb5a63296cc1df267c82c8502aea590276"
        );
    }

    #[test]
    fn absent_and_null_hash_differently() {
        let client = make_client();
        let absent = json!({
            "name": "x", "parameters": {}, "responseType": "T", "nestedTypes": {}
        });
        let null_present = json!({
            "name": "x", "parameters": {}, "responseType": "T", "nestedTypes": {},
            "httpMethod": null, "path": null
        });
        assert_ne!(
            client.cache_key(&absent, "p"),
            client.cache_key(&null_present, "p"),
            "absent must hash differently from explicit-null (matches Node)"
        );
    }

    #[test]
    fn system_message_per_language() {
        let mut c = make_client();
        c.language = "typescript".into();
        assert!(c.system_message().starts_with("You are an expert TypeScript"));
        c.language = "rust".into();
        assert!(c.system_message().starts_with("You are an expert Rust"));
        c.language = "cpp".into();
        assert!(c.system_message().starts_with("You are an expert C++"));
        c.language = "nim".into();
        assert!(c.system_message().starts_with("You are an expert Nim"));
        // Unknown language falls back to TypeScript persona (matches
        // Node's else branch at openai-client.js:125).
        c.language = "haskell".into();
        assert!(c.system_message().starts_with("You are an expert TypeScript"));
    }

    #[test]
    fn strip_code_fences_handles_node_variants() {
        // Mirrors src/sdk-doc-generators/openai-client.js:411-416.
        // Each prefix appears at start, paired with the closing ```.
        assert_eq!(strip_code_fences("```typescript\nfoo\n```"), "foo");
        assert_eq!(strip_code_fences("```ts\nfoo\n```"), "foo");
        assert_eq!(strip_code_fences("```rust\nlet x = 1;\n```"), "let x = 1;");
        assert_eq!(strip_code_fences("```\nbar\n```"), "bar");
        // .trim() up-front mirrors Node's codeExample.trim().
        assert_eq!(strip_code_fences("  \n```\nbar\n```\n  "), "bar");
        // No fences: returned as-is (after trim).
        assert_eq!(strip_code_fences("naked code"), "naked code");
        // Only the leading-most fence is stripped; embedded fences in
        // mid-snippet are left alone.
        assert_eq!(
            strip_code_fences("```typescript\nlet a = `\\`\\``;\n```"),
            "let a = `\\`\\``;"
        );
    }
}

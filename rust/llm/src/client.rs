//! OpenAI chat-completions client with disk-backed cache + model fallback.
//!
//! Mirrors `src/sdk-doc-generators/openai-client.js`:
//! - Cache directory laid out the same way (`<cache_dir>/<sha256>.json`).
//! - Cache file body matches the Node shape `{ method, codeExample, timestamp, model }`.
//! - Retries with model fallback for oversized requests (`gpt-5-mini` ->
//!   `gpt-4.1` -> `gpt-5.2`), matching the existing behavior.

use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use thiserror::Error;

use crate::cache_key::sha256_hex;

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("OpenAI API returned status {status}: {body}")]
    Api { status: u16, body: String },
    #[error("OpenAI response missing choices/message/content")]
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
}

impl LlmClient {
    pub fn new(
        cache_dir: impl Into<PathBuf>,
        model: impl Into<String>,
        language: impl Into<String>,
    ) -> Result<Self> {
        let cache_dir = cache_dir.into();
        std::fs::create_dir_all(&cache_dir)
            .with_context(|| format!("create cache dir {cache_dir:?}"))?;
        let model = model.into();
        let language = language.into();
        let fallback_models = default_fallbacks(&model);
        Ok(Self {
            cache_dir,
            api_key: std::env::var("OPENAI_API_KEY").ok(),
            model,
            language,
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(600))
                .build()
                .expect("reqwest client"),
            fallback_models,
        })
    }

    /// Compute the cache key for a (method, prompt, model) tuple,
    /// matching Node's `openai-client.js:58-72`.
    pub fn cache_key(&self, method_meta: &Value, prompt: &str) -> String {
        let key_value = json!({
            "methodName": method_meta.get("name"),
            "parameters": method_meta.get("parameters"),
            "responseType": method_meta.get("responseType"),
            "nestedTypes": method_meta.get("nestedTypes"),
            "httpMethod": method_meta.get("httpMethod"),
            "path": method_meta.get("path"),
            "prompt": prompt,
            "model": self.model.clone(),
        });
        sha256_hex(&key_value)
    }

    /// Cache-only lookup. Returns `Some(code_example)` if the cache
    /// file exists, else `None`. Use this when an API key isn't
    /// available (parity validation, CI, etc.) to surface only the
    /// cache hits.
    pub fn get_cached(&self, method_meta: &Value, prompt: &str) -> Option<String> {
        let key = self.cache_key(method_meta, prompt);
        let path = self.cache_dir.join(format!("{key}.json"));
        read_cache(&path).map(|c| c.code_example)
    }

    /// Generate a completion, with cache lookup keyed by
    /// SHA256(stable_stringify({ method, prompt, model })).
    /// `method_meta` should mirror the Node call site at
    /// `src/sdk-doc-generators/openai-client.js:58-72`.
    pub async fn generate(&self, method_meta: &Value, prompt: &str) -> Result<LlmResponse> {
        let cache_key = self.cache_key(method_meta, prompt);
        let cache_file = self.cache_dir.join(format!("{cache_key}.json"));

        if let Some(cached) = read_cache(&cache_file) {
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
            .context("OPENAI_API_KEY not set")?;

        let mut last_err: Option<anyhow::Error> = None;
        for model in &self.fallback_models {
            match self.call_openai(api_key, model, prompt).await {
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
                    );
                    return Ok(LlmResponse {
                        text,
                        model_used: model.clone(),
                        cached: false,
                    });
                }
                Err(e) => {
                    tracing::warn!(model = %model, error = %e, "OpenAI call failed; trying next fallback");
                    last_err = Some(e);
                }
            }
        }
        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("no models attempted")))
    }

    async fn call_openai(&self, api_key: &str, model: &str, prompt: &str) -> Result<String> {
        let body = json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "max_completion_tokens": 4000,
        });
        let resp = self
            .http
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(api_key)
            .json(&body)
            .send()
            .await
            .context("OpenAI POST")?;
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(LlmError::Api {
                status: status.as_u16(),
                body: text,
            }
            .into());
        }
        let value: Value = resp.json().await.context("parse OpenAI response")?;
        let text = value
            .pointer("/choices/0/message/content")
            .and_then(|v| v.as_str())
            .ok_or(LlmError::BadResponse)?
            .to_string();
        Ok(text)
    }
}

fn default_fallbacks(primary: &str) -> Vec<String> {
    // Mirrors the Node fallback chain in translate-with-gpt.js:66 and
    // similar use in openai-client.js: prefer the configured model, fall
    // back to gpt-4.1 then gpt-5.2 for oversized requests.
    let mut out = vec![primary.to_string()];
    for m in ["gpt-4.1", "gpt-5.2"] {
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

fn read_cache(path: &Path) -> Option<CacheRecord> {
    let bytes = std::fs::read(path).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn write_cache(path: &Path, record: &CacheRecord) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(record)?;
    std::fs::write(path, bytes)?;
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

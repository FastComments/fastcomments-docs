//! LLM client for paths that expect a JSON response: the UI
//! strings translator (translates a dict, expects a dict back) and
//! the meta.json translator (same shape). The "Json" in the name is
//! load-bearing: this client strips ```json fences and
//! `serde_json::from_str`s the body before returning.
//!
//! The retry/backoff/model-fallback loop lives in
//! [`crate::llm_client::CompletionClient`] — shared with the
//! markdown items translator in `run.rs` so all three paths agree
//! on transient-failure handling. This module is just the
//! JSON-response post-processing on top.

use std::sync::Arc;

use anyhow::{Context, Result};
use serde_json::Value;
use tracing::info;

use crate::llm_client::CompletionClient;

/// Wraps a [`CompletionClient`] with JSON-response post-processing
/// (fence stripping + parse). Cheap to clone (the underlying
/// CompletionClient is Arc-backed).
#[derive(Clone)]
pub struct JsonTranslator {
    pub client: Arc<reqwest::Client>,
    pub api_key: Arc<String>,
    pub primary_model: String,
}

impl JsonTranslator {
    fn completion_client(&self) -> CompletionClient {
        CompletionClient::new(
            self.client.clone(),
            self.api_key.clone(),
            self.primary_model.clone(),
        )
    }

    /// Translate one prompt (system + user) and return the parsed JSON
    /// object. Uses the shared retry+fallback loop, then strips
    /// fences and parses. `label` is a debug tag for log lines (e.g.
    /// `"UI strings for fr_fr"` or `"meta.json for guide-sso/fr_fr"`).
    pub async fn translate_to_json(
        &self,
        system: &str,
        prompt: &str,
        label: &str,
    ) -> Result<serde_json::Map<String, Value>> {
        // Parse INSIDE the retry loop: a 200 response carrying malformed
        // or truncated JSON is a retryable failure (retry same model,
        // then fall back), not a hard abort. Previously this parse ran
        // after the retry loop returned, so one flaky completion failed
        // the whole build.
        let (obj, model_used) = self
            .completion_client()
            .complete_validated(system, prompt, label, |text| {
                let cleaned = strip_json_fences(text);
                let parsed: Value = serde_json::from_str(&cleaned).with_context(|| {
                    format!(
                        "parse LLM response as JSON for {label} (first 200 chars: {first}...)",
                        first = cleaned.chars().take(200).collect::<String>()
                    )
                })?;
                let obj = parsed.as_object().ok_or_else(|| {
                    anyhow::anyhow!("LLM response for {label} was not a JSON object")
                })?;
                Ok(obj.clone())
            })
            .await?;
        info!(model = %model_used, %label, "translated");
        Ok(obj)
    }
}

/// Mirrors Node's strip-markdown-code-block logic at
/// the legacy Node translator:499-508 / :722-731:
///   if startsWith ```json -> strip 7 chars
///   else if startsWith ``` -> strip 3 chars
///   if endsWith ``` -> strip 3 chars
///   then trim
pub fn strip_json_fences(s: &str) -> String {
    let trimmed = s.trim();
    let mut body = trimmed;
    if let Some(rest) = body.strip_prefix("```json") {
        body = rest;
    } else if let Some(rest) = body.strip_prefix("```") {
        body = rest;
    }
    if let Some(rest) = body.strip_suffix("```") {
        body = rest;
    }
    body.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_fences_handles_node_variants() {
        assert_eq!(strip_json_fences("```json\n{\"a\":1}\n```"), "{\"a\":1}");
        assert_eq!(strip_json_fences("```\n{\"a\":1}\n```"), "{\"a\":1}");
        assert_eq!(strip_json_fences("{\"a\":1}"), "{\"a\":1}");
        // Whitespace tolerance.
        assert_eq!(strip_json_fences("  ```json\n{\"a\":1}\n```  "), "{\"a\":1}");
        // Only one fence shape stripped — don't accidentally strip ` from data.
        assert_eq!(strip_json_fences("`backtick in value`"), "`backtick in value`");
    }
}

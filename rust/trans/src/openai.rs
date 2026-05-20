//! Shared OpenAI chat/completions client used by the UI + meta.json
//! translation paths. Mirrors the request shape + retry/backoff + model
//! fallback chain at `src/translate-with-gpt.js:451-573` and :659-810.
//!
//! Output is parsed as a JSON object (Node strips ```json / ```
//! fences then `JSON.parse`s). The markdown items path in
//! `run.rs::call_openai` doesn't go through here — it returns raw
//! markdown text, not JSON — but it could be unified in a follow-up.

use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use serde_json::{json, Value};
use tracing::{info, warn};

/// Node's translate-with-gpt.js ALTERNATE_MODELS at line 66. Tried in
/// order after the primary model if length truncation hits.
const ALTERNATE_MODELS: &[&str] = &["gpt-4.1", "gpt-5.2"];
/// Node maxRetries at translate-with-gpt.js:465 / :688.
const MAX_RETRIES: u32 = 5;
const BASE_DELAY: Duration = Duration::from_millis(1000);
/// Node max_completion_tokens at translate-with-gpt.js:486 / :709.
const MAX_COMPLETION_TOKENS: u32 = 16000;

/// Shared OpenAI JSON-translation client. Wraps the reqwest client +
/// API key + primary model + the same fallback chain Node uses.
#[derive(Clone)]
pub struct JsonTranslator {
    pub client: Arc<reqwest::Client>,
    pub api_key: Arc<String>,
    pub primary_model: String,
}

impl JsonTranslator {
    /// Translate one prompt (system + user) and return the parsed JSON
    /// object. Implements the Node retry/backoff loop with model
    /// fallback on length-truncation, code-fence stripping, and JSON
    /// parsing. `label` is a debug tag for log lines (e.g.
    /// `"UI strings for fr_fr"` or `"meta.json for guide-sso/fr_fr"`).
    pub async fn translate_to_json(
        &self,
        system: &str,
        prompt: &str,
        label: &str,
    ) -> Result<serde_json::Map<String, Value>> {
        // Node builds modelsToTry = [primary, ...ALTERNATE_MODELS].
        let models: Vec<String> = std::iter::once(self.primary_model.clone())
            .chain(ALTERNATE_MODELS.iter().map(|m| (*m).to_string()))
            .collect();

        let mut last_err: Option<anyhow::Error> = None;
        for current_model in &models {
            for attempt in 1..=MAX_RETRIES {
                match self
                    .single_call(current_model, system, prompt, label)
                    .await
                {
                    Ok(CallResult::Ok(obj)) => return Ok(obj),
                    Ok(CallResult::LengthTruncated) => {
                        // Don't retry the same model — move to the next
                        // fallback. Matches Node's "break inner loop on
                        // length" behavior at line 516/736.
                        warn!(model = %current_model, %label, "OpenAI length truncation, trying next fallback");
                        break;
                    }
                    Err(e) => {
                        // Always log, like Node line 548/788.
                        warn!(model = %current_model, %label, attempt, error = %e, "OpenAI call failed");
                        last_err = Some(e);
                        if attempt == MAX_RETRIES {
                            // Move to next model.
                            break;
                        }
                        let delay = BASE_DELAY * 2u32.pow(attempt - 1);
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }
        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("exhausted all models without success")))
    }

    async fn single_call(
        &self,
        model: &str,
        system: &str,
        prompt: &str,
        label: &str,
    ) -> Result<CallResult> {
        let body = json!({
            "model": model,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": prompt}
            ],
            "max_completion_tokens": MAX_COMPLETION_TOKENS,
        });
        let resp = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(self.api_key.as_str())
            .json(&body)
            .send()
            .await
            .with_context(|| format!("POST OpenAI for {label}"))?;
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("OpenAI API error {status}: {text}");
        }
        let data: Value = resp.json().await.context("parse OpenAI response")?;
        let raw_content = data
            .pointer("/choices/0/message/content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();
        let finish_reason = data
            .pointer("/choices/0/finish_reason")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if raw_content.is_empty() {
            if finish_reason == "length" {
                return Ok(CallResult::LengthTruncated);
            }
            anyhow::bail!(
                "OpenAI returned empty content (finish_reason={finish_reason})"
            );
        }

        let cleaned = strip_json_fences(&raw_content);
        let parsed: Value = serde_json::from_str(&cleaned).with_context(|| {
            format!(
                "parse OpenAI response as JSON for {label} (first 200 chars: {first}...)",
                first = cleaned.chars().take(200).collect::<String>()
            )
        })?;
        let obj = parsed.as_object().ok_or_else(|| {
            anyhow::anyhow!("OpenAI response for {label} was not a JSON object")
        })?;
        info!(model = %model, %label, "translated");
        Ok(CallResult::Ok(obj.clone()))
    }
}

enum CallResult {
    Ok(serde_json::Map<String, Value>),
    LengthTruncated,
}

/// Mirrors Node's strip-markdown-code-block logic at
/// translate-with-gpt.js:499-508 / :722-731:
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

use std::time::Duration;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// HTTP client for the Node content sidecar (`src/content-sidecar.js`).
///
/// The sidecar owns two pieces of behavior we cannot easily reproduce in Rust:
///
/// - **highlight.js** syntax highlighting (matching the existing build output).
/// - **`vm.runInContext`** parsing of the JS object literals embedded in marker
///   tags like `[inline-code-attrs-start ... inline-code-attrs-end]`.
#[derive(Clone)]
pub struct SidecarClient {
    base_url: String,
    http: reqwest::Client,
}

#[derive(Debug, Serialize)]
pub struct HighlightRequest<'a> {
    pub code: &'a str,
    pub lang: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
pub struct HighlightResponse {
    pub html: String,
    #[serde(default)]
    pub language: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EvalMarkerRequest<'a> {
    pub kind: MarkerKind,
    pub config_source: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum MarkerKind {
    InlineCode,
    CodeExample,
    ApiResourceHeader,
    RelatedParameter,
}

impl SidecarClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("reqwest client");
        Self {
            base_url: base_url.into(),
            http,
        }
    }

    pub async fn highlight(&self, code: &str, lang: Option<&str>) -> Result<HighlightResponse> {
        let url = format!("{}/highlight", self.base_url);
        let resp = self
            .http
            .post(&url)
            .json(&HighlightRequest { code, lang })
            .send()
            .await
            .with_context(|| format!("POST {url}"))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("sidecar /highlight returned {status}: {body}");
        }
        Ok(resp.json().await?)
    }

    pub async fn eval_marker(
        &self,
        kind: MarkerKind,
        config_source: &str,
    ) -> Result<serde_json::Value> {
        let url = format!("{}/eval-marker", self.base_url);
        let resp = self
            .http
            .post(&url)
            .json(&EvalMarkerRequest { kind, config_source })
            .send()
            .await
            .with_context(|| format!("POST {url}"))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("sidecar /eval-marker returned {status}: {body}");
        }
        Ok(resp.json().await?)
    }

    pub async fn health(&self) -> Result<()> {
        let url = format!("{}/health", self.base_url);
        let resp = self.http.get(&url).send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("sidecar /health returned {}", resp.status());
        }
        Ok(())
    }
}

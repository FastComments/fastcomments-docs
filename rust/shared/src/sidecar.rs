use std::time::Duration;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// HTTP client for the Node content sidecar (`src/content-sidecar.js`).
///
/// The sidecar now owns exactly one piece of behavior:
///
/// - **highlight.js** syntax highlighting (matching the existing build
///   output byte-for-byte).
///
/// Previously also hosted `/eval-marker` for `vm.runInContext` — that's
/// now in-process Rust via QuickJS (see `markers::qjs`). The
/// `MarkerKind` enum stays here as a free-standing type because both
/// QuickJS and the (removed) sidecar path key off it.
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

/// Identifier for which marker shape a config block is. Used by
/// `markers::qjs::eval_marker_sync` to apply per-kind preset logic
/// (currently only `InlineCode` pre-sets a `globals = {}` so scripts
/// can reference `globals.foo`).
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

    pub async fn health(&self) -> Result<()> {
        let url = format!("{}/health", self.base_url);
        let resp = self.http.get(&url).send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("sidecar /health returned {}", resp.status());
        }
        Ok(())
    }
}

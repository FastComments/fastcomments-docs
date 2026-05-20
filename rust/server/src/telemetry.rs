//! Batch flush of search query telemetry to the FastComments API.
//!
//! Mirrors the 10-second interval in `src/server-search-engine.js:163-194`,
//! including the prefix-search filter (`filterPrefixSearches`) and the
//! `e2e-test` skip.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::{AppState, TELEMETRY_FLUSH_SECS};
use tracing::{error, info};

/// Per-tenant set of pending queries, keyed by query string -> recorded-at.
type TenantQueries = HashMap<String, Instant>;

#[derive(Default)]
pub struct Collector {
    pub by_tenant: HashMap<String, TenantQueries>,
}

impl Collector {
    pub fn record(&mut self, tenant_id: String, query: String) {
        self.by_tenant
            .entry(tenant_id)
            .or_default()
            .insert(query, Instant::now());
    }

    fn drain(&mut self) -> HashMap<String, TenantQueries> {
        std::mem::take(&mut self.by_tenant)
    }
}

pub async fn flush_loop(state: AppState) {
    let mut ticker = tokio::time::interval(Duration::from_secs(TELEMETRY_FLUSH_SECS));
    ticker.tick().await; // skip first immediate tick
    loop {
        ticker.tick().await;
        let snapshot = {
            let mut t = state.telemetry.lock().await;
            t.drain()
        };
        if snapshot.is_empty() {
            continue;
        }
        let Some(api_key) = state.search_api_key.as_ref() else {
            // No key configured — quietly drop, matching the prod behavior
            // when SEARCH_API_KEY is unset (the existing Node code would
            // build a URL with `undefined` and fail).
            continue;
        };
        let api_key = api_key.clone();
        let client = reqwest::Client::new();
        for (tenant_id, queries) in snapshot {
            for input in filter_prefix_searches(queries.keys()) {
                if input.contains("e2e-test") {
                    info!(input = %input, "skip e2e test telemetry");
                    continue;
                }
                let url = format!(
                    "https://fastcomments.com/docs-search/track-search-event?API_KEY={api}&tenantId={tid}&searchInput={inp}",
                    api = urlencoding::encode(&api_key),
                    tid = urlencoding::encode(&tenant_id),
                    inp = urlencoding::encode(&input),
                );
                match client.post(&url).send().await {
                    Ok(r) => {
                        info!(status = ?r.status(), input = %input, "tracked");
                    }
                    Err(e) => {
                        error!(error = %e, input = %input, "telemetry post failed");
                    }
                }
            }
        }
    }
}

/// Mirrors `filterPrefixSearches` in `src/server-search-engine.js:142-160`.
/// Drops any query that is a strict prefix of another query in the same
/// batch (so we only record the "final" query the user typed, not the
/// intermediate keystrokes).
fn filter_prefix_searches<'a, I>(inputs: I) -> Vec<String>
where
    I: IntoIterator<Item = &'a String>,
{
    let arr: Vec<&String> = inputs.into_iter().collect();
    let mut out: Vec<String> = Vec::new();
    for (i, a) in arr.iter().enumerate() {
        let mut is_prefix = false;
        for (j, b) in arr.iter().enumerate() {
            if i != j && b.starts_with(a.as_str()) && *b != *a {
                is_prefix = true;
                break;
            }
        }
        if !is_prefix {
            out.push((*a).clone());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_filter_drops_intermediate_keystrokes() {
        let v: Vec<String> = vec!["foo".into(), "foob".into(), "foobar".into(), "bar".into()];
        let mut out = filter_prefix_searches(v.iter());
        out.sort();
        assert_eq!(out, vec!["bar".to_string(), "foobar".to_string()]);
    }
}

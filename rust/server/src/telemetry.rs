//! Batch flush of search query telemetry to the FastComments API.
//!
//! Mirrors the 10-second interval in `src/server-search-engine.js:163-194`,
//! including the prefix-search filter (`filterPrefixSearches`) and the
//! `e2e-test` skip.
//!
//! Hardening (vs the original port):
//!
//! * Each tenant slot is capped at `MAX_QUERIES_PER_TENANT`, and the
//!   global map at `MAX_TENANTS`. Beyond the cap, new entries are dropped
//!   so an attacker can't grow this map without bound.
//! * Per-query length is truncated to `MAX_RECORDED_LEN`.
//! * Flush uses the shared `AppState.http` client (timeouts inherited).
//! * The API key is sent in an `X-API-Key` header instead of the URL, so
//!   it doesn't end up in upstream access logs / metrics pipelines.

use std::collections::{BTreeSet, HashMap};
use std::time::{Duration, Instant};

use crate::{AppState, TELEMETRY_FLUSH_SECS};
use tracing::{error, info, warn};

/// Hard cap on queries recorded per tenant per flush window. Above this,
/// new queries are dropped (silently — telemetry is not load-bearing).
const MAX_QUERIES_PER_TENANT: usize = 1_000;
/// Hard cap on total tenants. Above this, new tenants are dropped.
const MAX_TENANTS: usize = 5_000;
/// Truncation applied to each recorded query (defense in depth — the
/// route already enforces `MAX_QUERY_LEN`).
const MAX_RECORDED_LEN: usize = 256;

/// Per-tenant set of pending queries, keyed by query string -> recorded-at.
type TenantQueries = HashMap<String, Instant>;

#[derive(Default)]
pub struct Collector {
    pub by_tenant: HashMap<String, TenantQueries>,
}

impl Collector {
    pub fn record(&mut self, tenant_id: String, query: String) {
        if !self.by_tenant.contains_key(&tenant_id) && self.by_tenant.len() >= MAX_TENANTS {
            return;
        }
        let entry = self.by_tenant.entry(tenant_id).or_default();
        if entry.len() >= MAX_QUERIES_PER_TENANT && !entry.contains_key(&query) {
            return;
        }
        let q = if query.len() > MAX_RECORDED_LEN {
            // Slice on a char boundary so we never panic on UTF-8 input.
            let mut end = MAX_RECORDED_LEN;
            while end > 0 && !query.is_char_boundary(end) {
                end -= 1;
            }
            query[..end].to_string()
        } else {
            query
        };
        entry.insert(q, Instant::now());
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
        for (tenant_id, queries) in snapshot {
            let inputs = filter_prefix_searches(queries.keys().cloned());
            for input in inputs {
                if input.contains("e2e-test") {
                    info!(input = %input, "skip e2e test telemetry");
                    continue;
                }
                let url = format!(
                    "https://fastcomments.com/docs-search/track-search-event?tenantId={tid}&searchInput={inp}",
                    tid = urlencoding::encode(&tenant_id),
                    inp = urlencoding::encode(&input),
                );
                let res = state
                    .http
                    .post(&url)
                    .header("X-API-Key", api_key.as_str())
                    .send()
                    .await;
                match res {
                    Ok(r) if r.status().is_success() => {
                        info!(status = ?r.status(), input = %input, "tracked");
                    }
                    Ok(r) => {
                        warn!(status = ?r.status(), input = %input, "telemetry non-2xx");
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
///
/// Old impl was O(n²). Sort the inputs lexicographically and a string is
/// a strict prefix of another only if its successor in the sorted list
/// starts with it. That collapses to O(n log n).
fn filter_prefix_searches<I>(inputs: I) -> Vec<String>
where
    I: IntoIterator<Item = String>,
{
    let sorted: BTreeSet<String> = inputs.into_iter().collect();
    let arr: Vec<String> = sorted.into_iter().collect();
    let mut out = Vec::with_capacity(arr.len());
    for (i, a) in arr.iter().enumerate() {
        let drops = arr
            .get(i + 1)
            .map(|next| next.starts_with(a.as_str()) && next != a)
            .unwrap_or(false);
        if !drops {
            out.push(a.clone());
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
        let mut out = filter_prefix_searches(v.into_iter());
        out.sort();
        assert_eq!(out, vec!["bar".to_string(), "foobar".to_string()]);
    }

    #[test]
    fn record_drops_beyond_tenant_cap() {
        let mut c = Collector::default();
        for i in 0..(MAX_QUERIES_PER_TENANT + 50) {
            c.record("t".to_string(), format!("q{i}"));
        }
        assert_eq!(c.by_tenant.get("t").unwrap().len(), MAX_QUERIES_PER_TENANT);
    }

    #[test]
    fn record_truncates_oversized_query() {
        let mut c = Collector::default();
        let long = "a".repeat(MAX_RECORDED_LEN + 100);
        c.record("t".to_string(), long);
        let only = c.by_tenant.get("t").unwrap().keys().next().unwrap();
        assert_eq!(only.len(), MAX_RECORDED_LEN);
    }
}

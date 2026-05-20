//! Shared helpers for the per-language AI doc generators.

use std::path::PathBuf;

use anyhow::{Context, Result};
use serde_json::Value;

/// Read and parse an OpenAPI spec from disk. Mirrors `parseOpenAPISpec`
/// in `typescript-ai-generator.js:174-189`.
pub fn parse_openapi_spec(path: &std::path::Path) -> Result<Value> {
    if !path.exists() {
        anyhow::bail!("OpenAPI spec not found: {path:?}");
    }
    let bytes = std::fs::read(path).with_context(|| format!("read {path:?}"))?;
    Ok(serde_json::from_slice(&bytes).with_context(|| format!("parse {path:?}"))?)
}

/// Operation info keyed by operationId. Mirrors `buildOperationMap`
/// in `typescript-ai-generator.js:196-223`.
pub struct OperationInfo {
    pub method: String,
    pub path: String,
    pub tag: Option<String>,
    pub description: Option<String>,
}

const HTTP_METHODS: &[&str] = &["get", "post", "put", "patch", "delete"];

pub fn build_operation_map(spec: &Value) -> indexmap::IndexMap<String, OperationInfo> {
    let mut out = indexmap::IndexMap::new();
    let Some(paths) = spec.get("paths").and_then(|v| v.as_object()) else {
        return out;
    };
    for (path_str, path_item) in paths {
        let Some(item) = path_item.as_object() else { continue };
        for (method, op) in item {
            if !HTTP_METHODS.contains(&method.as_str()) {
                continue;
            }
            let tag = op
                .get("tags")
                .and_then(|v| v.as_array())
                .and_then(|a| a.first())
                .and_then(|t| t.as_str())
                .map(String::from);
            if tag.as_deref() == Some("Hidden") {
                continue;
            }
            let Some(op_id) = op.get("operationId").and_then(|v| v.as_str()) else {
                continue;
            };
            let description = op
                .get("summary")
                .or_else(|| op.get("description"))
                .and_then(|v| v.as_str())
                .map(String::from);
            out.insert(
                op_id.to_string(),
                OperationInfo {
                    method: method.clone(),
                    path: path_str.clone(),
                    tag,
                    description,
                },
            );
        }
    }
    out
}

/// Default `sdk-ai-cache/<sdk-id>/` location for cache files. Mirrors
/// the path constructed at `typescript-ai-generator.js:32`.
pub fn ai_cache_dir(repo_root: &std::path::Path, sdk_id: &str) -> PathBuf {
    repo_root.join("src/sdk-ai-cache").join(sdk_id)
}

/// Sanitize a method name into a filename component. Mirrors
/// `sanitizeFilename` in `base-generator.js`. Used both for marker
/// filenames and for code-snippet IDs.
pub fn sanitize_filename(name: &str) -> String {
    let lower = name.to_lowercase();
    let re = regex::Regex::new(r"[^a-z0-9]+").expect("regex");
    re.replace_all(&lower, "-")
        .trim_matches('-')
        .to_string()
}

/// Find the repo root from a sub-path.
pub fn repo_root_from(p: &std::path::Path) -> Option<PathBuf> {
    let mut cur = p;
    loop {
        if cur.join("src").join("locales.json").exists() {
            return Some(cur.to_path_buf());
        }
        cur = cur.parent()?;
    }
}

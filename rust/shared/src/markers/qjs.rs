//! QuickJS-backed marker config evaluator. Replaces the Node sidecar's
//! `/eval-marker` endpoint.
//!
//! Marker tags in source markdown embed real JS object-literal config blocks
//! between start/end tokens (e.g.
//! `[inline-code-attrs-start title = 'x'; type = 'js'; inline-code-attrs-end]`).
//! The original Node implementation used `vm.createContext(args)` +
//! `vm.runInContext(code, args)`. With V8's semantics, bare assignments
//! (`title = 'x'`) become properties on the context object (treated as the
//! global).
//!
//! We reproduce that by: snapshotting the globals before eval, running the
//! source in a fresh QuickJS context, then diffing the resulting globals
//! against the snapshot and serializing the additions as JSON. Inline-code
//! markers pre-set `globals = {}` (and then drop it from the result),
//! mirroring `src/inline-code-generator.js:73-82`.

use std::collections::HashSet;

use anyhow::{Context as _, Result};
use rquickjs::context::EvalOptions;
use rquickjs::{Context, Object, Runtime, Value};
use serde_json::Value as JsonValue;

use crate::sidecar::MarkerKind;

/// Evaluate a marker config block and return the resulting JSON object —
/// the same shape the Node sidecar's `/eval-marker` returned.
pub fn eval_marker_sync(kind: MarkerKind, config_source: &str) -> Result<JsonValue> {
    let rt = Runtime::new().context("create QuickJS runtime")?;
    let ctx = Context::full(&rt).context("create QuickJS context")?;
    ctx.with(|ctx| -> Result<JsonValue> {
        let globals = ctx.globals();

        // Snapshot the set of preexisting global property names so we can
        // diff against them after evaluation. QuickJS preloads things like
        // Object, Array, Math, JSON, etc. — we want only the script's
        // additions.
        let before: HashSet<String> = collect_own_keys(&globals)?;

        // Inline-code markers pre-set `globals = {}` (see
        // src/inline-code-generator.js:73-74).
        if matches!(kind, MarkerKind::InlineCode) {
            let empty = Object::new(ctx.clone()).context("alloc inline-code globals")?;
            globals
                .set("globals", empty)
                .context("set globals = {} for inline-code")?;
        }

        // Run the script. Use non-strict global eval so bare assignments
        // become implicit globals, matching V8's `vm.runInContext` behavior.
        // (rquickjs's default EvalOptions is strict-mode, which would
        // reject `title = 'x'` with a ReferenceError.)
        let mut opts = EvalOptions::default();
        opts.global = true;
        opts.strict = false;
        if let Err(e) = ctx.eval_with_options::<(), _>(config_source, opts) {
            let detail = match &e {
                rquickjs::Error::Exception => {
                    let exc = ctx.catch();
                    format!("{e}: {exc:?}")
                }
                other => format!("{other}"),
            };
            anyhow::bail!("marker eval failed: {detail}\n--- source ---\n{config_source}");
        }

        // Now compute the diff and serialize.
        let after: HashSet<String> = collect_own_keys(&globals)?;
        let mut out = serde_json::Map::new();
        for key in after.difference(&before) {
            // Strip the `globals` preset for inline-code (matches
            // `delete args.globals;` at inline-code-generator.js:82).
            if matches!(kind, MarkerKind::InlineCode) && key == "globals" {
                continue;
            }
            let v: Value = globals
                .get(key.as_str())
                .map_err(|e| anyhow::anyhow!("read global {key}: {e}"))?;
            out.insert(key.clone(), js_value_to_json(v)?);
        }
        Ok(JsonValue::Object(out))
    })
}

/// Async wrapper so callers' `.await` chains keep compiling unchanged.
/// QuickJS eval is sync but very fast (microseconds for our configs); no
/// need to spawn_blocking.
pub async fn eval_marker(
    _sidecar: &crate::sidecar::SidecarClient,
    kind: MarkerKind,
    config_source: &str,
) -> Result<JsonValue> {
    eval_marker_sync(kind, config_source)
}

fn collect_own_keys(obj: &Object<'_>) -> Result<HashSet<String>> {
    let mut out = HashSet::new();
    for prop in obj.keys::<String>() {
        let k = prop.map_err(|e| anyhow::anyhow!("enumerate keys: {e}"))?;
        out.insert(k);
    }
    Ok(out)
}

fn js_value_to_json(v: Value<'_>) -> Result<JsonValue> {
    if v.is_null() || v.is_undefined() {
        return Ok(JsonValue::Null);
    }
    if let Some(b) = v.as_bool() {
        return Ok(JsonValue::Bool(b));
    }
    if let Some(i) = v.as_int() {
        return Ok(JsonValue::from(i));
    }
    if let Some(n) = v.as_float() {
        return Ok(serde_json::Number::from_f64(n)
            .map(JsonValue::Number)
            .unwrap_or(JsonValue::Null));
    }
    if let Some(s) = v.as_string() {
        let owned = s
            .to_string()
            .map_err(|e| anyhow::anyhow!("string to_string: {e}"))?;
        return Ok(JsonValue::String(owned));
    }
    if let Some(arr) = v.as_array() {
        let mut out = Vec::with_capacity(arr.len());
        for item in arr.iter::<Value>() {
            let item = item.map_err(|e| anyhow::anyhow!("array iter: {e}"))?;
            out.push(js_value_to_json(item)?);
        }
        return Ok(JsonValue::Array(out));
    }
    if let Some(obj) = v.as_object() {
        // Functions show up as `is_function` and we want to skip them
        // (JSON.stringify drops them in the Node impl).
        if obj.as_function().is_some() {
            return Ok(JsonValue::Null);
        }
        let mut m = serde_json::Map::new();
        for prop in obj.keys::<String>() {
            let k = prop.map_err(|e| anyhow::anyhow!("obj key iter: {e}"))?;
            let val: Value = obj
                .get(k.as_str())
                .map_err(|e| anyhow::anyhow!("obj get {k}: {e}"))?;
            m.insert(k, js_value_to_json(val)?);
        }
        return Ok(JsonValue::Object(m));
    }
    Ok(JsonValue::Null)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_code_basic() {
        let out = eval_marker_sync(
            MarkerKind::InlineCode,
            "title = 'My Example'; type = 'javascript'; isFunctional = true;",
        )
        .unwrap();
        assert_eq!(out["title"], "My Example");
        assert_eq!(out["type"], "javascript");
        assert_eq!(out["isFunctional"], true);
        // The preset `globals = {}` must be dropped from the output
        assert!(out.get("globals").is_none());
    }

    #[test]
    fn inline_code_with_globals_assignment() {
        // Scripts may reference `globals.foo = bar` against the preset
        // object. The preset itself is dropped from output, but any
        // *new* properties on globals stay (no — Node deletes args.globals
        // entirely). We mirror Node: globals is dropped entirely.
        let out = eval_marker_sync(
            MarkerKind::InlineCode,
            "title = 't'; type = 'js'; globals.foo = 'bar';",
        )
        .unwrap();
        assert!(out.get("globals").is_none());
        assert_eq!(out["title"], "t");
    }

    #[test]
    fn api_resource_header() {
        let out = eval_marker_sync(
            MarkerKind::ApiResourceHeader,
            "name = 'Get Comments'; route = 'GET /api/v1/public/comments'; creditsCost = 10;",
        )
        .unwrap();
        assert_eq!(out["name"], "Get Comments");
        assert_eq!(out["route"], "GET /api/v1/public/comments");
        assert_eq!(out["creditsCost"], 10);
    }

    #[test]
    fn code_example_with_nested_object() {
        let out = eval_marker_sync(
            MarkerKind::CodeExample,
            "title = 'Demo'; config = {urlId: 'my-page', tenantId: 'demo'};",
        )
        .unwrap();
        assert_eq!(out["title"], "Demo");
        assert_eq!(out["config"]["urlId"], "my-page");
        assert_eq!(out["config"]["tenantId"], "demo");
    }

    #[test]
    fn related_parameter() {
        let out = eval_marker_sync(
            MarkerKind::RelatedParameter,
            "name = 'absoluteAndRelativeDates'; type = 'boolean';",
        )
        .unwrap();
        assert_eq!(out["name"], "absoluteAndRelativeDates");
        assert_eq!(out["type"], "boolean");
    }

    #[test]
    fn function_values_are_dropped() {
        let out = eval_marker_sync(
            MarkerKind::CodeExample,
            "title = 'Demo'; config = {urlId: 'p', events: {onLoad: function(){}}};",
        )
        .unwrap();
        assert_eq!(out["title"], "Demo");
        assert_eq!(out["config"]["urlId"], "p");
        // The function inside events.onLoad becomes JSON null.
        assert!(out["config"]["events"]["onLoad"].is_null());
    }

    #[test]
    fn line_to_highlight_array() {
        let out = eval_marker_sync(
            MarkerKind::CodeExample,
            "title = 't'; linesToHighlight = [1, 3, 5];",
        )
        .unwrap();
        assert_eq!(out["linesToHighlight"], serde_json::json!([1, 3, 5]));
    }
}

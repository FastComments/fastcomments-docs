//! Deterministic cache key generation matching the Node
//! `sortObjectKeys` + `JSON.stringify` + `sha256` chain in
//! `src/sdk-doc-generators/the legacy Node LLM client:13-72`.

use serde_json::Value;
use sha2::{Digest, Sha256};

/// Stable JSON stringification: keys sorted lexicographically, no whitespace.
/// Matches Node `JSON.stringify(sortObjectKeys(obj))` byte-for-byte for the
/// inputs we feed it (ASCII keys, string/number/bool/null/object/array values).
pub fn stable_stringify(value: &Value) -> String {
    let mut out = String::new();
    write_value(value, &mut out);
    out
}

fn write_value(v: &Value, out: &mut String) {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => out.push_str(&n.to_string()),
        Value::String(s) => write_json_string(s, out),
        Value::Array(arr) => {
            out.push('[');
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_value(item, out);
            }
            out.push(']');
        }
        Value::Object(map) => {
            // Sort keys lexicographically to match Node's
            // `Object.keys(o).sort()` ordering (default UTF-16 code-unit
            // compare; for ASCII keys this equals byte order).
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            out.push('{');
            for (i, k) in keys.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_json_string(k, out);
                out.push(':');
                write_value(&map[*k], out);
            }
            out.push('}');
        }
    }
}

fn write_json_string(s: &str, out: &mut String) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\x08' => out.push_str("\\b"),
            '\x0c' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// SHA256 hex digest of the stable-stringified value. Matches
/// `crypto.createHash('sha256').update(stableStringify(data)).digest('hex')`
/// in the Node client.
pub fn sha256_hex(value: &Value) -> String {
    let mut hasher = Sha256::new();
    hasher.update(stable_stringify(value).as_bytes());
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(64);
    for byte in digest {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn stringify_sorts_object_keys() {
        let v = json!({"b": 1, "a": 2});
        assert_eq!(stable_stringify(&v), r#"{"a":2,"b":1}"#);
    }

    #[test]
    fn stringify_nested_objects() {
        let v = json!({"outer": {"z": 1, "a": 2}, "first": "hi"});
        assert_eq!(
            stable_stringify(&v),
            r#"{"first":"hi","outer":{"a":2,"z":1}}"#
        );
    }

    #[test]
    fn stringify_arrays_preserve_order() {
        let v = json!([3, 1, 2]);
        assert_eq!(stable_stringify(&v), "[3,1,2]");
    }

    #[test]
    fn stringify_special_chars_in_strings() {
        let v = json!({"key": "line1\nline2\t\"q\""});
        assert_eq!(stable_stringify(&v), r#"{"key":"line1\nline2\t\"q\""}"#);
    }

    #[test]
    fn sha256_matches_node_for_simple_object() {
        // Cross-checked against the Node implementation in
        // src/sdk-doc-generators/the legacy Node LLM client by running:
        //   node -e "const c=require('crypto');const s=o=>JSON.stringify(
        //     (function r(x){if(x&&typeof x==='object'&&!Array.isArray(x)){
        //       const o={};for(const k of Object.keys(x).sort())o[k]=r(x[k]);return o;}
        //       return Array.isArray(x)?x.map(r):x;})(o));
        //     console.log(c.createHash('sha256').update(s({a:1})).digest('hex'));"
        let v = json!({"a": 1});
        assert_eq!(
            sha256_hex(&v),
            "015abd7f5cc57a2dd94b7590f04ad8084273905ee33ec5cebeae62276a97f862"
        );
    }

    #[test]
    fn sha256_matches_node_for_unsorted_keys() {
        let v = json!({"b": 1, "a": 2});
        assert_eq!(
            sha256_hex(&v),
            "d3626ac30a87e6f7a6428233b3c68299976865fa5508e4267c5415c76af7a772"
        );
    }

    #[test]
    fn sha256_matches_node_for_nested_objects() {
        let v = json!({"outer": {"z": 1, "a": 2}, "first": "hi"});
        assert_eq!(
            sha256_hex(&v),
            "db2bab02991eae961b185274f1031e7e45e28eab32aa85bd31a0c6b535b20e5b"
        );
    }
}

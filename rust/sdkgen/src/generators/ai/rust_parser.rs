//! Port of `src/sdk-doc-generators/rust-parser.js` (441 LOC).
//!
//! Parses Rust API source files, extracting `pub async fn` signatures
//! and resolving param-struct fields + nested model types from
//! `client/src/models/`.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::typescript_parser::NestedType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamInfo {
    #[serde(rename = "type")]
    pub type_: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    #[serde(rename = "paramsType")]
    pub params_type: String,
    #[serde(rename = "responseType")]
    pub response_type: String,
    #[serde(serialize_with = "serialize_indexmap")]
    pub parameters: indexmap::IndexMap<String, ParamInfo>,
    #[serde(rename = "nestedTypes", serialize_with = "serialize_indexmap")]
    pub nested_types: indexmap::IndexMap<String, NestedType>,
    #[serde(rename = "httpMethod", default, skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

fn serialize_indexmap<S, K, V>(
    map: &indexmap::IndexMap<K, V>,
    ser: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    K: serde::Serialize + Eq + std::hash::Hash,
    V: serde::Serialize,
{
    use serde::ser::SerializeMap;
    let mut m = ser.serialize_map(Some(map.len()))?;
    for (k, v) in map {
        m.serialize_entry(k, v)?;
    }
    m.end()
}

pub struct RustParser {
    pub repo_path: PathBuf,
    pub models_path: PathBuf,
    type_cache: RefCell<BTreeMap<String, Option<TypeDef>>>,
}

#[derive(Debug, Clone)]
struct TypeDef {
    #[allow(dead_code)]
    name: String,
    kind: &'static str, // "struct" or "enum"
    fields: indexmap::IndexMap<String, ParamInfo>,
    summary: String,
    file_path: String,
}

impl RustParser {
    pub fn new(repo_path: PathBuf, models_path_rel: &str) -> Self {
        let models_path = repo_path.join(models_path_rel);
        Self {
            repo_path,
            models_path,
            type_cache: RefCell::new(BTreeMap::new()),
        }
    }

    pub fn extract_api_methods(&self, api_file_rel: &str) -> Vec<Method> {
        let path = self.repo_path.join(api_file_rel);
        let Ok(content) = std::fs::read_to_string(&path) else {
            tracing::warn!(path = %path.display(), "rust API file not found");
            return Vec::new();
        };
        let mut out = Vec::new();
        // Mirrors rust-parser.js:32.
        static SIG_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"pub\s+async\s+fn\s+(\w+)\s*\([^,]+,\s*params:\s*(\w+)\)\s*->\s*Result<([^,]+),\s*Error<[^>]+>>",
            )
            .expect("regex")
        });
        for cap in SIG_RE.captures_iter(&content) {
            let name = cap[1].to_string();
            let params_type = cap[2].trim().to_string();
            let mut response_type = cap[3].trim().to_string();
            if let Some(stripped) = response_type.strip_prefix("models::") {
                response_type = stripped.to_string();
            }

            let mut method = Method {
                name,
                params_type: params_type.clone(),
                response_type: response_type.clone(),
                parameters: indexmap::IndexMap::new(),
                nested_types: indexmap::IndexMap::new(),
                http_method: None,
                path: None,
                tag: None,
                auth_type: None,
                description: None,
            };
            // Mirror Node's two-function shape: parameters use
            // extractParamsStruct (pure type-based), nested types use
            // extractStruct (combined check). Using the combined-check
            // extractor for both — as we did before — diverged from
            // Node on any params struct whose fields had a
            // `#[serde(skip_serializing_if = "Option::is_none")]`
            // attribute. Different `required` flags propagate into the
            // cache key + the prompt, so the next cache miss
            // re-generates with a subtly different signature.
            if let Some(s) = self.extract_params_struct(&params_type, &content) {
                method.parameters = s.fields.clone();
                self.resolve_nested_types(&s.fields, &mut method.nested_types, 0, 3);
            }
            if let Some(td) = self.load_type_definition(&response_type) {
                method.nested_types.insert(
                    response_type.clone(),
                    NestedType {
                        summary: td.summary.clone(),
                        file_path: td.file_path.clone(),
                    },
                );
                if td.kind == "struct" {
                    self.resolve_nested_types(&td.fields, &mut method.nested_types, 0, 2);
                }
            }
            out.push(method);
        }
        out
    }

    /// Extract a method's params struct using Node's
    /// `extractParamsStruct` semantics (line 83-142 of rust-parser.js):
    /// `required` is set purely from whether the field type starts
    /// with `Option<`. Serde attributes are NOT consulted.
    ///
    /// This differs from `extract_struct` (which mirrors Node
    /// `extractStruct` at line 291-362) by NOT carrying forward
    /// `#[serde(skip_serializing_if = "Option::is_none")]` from a
    /// preceding line. Using extract_struct for parameters silently
    /// diverged from Node on any params struct whose fields were
    /// annotated, which would have produced different cache hashes
    /// and prompts on regeneration.
    fn extract_params_struct(&self, name: &str, content: &str) -> Option<StructDef> {
        let body = extract_struct_body(name, content)?;
        // Node uses /pub\s+(\w+):\s*([^,\n]+)/g — note the \n in the
        // negation, so each field is captured per-line. Mirror that
        // exactly so multi-line declarations behave the same as Node.
        static FIELD_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"pub\s+(\w+):\s*([^,\n]+)").expect("regex"));
        let mut fields: indexmap::IndexMap<String, ParamInfo> = indexmap::IndexMap::new();
        for cap in FIELD_RE.captures_iter(&body) {
            let field_name = cap[1].to_string();
            let clean = cap[2].trim().to_string();
            let type_is_option = clean.starts_with("Option<");
            let actual = if type_is_option {
                Regex::new(r"Option<(.+)>")
                    .ok()
                    .and_then(|r| r.captures(&clean))
                    .map(|c| c[1].to_string())
                    .unwrap_or_else(|| clean.clone())
            } else {
                clean
            };
            fields.insert(
                field_name,
                ParamInfo {
                    type_: actual,
                    // Pure type-based; no skip_serializing_if check.
                    required: !type_is_option,
                },
            );
        }
        Some(StructDef {
            name: name.to_string(),
            fields,
        })
    }

    fn extract_struct(&self, name: &str, content: &str) -> Option<StructDef> {
        let body = extract_struct_body(name, content)?;
        let mut fields: indexmap::IndexMap<String, ParamInfo> = indexmap::IndexMap::new();
        let mut is_optional = false;
        for line in body.split('\n') {
            if line.contains("skip_serializing_if = \"Option::is_none\"") {
                is_optional = true;
            }
            static FIELD_RE: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"pub\s+(\w+):\s*([^,]+)").expect("regex"));
            if let Some(cap) = FIELD_RE.captures(line) {
                let name = cap[1].to_string();
                let clean = cap[2].trim().to_string();
                let type_is_option = clean.starts_with("Option<");
                let actual = if type_is_option {
                    Regex::new(r"Option<(.+)>")
                        .ok()
                        .and_then(|r| r.captures(&clean))
                        .map(|c| c[1].to_string())
                        .unwrap_or(clean.clone())
                } else {
                    clean
                };
                fields.insert(
                    name,
                    ParamInfo {
                        type_: actual,
                        required: !(is_optional || type_is_option),
                    },
                );
                is_optional = false;
            }
        }
        Some(StructDef {
            name: name.to_string(),
            fields,
        })
    }

    fn resolve_nested_types(
        &self,
        fields: &indexmap::IndexMap<String, ParamInfo>,
        out: &mut indexmap::IndexMap<String, NestedType>,
        current: usize,
        max: usize,
    ) {
        if current >= max {
            return;
        }
        for info in fields.values() {
            for type_name in self.extract_type_names(&info.type_) {
                if Self::is_primitive(&type_name) || out.contains_key(&type_name) {
                    continue;
                }
                if let Some(td) = self.load_type_definition(&type_name) {
                    out.insert(
                        type_name.clone(),
                        NestedType {
                            summary: td.summary.clone(),
                            file_path: td.file_path.clone(),
                        },
                    );
                    if td.kind == "struct" {
                        self.resolve_nested_types(&td.fields, out, current + 1, max);
                    }
                }
            }
        }
    }

    fn extract_type_names(&self, expr: &str) -> Vec<String> {
        let s = expr.trim();
        // Vec<T>
        if let Some(c) = Regex::new(r"Vec<(.+)>").ok().and_then(|r| r.captures(s).map(|c| c[1].to_string())) {
            return self.extract_type_names(&c);
        }
        // Box<T>
        if let Some(c) = Regex::new(r"Box<(.+)>").ok().and_then(|r| r.captures(s).map(|c| c[1].to_string())) {
            return self.extract_type_names(&c);
        }
        // HashMap<K, V>
        if let Some(c) = Regex::new(r"std::collections::HashMap<[^,]+,\s*(.+)>")
            .ok()
            .and_then(|r| r.captures(s).map(|c| c[1].to_string()))
        {
            return self.extract_type_names(&c);
        }
        let stripped = s.strip_prefix("models::").unwrap_or(s).to_string();
        if Self::is_primitive(&stripped) {
            return Vec::new();
        }
        vec![stripped]
    }

    fn is_primitive(t: &str) -> bool {
        matches!(
            t,
            "String" | "str" | "i32" | "i64" | "u32" | "u64" | "f32" | "f64" | "bool" | "usize" | "isize"
        )
    }

    fn load_type_definition(&self, type_name: &str) -> Option<TypeDef> {
        if let Some(cached) = self.type_cache.borrow().get(type_name) {
            return cached.clone();
        }
        let file_name = Self::to_snake_case(type_name);
        let type_file = self.models_path.join(format!("{file_name}.rs"));
        if !type_file.exists() {
            self.type_cache
                .borrow_mut()
                .insert(type_name.to_string(), None);
            return None;
        }
        let Ok(content) = std::fs::read_to_string(&type_file) else {
            self.type_cache
                .borrow_mut()
                .insert(type_name.to_string(), None);
            return None;
        };
        // Enum?
        if let Some(variants) = self.extract_enum(type_name, &content) {
            let summary = format!(
                "enum: {}",
                variants
                    .iter()
                    .map(|v| format!("'{v}'"))
                    .collect::<Vec<_>>()
                    .join(" | ")
            );
            let td = TypeDef {
                name: type_name.to_string(),
                kind: "enum",
                fields: indexmap::IndexMap::new(),
                summary,
                file_path: format!("{file_name}.rs"),
            };
            self.type_cache
                .borrow_mut()
                .insert(type_name.to_string(), Some(td.clone()));
            return Some(td);
        }
        // Struct?
        if let Some(s) = self.extract_struct(type_name, &content) {
            let summary = Self::summarize_struct(&s);
            let td = TypeDef {
                name: type_name.to_string(),
                kind: "struct",
                fields: s.fields,
                summary,
                file_path: format!("{file_name}.rs"),
            };
            self.type_cache
                .borrow_mut()
                .insert(type_name.to_string(), Some(td.clone()));
            return Some(td);
        }
        self.type_cache
            .borrow_mut()
            .insert(type_name.to_string(), None);
        None
    }

    fn extract_enum(&self, name: &str, content: &str) -> Option<Vec<String>> {
        let re = Regex::new(&format!(
            r"(?s)pub\s+enum\s+{}\s*\{{([^}}]+)\}}",
            regex::escape(name)
        ))
        .ok()?;
        let cap = re.captures(content)?;
        let body = &cap[1];
        let mut variants = Vec::new();
        for line in body.split('\n') {
            if let Some(c) =
                Regex::new(r#"#\[serde\(rename\s*=\s*"([^"]+)"\)\]"#)
                    .ok()
                    .and_then(|r| r.captures(line).map(|c| c[1].to_string()))
            {
                variants.push(c);
            }
        }
        if variants.is_empty() {
            // Fallback: extract bare variant names (snake_case via to_snake_case).
            let re = Regex::new(r"(\w+)(?:\([^)]*\))?\s*,").expect("regex");
            for c in re.captures_iter(body) {
                variants.push(Self::to_snake_case(&c[1]));
            }
        }
        Some(variants)
    }

    fn summarize_struct(s: &StructDef) -> String {
        let mut parts = Vec::new();
        for (name, info) in &s.fields {
            let opt = if info.required { "" } else { "?" };
            parts.push(format!("{name}{opt}: {}", info.type_));
        }
        format!("{{ {} }}", parts.join(", "))
    }

    /// Mirrors `rust-parser.js:432-438` — CamelCase -> snake_case
    /// with a digit-insertion rule.
    pub fn to_snake_case(s: &str) -> String {
        // Step 1: insert `_` before each uppercase letter.
        let mut out = String::new();
        for c in s.chars() {
            if c.is_ascii_uppercase() {
                out.push('_');
            }
            out.push(c);
        }
        // Step 2: insert `_` before each digit run. JS `.replace(/([0-9]+)/g, '_$1')`.
        let with_digits = Regex::new(r"([0-9]+)").unwrap().replace_all(&out, "_$1").into_owned();
        let lower = with_digits.to_lowercase();
        lower.trim_start_matches('_').to_string()
    }
}

struct StructDef {
    #[allow(dead_code)]
    name: String,
    fields: indexmap::IndexMap<String, ParamInfo>,
}

/// Locate `pub struct <name> { ... }` in `content` and return the
/// brace-balanced body. Shared by both `extract_params_struct` and
/// `extract_struct` — they only differ in how they interpret the
/// body, not in how they slice it.
fn extract_struct_body<'a>(name: &str, content: &'a str) -> Option<&'a str> {
    let re = Regex::new(&format!(
        r"pub\s+struct\s+{}\s*\{{",
        regex::escape(name)
    ))
    .ok()?;
    let m = re.find(content)?;
    let start = m.end();
    let bytes = content.as_bytes();
    let mut depth = 1i32;
    let mut end = start;
    let mut i = start;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    end = i;
                    break;
                }
            }
            _ => {}
        }
        i += 1;
    }
    Some(&content[start..end])
}

/// snake_case -> camelCase with acronym handling for OpenAPI name
/// matching. Mirrors `rust-ai-generator.js:357-401`.
pub fn snake_to_camel_case(s: &str) -> String {
    const ACRONYMS: &[&str] = &["sso", "api", "url", "uri", "http", "https", "html", "json", "xml"];
    let parts: Vec<&str> = s.split('_').collect();
    let mut out = String::new();
    for (i, part) in parts.iter().enumerate() {
        let lower = part.to_lowercase();
        if lower == "id" {
            out.push_str("Id");
            continue;
        }
        if ACRONYMS.contains(&lower.as_str()) {
            out.push_str(&part.to_uppercase());
            continue;
        }
        // Compound acronym at suffix.
        let mut handled = false;
        for ac in ACRONYMS {
            if lower.ends_with(ac) && lower != *ac {
                let prefix = &lower[..lower.len() - ac.len()];
                if ACRONYMS.contains(&prefix) {
                    out.push_str(&prefix.to_uppercase());
                    let mut chars = ac.chars();
                    if let Some(first) = chars.next() {
                        out.extend(first.to_uppercase());
                        out.push_str(chars.as_str());
                    }
                    handled = true;
                    break;
                }
            }
        }
        if handled {
            continue;
        }
        if lower.ends_with("id") && lower != "id" {
            let prefix = &lower[..lower.len() - 2];
            if ACRONYMS.contains(&prefix) {
                out.push_str(&prefix.to_uppercase());
                out.push_str("Id");
                continue;
            }
        }
        if i == 0 {
            out.push_str(part);
        } else {
            let mut chars = part.chars();
            if let Some(first) = chars.next() {
                out.extend(first.to_uppercase());
                out.push_str(chars.as_str());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    //! Pins the params-vs-nested-type divergence Node has at
    //! src/sdk-doc-generators/rust-parser.js — extractParamsStruct
    //! (line 83-142) is pure type-based; extractStruct (line 291-362)
    //! also carries forward `skip_serializing_if = "Option::is_none"`
    //! from the preceding line. The Rust port used the combined
    //! extractor for BOTH, so params silently picked up the carry-
    //! forward behavior and produced different `required` flags than
    //! Node — invalidating cache hits and changing prompt content.
    use super::*;
    use std::path::PathBuf;

    fn parser() -> RustParser {
        // models_path doesn't matter for the in-memory body tests
        // below; load_type_definition is what touches disk.
        RustParser::new(PathBuf::from("/tmp"), "client/src/models/")
    }

    #[test]
    fn params_struct_ignores_serde_skip_attribute() {
        // The exact divergence shape Node + Rust differ on:
        // a `#[serde(skip_serializing_if = "Option::is_none")]` line
        // followed by a NON-Option field. Node extractParamsStruct
        // ignores serde attrs entirely -> required = true. Old Rust
        // extract_struct carried the attr forward -> required = false.
        let source = r#"
pub struct AddDomainConfigParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: String,
    pub add_domain_config_params: models::AddDomainConfigParams
}
"#;
        let s = parser()
            .extract_params_struct("AddDomainConfigParams", source)
            .expect("struct present");
        let tenant = s.fields.get("tenant_id").expect("tenant_id field");
        assert!(
            tenant.required,
            "params extractor must ignore #[serde(skip_serializing_if=…)]; \
             Node marks this field required=true (line 83-142 pure type-based)"
        );
    }

    #[test]
    fn params_struct_marks_option_fields_optional() {
        // Normal case (which Node + the old combined extractor agreed
        // on) — keep it locked so refactors don't drift back.
        let source = r#"
pub struct GetTicketsParams {
    pub tenant_id: Option<String>,
    pub status: String,
}
"#;
        let s = parser()
            .extract_params_struct("GetTicketsParams", source)
            .expect("struct present");
        assert_eq!(
            s.fields.get("tenant_id").map(|p| p.required),
            Some(false),
            "Option<T> -> required=false"
        );
        assert_eq!(
            s.fields.get("status").map(|p| p.required),
            Some(true),
            "non-Option -> required=true"
        );
    }

    #[test]
    fn params_unwraps_option_inner_type() {
        let source = r#"
pub struct P {
    pub a: Option<MyType>,
    pub b: Option<Vec<u8>>,
}
"#;
        let s = parser().extract_params_struct("P", source).unwrap();
        assert_eq!(s.fields.get("a").map(|p| p.type_.as_str()), Some("MyType"));
        assert_eq!(s.fields.get("b").map(|p| p.type_.as_str()), Some("Vec<u8>"));
    }

    /// extract_struct (used for nested types) is the one that DOES
    /// match Node's combined check at extractStruct line 291-362.
    /// Pin that semantics so the split doesn't accidentally flip
    /// the nested-type extractor too.
    #[test]
    fn nested_struct_extractor_keeps_combined_check() {
        let source = r#"
pub struct ResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_field: Option<String>,
    pub required_field: String,
}
"#;
        let s = parser()
            .extract_struct("ResponseModel", source)
            .expect("struct present");
        // maybe_field: both attr AND Option<...> -> required=false
        assert_eq!(
            s.fields.get("maybe_field").map(|p| p.required),
            Some(false)
        );
        // required_field: no attr, no Option -> required=true
        assert_eq!(
            s.fields.get("required_field").map(|p| p.required),
            Some(true)
        );
    }
}

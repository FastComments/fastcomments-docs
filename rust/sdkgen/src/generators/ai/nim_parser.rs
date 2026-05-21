//! Port of `src/sdk-doc-generators/nim-parser.js`.

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
    #[serde(rename = "isArray", default)]
    pub is_array: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    #[serde(rename = "responseType")]
    pub response_type: String,
    #[serde(serialize_with = "serialize_indexmap")]
    pub parameters: indexmap::IndexMap<String, ParamInfo>,
    pub description: String,
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
}

impl super::common::EnrichableMethod for Method {
    fn set_http_method(&mut self, v: Option<String>) { self.http_method = v; }
    fn set_path(&mut self, v: Option<String>) { self.path = v; }
    fn set_tag(&mut self, v: Option<String>) { self.tag = v; }
    fn set_auth_type(&mut self, v: Option<String>) { self.auth_type = v; }
    /// nim Method.description is `String`; only overridden when the
    /// OpenAPI description is non-empty (nim-ai-generator.js parity).
    fn override_description_with_openapi(&mut self, d: Option<&str>) {
        if let Some(s) = d {
            if !s.is_empty() {
                self.description = s.to_string();
            }
        }
    }
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

pub struct NimParser {
    pub repo_path: PathBuf,
    pub models_path: PathBuf,
    type_cache: RefCell<BTreeMap<String, Option<NestedType>>>,
}

impl NimParser {
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
            tracing::warn!(path = %path.display(), "nim API file not found");
            return Vec::new();
        };
        // Mirrors nim-parser.js:32. The `(?s)` flag is needed so `[\s\S]*?`
        // (param list spanning multiple lines) works.
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"(?s)proc\s+(\w+)\*\((.*?)\):\s*\(Option\[(\w+)\],\s*Response\)\s*=",
            )
            .expect("regex")
        });
        let mut out = Vec::new();
        for cap in RE.captures_iter(&content) {
            let method_name = cap[1].to_string();
            let params_str = cap[2].to_string();
            let response_type = cap[3].trim().to_string();
            let parameters = Self::parse_parameters(&params_str);
            let mut nested_types: indexmap::IndexMap<String, NestedType> =
                indexmap::IndexMap::new();
            if let Some(td) = self.load_type_definition(&response_type) {
                nested_types.insert(response_type.clone(), td);
            }
            out.push(Method {
                name: method_name,
                response_type,
                parameters,
                description: String::new(),
                nested_types,
                http_method: None,
                path: None,
                tag: None,
                auth_type: None,
            });
        }
        out
    }

    fn parse_parameters(s: &str) -> indexmap::IndexMap<String, ParamInfo> {
        let mut out = indexmap::IndexMap::new();
        let s = s.trim();
        if s.is_empty() {
            return out;
        }
        // Split on `,` ignoring brackets.
        let mut parts: Vec<String> = Vec::new();
        let mut current = String::new();
        let mut depth = 0i32;
        for c in s.chars() {
            if c == '[' {
                depth += 1;
                current.push(c);
            } else if c == ']' {
                depth -= 1;
                current.push(c);
            } else if c == ',' && depth == 0 {
                parts.push(current.trim().to_string());
                current.clear();
            } else {
                current.push(c);
            }
        }
        if !current.trim().is_empty() {
            parts.push(current.trim().to_string());
        }
        let re = Regex::new(r"(\w+):\s*(.+?)$").unwrap();
        for part in parts {
            if let Some(cap) = re.captures(&part) {
                let name = cap[1].to_string();
                let ty = cap[2].trim().to_string();
                // Optionality rule from nim-parser.js:107-110.
                let is_optional = ty != "HttpClient"
                    && name != "tenantId"
                    && name != "urlId"
                    && name != "commentId";
                let is_array = ty.starts_with("seq[");
                out.insert(
                    name,
                    ParamInfo {
                        type_: ty,
                        required: !is_optional,
                        is_array,
                    },
                );
            }
        }
        out
    }

    fn load_type_definition(&self, type_name: &str) -> Option<NestedType> {
        if let Some(cached) = self.type_cache.borrow().get(type_name) {
            return cached.clone();
        }
        let snake = nim_type_to_snake(type_name);
        let file_name = format!("model_{snake}.nim");
        let full = self.models_path.join(&file_name);
        if !full.exists() {
            self.type_cache
                .borrow_mut()
                .insert(type_name.to_string(), None);
            return None;
        }
        // Match Node's path.replace(repo + '/', '').
        let rel = match full.strip_prefix(&self.repo_path) {
            Ok(p) => p.to_string_lossy().into_owned(),
            Err(_) => file_name.clone(),
        };
        let td = NestedType {
            summary: format!("Type definition for {type_name}"),
            file_path: rel,
        };
        self.type_cache
            .borrow_mut()
            .insert(type_name.to_string(), Some(td.clone()));
        Some(td)
    }
}

/// Mirrors nim-parser.js:137-143:
///   1. strip underscores
///   2. insert `_` between lowercase/digit -> uppercase
///   3. insert `_` between uppercase -> uppercase+lowercase
///   4. lowercase
///   5. replace `url_id` -> `urlid`
fn nim_type_to_snake(s: &str) -> String {
    let no_under: String = s.chars().filter(|c| *c != '_').collect();
    let re1 = Regex::new(r"([a-z0-9])([A-Z])").unwrap();
    let s = re1.replace_all(&no_under, "${1}_${2}").into_owned();
    let re2 = Regex::new(r"([A-Z])([A-Z][a-z])").unwrap();
    let s = re2.replace_all(&s, "${1}_${2}").into_owned();
    let lower = s.to_lowercase();
    lower.replace("url_id", "urlid")
}

/// snake_case -> camelCase. Mirrors nim-ai-generator.js:66
/// (`method.name.replace(/_([a-z])/g, (m, p1) => p1.toUpperCase())`).
pub fn snake_to_camel(s: &str) -> String {
    let re = Regex::new(r"_([a-z])").unwrap();
    re.replace_all(s, |c: &regex::Captures| c[1].to_uppercase())
        .into_owned()
}

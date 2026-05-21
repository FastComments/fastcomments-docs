//! Port of `src/sdk-doc-generators/cpp-parser.js`.
//!
//! Extracts method signatures from C++ header files preceded by a
//! `/// <summary>...</summary>` doc block.

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
    /// cpp Method.description is `String`; only overridden when the
    /// OpenAPI description is non-empty (cpp-ai-generator.js:69).
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

pub struct CppParser {
    pub repo_path: PathBuf,
    pub models_path: PathBuf,
    type_cache: RefCell<BTreeMap<String, Option<NestedType>>>,
}

impl CppParser {
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
            tracing::warn!(path = %path.display(), "cpp API file not found");
            return Vec::new();
        };
        // Mirrors cpp-parser.js:32.
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"(?s)/// <summary>(.*?)/// </summary>.*?pplx::task<std::shared_ptr<(\w+)>>\s+(\w+)\s*\((.*?)\)\s*const;",
            )
            .expect("regex")
        });
        let mut out = Vec::new();
        for cap in RE.captures_iter(&content) {
            let summary = cap[1].to_string();
            let response_type = cap[2].trim().to_string();
            let method_name = cap[3].to_string();
            let params_str = cap[4].to_string();

            let parameters = Self::parse_parameters(&params_str);
            let description = clean_summary(&summary);

            let mut nested_types: indexmap::IndexMap<String, NestedType> = indexmap::IndexMap::new();
            if let Some(rt) = self.load_type_definition(&response_type) {
                nested_types.insert(response_type.clone(), rt);
            }

            out.push(Method {
                name: method_name,
                response_type,
                parameters,
                description,
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
        // Split on `,` but ignore commas inside angle brackets.
        let mut parts: Vec<String> = Vec::new();
        let mut current = String::new();
        let mut depth = 0i32;
        for c in s.chars() {
            if c == '<' {
                depth += 1;
                current.push(c);
            } else if c == '>' {
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
        // Match: Type paramName or boost::optional<Type> paramName.
        // Mirrors cpp-parser.js:100.
        let re = Regex::new(
            r"(?s)(?:boost::optional<(.+?)>|std::shared_ptr<(.+?)>|(.+?))\s+(\w+)(?:\s*=\s*.+)?$",
        )
        .unwrap();
        for part in parts {
            if let Some(cap) = re.captures(&part) {
                let optional = cap.get(1).map(|m| m.as_str().to_string());
                let shared_ptr = cap.get(2).map(|m| m.as_str().to_string());
                let direct = cap.get(3).map(|m| m.as_str().to_string());
                let name = cap[4].to_string();
                let is_optional = optional.is_some();
                let mut actual = optional.or(shared_ptr).or(direct).unwrap_or_default();
                actual = actual
                    .replace("utility::string_t", "string")
                    .replace("utility::datetime", "datetime")
                    .replace("std::vector<", "vector<")
                    .replace("std::shared_ptr<", "")
                    .replace('>', "")
                    .trim()
                    .to_string();
                out.insert(
                    name,
                    ParamInfo {
                        type_: actual,
                        required: !is_optional,
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
        let file_name = format!("{type_name}.h");
        let full = self.models_path.join(&file_name);
        if !full.exists() {
            self.type_cache.borrow_mut().insert(type_name.to_string(), None);
            return None;
        }
        // Match Node's `path.relative(repo, full)`. Mirrors cpp-parser.js:145.
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

fn clean_summary(s: &str) -> String {
    let r = Regex::new(r"///").unwrap();
    r.replace_all(s, "").trim().to_string()
}

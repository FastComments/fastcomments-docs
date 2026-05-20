//! Port of `src/sdk-doc-generators/typescript-parser.js`.
//!
//! Extracts `async {methodName}(requestParameters: {RequestType})`
//! signatures from TypeScript API files, then for each method
//! resolves the request interface and the response type by reading
//! the matching `models/*.ts` files.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamInfo {
    #[serde(rename = "type")]
    pub type_: String,
    pub required: bool,
}

/// Matches the Node parser's per-method output shape (see
/// `typescript-parser.js:42-50`). Field order is `name`,
/// `requestType`, `responseType`, `parameters`, `nestedTypes` to mirror
/// JS object insertion order so the cache-key SHA256 matches.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    #[serde(rename = "requestType")]
    pub request_type: String,
    #[serde(rename = "responseType")]
    pub response_type: String,
    /// Insertion-ordered. JS uses Object literal field order; we
    /// emulate with a BTreeMap (since interface property declaration
    /// order is alphabetical for the JSON serializer anyway). Actually
    /// Node uses standard Object — let's match.
    #[serde(serialize_with = "serialize_indexmap")]
    pub parameters: indexmap::IndexMap<String, ParamInfo>,
    /// Nested type definitions resolved from models/*.ts files.
    /// Each value is either a string (enum summary) or a structured
    /// object (interface summary + filePath). Mirrors the JS shape:
    /// nestedTypes is a flat `{TypeName: {summary, filePath}}` map
    /// keyed by type name, populated during recursive resolution.
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedType {
    pub summary: String,
    #[serde(rename = "filePath")]
    pub file_path: String,
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

pub struct TypescriptParser {
    pub repo_path: PathBuf,
    pub models_path: PathBuf,
    /// Cached type definitions to avoid re-parsing the same model file
    /// during recursive nested-type resolution.
    type_cache: std::cell::RefCell<BTreeMap<String, Option<TypeDef>>>,
}

#[derive(Debug, Clone)]
struct TypeDef {
    name: String,
    kind: &'static str, // "enum" or "interface"
    properties: indexmap::IndexMap<String, ParamInfo>,
    summary: String,
    file_path: String,
}

impl TypescriptParser {
    pub fn new(repo_path: PathBuf, models_path_rel: &str) -> Self {
        let models_path = repo_path.join(models_path_rel);
        Self {
            repo_path,
            models_path,
            type_cache: std::cell::RefCell::new(BTreeMap::new()),
        }
    }

    /// Extract every API method declared in `api_file_rel` (path
    /// relative to repo_path). Mirrors `extractApiMethods` at
    /// `typescript-parser.js:19-79`.
    pub fn extract_api_methods(&self, api_file_rel: &str) -> Vec<Method> {
        let path = self.repo_path.join(api_file_rel);
        let Ok(content) = std::fs::read_to_string(&path) else {
            tracing::warn!(path = %path.display(), "API file not found");
            return Vec::new();
        };
        let mut out = Vec::new();
        static SIG_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"async\s+(\w+)\s*\(\s*requestParameters:\s*(\w+)(?:[^)]*)\):\s*Promise<([^>]+)>",
            )
            .expect("regex")
        });
        for cap in SIG_RE.captures_iter(&content) {
            let name = cap[1].to_string();
            // Skip the `xxxRaw` variants — they're internal helpers.
            if name.ends_with("Raw") {
                continue;
            }
            let request_type = cap[2].trim().to_string();
            let response_type = cap[3].trim().to_string();

            let mut method = Method {
                name,
                request_type: request_type.clone(),
                response_type: response_type.clone(),
                parameters: indexmap::IndexMap::new(),
                nested_types: indexmap::IndexMap::new(),
                http_method: None,
                path: None,
                tag: None,
                auth_type: None,
                description: None,
            };

            // Extract the request interface.
            if let Some(iface) = self.extract_interface(&request_type, &content) {
                method.parameters = iface.properties.clone();
                self.resolve_nested_types(&iface.properties, &mut method.nested_types, 0, 3);
            }

            // Resolve the response type.
            if let Some(rt) = self.load_type_definition(&response_type) {
                method.nested_types.insert(
                    response_type.clone(),
                    NestedType {
                        summary: rt.summary.clone(),
                        file_path: rt.file_path.clone(),
                    },
                );
                if rt.kind == "interface" {
                    self.resolve_nested_types(&rt.properties, &mut method.nested_types, 0, 2);
                }
            }
            out.push(method);
        }
        out
    }

    fn extract_interface(&self, name: &str, content: &str) -> Option<InterfaceDef> {
        let re = Regex::new(&format!(
            r"export\s+interface\s+{}\s*\{{",
            regex::escape(name)
        ))
        .ok()?;
        let m = re.find(content)?;
        let start = m.end();
        // Find matching closing brace.
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
        let props_text = &content[start..end];
        // Match `propertyName?: Type;`. Stops at `;`.
        static PROP_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(\w+)(\?)?:\s*([^;]+);").expect("regex"));
        let mut properties: indexmap::IndexMap<String, ParamInfo> = indexmap::IndexMap::new();
        for cap in PROP_RE.captures_iter(props_text) {
            let pname = cap[1].to_string();
            let optional = cap.get(2).is_some();
            let ptype = cap[3].trim().to_string();
            properties.insert(
                pname,
                ParamInfo {
                    type_: ptype,
                    required: !optional,
                },
            );
        }
        Some(InterfaceDef {
            name: name.to_string(),
            properties,
        })
    }

    fn resolve_nested_types(
        &self,
        properties: &indexmap::IndexMap<String, ParamInfo>,
        out: &mut indexmap::IndexMap<String, NestedType>,
        current_depth: usize,
        max_depth: usize,
    ) {
        if current_depth >= max_depth {
            return;
        }
        for info in properties.values() {
            for type_name in self.extract_type_names(&info.type_) {
                if Self::is_primitive_type(&type_name) || out.contains_key(&type_name) {
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
                    if td.kind == "interface" {
                        self.resolve_nested_types(
                            &td.properties,
                            out,
                            current_depth + 1,
                            max_depth,
                        );
                    }
                }
            }
        }
    }

    fn extract_type_names(&self, type_expr: &str) -> Vec<String> {
        let mut s = type_expr.to_string();
        // Strip ` | null` and ` | undefined`.
        s = Regex::new(r"\s*\|\s*null")
            .unwrap()
            .replace_all(&s, "")
            .into_owned();
        s = Regex::new(r"\s*\|\s*undefined")
            .unwrap()
            .replace_all(&s, "")
            .into_owned();
        s = s.trim().to_string();

        // Array<T> -> recurse on T.
        if let Some(c) = Regex::new(r"Array<(.+)>").unwrap().captures(&s) {
            return self.extract_type_names(&c[1]);
        }
        // Union types Type1 | Type2.
        if s.contains('|') {
            let mut out = Vec::new();
            for part in s.split('|') {
                out.extend(self.extract_type_names(part.trim()));
            }
            return out;
        }
        // Dictionary { [key: string]: ValueType }
        if let Some(c) = Regex::new(r"\{\s*\[.*\]:\s*([^}]+)\}").unwrap().captures(&s) {
            return self.extract_type_names(&c[1]);
        }
        if Self::is_primitive_type(&s) {
            return Vec::new();
        }
        vec![s]
    }

    fn is_primitive_type(t: &str) -> bool {
        matches!(t, "string" | "number" | "boolean" | "Date" | "any" | "void" | "unknown")
    }

    fn load_type_definition(&self, type_name: &str) -> Option<TypeDef> {
        if let Some(cached) = self.type_cache.borrow().get(type_name) {
            return cached.clone();
        }
        let type_file = self.models_path.join(format!("{type_name}.ts"));
        if !type_file.exists() {
            self.type_cache.borrow_mut().insert(type_name.to_string(), None);
            return None;
        }
        let Ok(content) = std::fs::read_to_string(&type_file) else {
            self.type_cache.borrow_mut().insert(type_name.to_string(), None);
            return None;
        };
        // Try enum first.
        if let Some(values) = self.extract_enum(type_name, &content) {
            let summary = format!(
                "enum: {}",
                values
                    .iter()
                    .map(|v| format!("'{v}'"))
                    .collect::<Vec<_>>()
                    .join(" | ")
            );
            let td = TypeDef {
                name: type_name.to_string(),
                kind: "enum",
                properties: indexmap::IndexMap::new(),
                summary,
                file_path: format!("{type_name}.ts"),
            };
            self.type_cache
                .borrow_mut()
                .insert(type_name.to_string(), Some(td.clone()));
            return Some(td);
        }
        // Interface.
        if let Some(iface) = self.extract_interface(type_name, &content) {
            let summary = self.summarize_interface(&iface);
            let td = TypeDef {
                name: type_name.to_string(),
                kind: "interface",
                properties: iface.properties,
                summary,
                file_path: format!("{type_name}.ts"),
            };
            self.type_cache
                .borrow_mut()
                .insert(type_name.to_string(), Some(td.clone()));
            return Some(td);
        }
        self.type_cache.borrow_mut().insert(type_name.to_string(), None);
        None
    }

    fn extract_enum(&self, name: &str, content: &str) -> Option<Vec<String>> {
        let re = Regex::new(&format!(
            r"(?s)export\s+enum\s+{}\s*\{{([^}}]+)\}}",
            regex::escape(name)
        ))
        .ok()?;
        let cap = re.captures(content)?;
        let body = &cap[1];
        static VAL_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r#"(\w+)\s*=\s*['"]([^'"]+)['"]"#).expect("regex"));
        let mut out = Vec::new();
        for c in VAL_RE.captures_iter(body) {
            out.push(c[2].to_string());
        }
        Some(out)
    }

    fn summarize_interface(&self, iface: &InterfaceDef) -> String {
        let mut props = Vec::new();
        for (name, info) in &iface.properties {
            let opt = if info.required { "" } else { "?" };
            props.push(format!("{name}{opt}: {type_}", type_ = info.type_));
        }
        format!("{{ {} }}", props.join(", "))
    }
}

struct InterfaceDef {
    #[allow(dead_code)]
    name: String,
    properties: indexmap::IndexMap<String, ParamInfo>,
}

/// Cargo-local re-export used by per-language modules.
pub(crate) use indexmap;

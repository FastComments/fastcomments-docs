//! Port of `src/sdk-doc-generators/typescript-parser.js`.
//!
//! Extracts `async {methodName}(requestParameters: {RequestType})`
//! signatures from TypeScript API files, then for each method
//! resolves the request interface and the response type by reading
//! the matching `models/*.ts` files.

use std::collections::BTreeMap;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

pub use super::common::ParamInfo;

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
    #[serde(serialize_with = "super::common::serialize_indexmap")]
    pub parameters: indexmap::IndexMap<String, ParamInfo>,
    /// Nested type definitions resolved from models/*.ts files.
    /// Each value is either a string (enum summary) or a structured
    /// object (interface summary + filePath). Mirrors the JS shape:
    /// nestedTypes is a flat `{TypeName: {summary, filePath}}` map
    /// keyed by type name, populated during recursive resolution.
    #[serde(rename = "nestedTypes", serialize_with = "super::common::serialize_indexmap")]
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

crate::impl_enrichable_method_setters!(Method);
impl super::common::DescriptionOverride for Method {
    /// Option<String> raw assignment — mirrors Node typescript-ai-generator.js.
    fn override_description(&mut self, d: Option<&str>) {
        self.description = d.map(|s| s.to_string());
    }
}

impl super::common::MethodForSection for Method {
    const LANG_TAG: &'static str = "typescript";
    const PREPEND_MODELS_PATH: bool = true;
    fn section_name(&self) -> &str { &self.name }
    fn section_description(&self) -> &str { self.description.as_deref().unwrap_or("") }
    fn section_params(&self) -> Vec<(String, String, bool)> {
        self.parameters.iter().map(|(k, v)| (k.clone(), v.type_.clone(), v.required)).collect()
    }
    fn section_response_type(&self) -> &str { &self.response_type }
    fn section_response_display(&self) -> String { self.response_type.clone() }
    fn section_nested_file_path(&self) -> Option<&str> {
        self.nested_types.get(&self.response_type).map(|n| n.file_path.as_str())
    }
    fn section_tag(&self) -> Option<&str> { self.tag.as_deref() }
    fn section_path(&self) -> Option<&str> { self.path.as_deref() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedType {
    pub summary: String,
    #[serde(rename = "filePath")]
    pub file_path: String,
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

        // Array<T> -> recurse on T. MUST match Node lazily (`[^>]+`),
        // NOT greedily (`.+`). For nested generics like
        // `Array<Array<X>>`:
        //   - Node (lazy)  captures `Array<X`  — stops at the first
        //     `>`, recurses on a broken fragment that no model file
        //     resolves; the inner X is never added to nestedTypes.
        //   - Rust (greedy) would capture `Array<X>` — stops at the
        //     last `>`, recurses, and eventually finds X. nestedTypes
        //     would gain an entry Node doesn't have.
        // Both behaviors are arguably broken in different ways, but
        // matching Node's broken behavior is mandatory for cache-key
        // parity (nestedTypes is part of the SHA256). The "right"
        // fix lives outside this parity port.
        if let Some(c) = Regex::new(r"Array<([^>]+)>").unwrap().captures(&s) {
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

#[cfg(test)]
mod tests {
    //! Pin the type-name extraction to Node's exact shape — the
    //! captured names propagate into method.nestedTypes which is
    //! part of the cache SHA256, so even arguably-broken corner
    //! cases must match Node bit-for-bit.
    use super::*;
    use std::path::PathBuf;

    fn p() -> TypescriptParser {
        TypescriptParser::new(PathBuf::from("/tmp"), "src/generated/src/models/")
    }

    fn names(t: &str) -> Vec<String> {
        p().extract_type_names(t)
    }

    /// Pins the regression. Real source has this exact shape:
    ///   `images?: Array<Array<any>>;` (GifSearchResponse.ts)
    /// Node lazily captures `Array<X` from the outer match (`[^>]+`
    /// stops at the FIRST `>`), then recurses on the broken fragment
    /// — net nestedTypes contribution = empty. Rust greedily would
    /// have captured `Array<X>` and recursed further, eventually
    /// adding the inner type to nestedTypes. Different nestedTypes
    /// -> different cache SHA256.
    #[test]
    fn nested_array_matches_node_lazy_capture() {
        // With a real inner type the divergence would have been a
        // single nestedTypes entry — verify Rust no longer adds it.
        // (TypescriptParser only adds non-primitive names; the broken
        // fragment `Array<X` is never primitive but also doesn't
        // resolve to any model file, so it's effectively dropped at
        // load time. We pin the EXTRACTION step here — the same step
        // Node does.)
        let got = names("Array<Array<MyType>>");
        // Node's broken-fragment behavior: outer match captures the
        // inner `Array<MyType`, that recursion finds no Array match,
        // no `|`, no dict, and the fragment isn't a primitive, so it
        // ends up as the "type name". Match exactly.
        assert_eq!(got, vec!["Array<MyType".to_string()]);
    }

    #[test]
    fn simple_array_unchanged() {
        // `Array<MyType>` → `MyType`. Both regexes agree on this.
        let got = names("Array<MyType>");
        assert_eq!(got, vec!["MyType".to_string()]);
    }

    #[test]
    fn array_with_primitive_inner_returns_empty() {
        // `Array<any>` -> recurse on `any` -> primitive -> filtered.
        let got = names("Array<any>");
        assert!(got.is_empty(), "got: {got:?}");
        // The real-source case: `Array<Array<any>>` -> outer captures
        // `Array<any` (broken). It's not a primitive, so it would
        // leak as a "name" — but the loader never finds a matching
        // model file so it doesn't bloat nestedTypes. Pin the
        // extraction shape anyway.
        let got = names("Array<Array<any>>");
        assert_eq!(got, vec!["Array<any".to_string()]);
    }

    #[test]
    fn union_with_array_arm_matches_node_first_match_semantic() {
        // Node `.match` returns the FIRST match only. Pre-fix Rust
        // greedy would have eaten across the union boundary, then
        // tripped the `|` branch with garbage. Lazy regex stops at
        // the first `>`, so we recurse on just the array inner.
        let got = names("Array<First> | Second");
        // The wrapper-removal pass only strips `| null` / `| undefined`,
        // not arbitrary unions, so this input reaches the Array branch
        // intact. Lazy captures `First`, recurses, returns `[First]`.
        // The `| Second` half is dropped — same as Node.
        assert_eq!(got, vec!["First".to_string()]);
    }

    #[test]
    fn null_undefined_wrappers_stripped_before_array_match() {
        // Wrappers are removed at the top of the fn. After:
        //   `Array<MyType> | null` -> `Array<MyType>` -> `[MyType]`
        let got = names("Array<MyType> | null");
        assert_eq!(got, vec!["MyType".to_string()]);
        let got = names("Array<MyType> | undefined");
        assert_eq!(got, vec!["MyType".to_string()]);
    }
}

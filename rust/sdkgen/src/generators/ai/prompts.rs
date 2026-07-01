//! Per-language prompt builders. Each is a verbatim port of the
//! matching `build{Lang}Prompt` in
//! `src/sdk-doc-generators/the legacy Node LLM client:151-359`. Byte-for-byte
//! string output is required so the cache key SHA256 matches the
//! Node-computed value and we hit the existing `sdk-ai-cache/` files.

use super::typescript_parser::{Method, NestedType, ParamInfo};

/// Append the standard "Function Parameters:" block, mirroring the
/// shape every per-language prompt emits. The only variations are:
///   - `header`: typescript/cpp/nim use "Function Parameters:"; rust
///     uses "Function Parameters (inside the params struct):".
///   - `optional_label`: typescript/nim "optional"; cpp
///     "optional (boost::optional<T>)"; rust "optional (Option<T>)".
///   - `skip`: nim drops `httpClient`; the others keep everything.
/// Generic over the per-language `ParamInfo` so each parser's own
/// type stays intact.
fn push_parameters_block<P>(
    lines: &mut Vec<String>,
    header: &str,
    params: &indexmap::IndexMap<String, P>,
    type_of: impl Fn(&P) -> &str,
    required_of: impl Fn(&P) -> bool,
    optional_label: &str,
    skip: impl Fn(&str) -> bool,
) {
    lines.push(header.to_string());
    if params.is_empty() {
        lines.push("  (none)".to_string());
        return;
    }
    for (name, info) in params {
        if skip(name) {
            continue;
        }
        let required_str = if required_of(info) {
            "required"
        } else {
            optional_label
        };
        lines.push(format!("  - {name}: {ty} ({required_str})", ty = type_of(info)));
    }
}

/// Append the "Type Definitions: { Name: [object Object] }" block
/// that typescript_prompt and rust_prompt both emit verbatim. The
/// `[object Object]` literal is preserved for byte-parity with Node's
/// `${typeDef}` -> stringification bug.
fn push_nested_types_obj_object(
    lines: &mut Vec<String>,
    nested_types: &indexmap::IndexMap<String, NestedType>,
) {
    if nested_types.is_empty() {
        return;
    }
    lines.push(String::new());
    lines.push("Type Definitions:".to_string());
    for (type_name, _td) in nested_types {
        lines.push(format!("  {type_name}: [object Object]"));
    }
}

/// Append the "Type Definitions: { Name: summary | 'Type definition' }"
/// block that cpp_prompt and nim_prompt both emit. Falls back to
/// "Type definition" when the summary is empty, matching Node.
fn push_nested_types_with_summary<N>(
    lines: &mut Vec<String>,
    nested_types: &indexmap::IndexMap<String, N>,
    summary_of: impl Fn(&N) -> &str,
) {
    if nested_types.is_empty() {
        return;
    }
    lines.push(String::new());
    lines.push("Type Definitions:".to_string());
    for (type_name, td) in nested_types {
        let s = summary_of(td);
        let summary = if s.is_empty() { "Type definition" } else { s };
        lines.push(format!("  {type_name}: {summary}"));
    }
}

pub fn typescript_prompt(method: &Method) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "Create a TypeScript code example that calls the async function \"{}\".",
        method.name
    ));
    lines.push(String::new());
    lines.push("Pretend this function is globally available (do not import it or create an API instance).".to_string());
    lines.push(String::new());
    push_parameters_block(
        &mut lines,
        "Function Parameters:",
        &method.parameters,
        |p| &p.type_,
        |p| p.required,
        "optional",
        |_| false,
    );
    lines.push(String::new());
    let rt = if method.response_type.is_empty() {
        "void".to_string()
    } else {
        method.response_type.clone()
    };
    lines.push(format!("Return Type: {rt}"));

    push_nested_types_obj_object(&mut lines, &method.nested_types);

    lines.push(String::new());
    lines.push("Requirements:".to_string());
    lines.push("1. Do NOT include any imports".to_string());
    lines.push("2. Do NOT create an API instance or configuration".to_string());
    lines.push("3. Do NOT include error handling (no try/catch)".to_string());
    lines.push("4. Use realistic parameter values (not 'example' strings)".to_string());
    lines.push(
        "5. Show the function call with await and assign the result to a typed variable"
            .to_string(),
    );
    lines.push(
        "6. Use the correct return type for the result variable (from Return Type above)"
            .to_string(),
    );
    lines.push("7. Use TypeScript type annotations for all variables".to_string());
    lines.push(
        "8. NEVER use \"as any\" casts - use proper types from the Type Definitions above"
            .to_string(),
    );
    lines.push("9. Demonstrate optional parameters where relevant".to_string());
    lines.push("10. Keep example very concise (< 20 lines)".to_string());
    lines.push(String::new());
    lines.push("Return only the TypeScript code, no explanations or markdown formatting.".to_string());

    lines.join("\n")
}

/// Reference for parameter formatting in the prompts. Kept for
/// future use if we extend non-TS generators.
#[allow(dead_code)]
pub fn fmt_param(name: &str, info: &ParamInfo) -> String {
    let required = if info.required { "required" } else { "optional" };
    format!("  - {name}: {} ({required})", info.type_)
}

/// Reference for nested-type formatting. Kept for future use.
#[allow(dead_code)]
pub fn fmt_nested(name: &str, type_def: &NestedType) -> String {
    format!("  {name}: {}", type_def.summary)
}

// ------------------------------------------------------------------
// Rust prompt
// ------------------------------------------------------------------

pub fn rust_prompt(method: &super::rust_parser::Method) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "Create an idiomatic Rust code example that calls the async function \"{}\".",
        method.name
    ));
    lines.push(String::new());
    lines.push("The function signature is:".to_string());
    lines.push(format!(
        "pub async fn {}(configuration: &configuration::Configuration, params: {}) -> Result<{}, Error>",
        method.name, method.params_type, method.response_type
    ));
    lines.push(String::new());
    push_parameters_block(
        &mut lines,
        "Function Parameters (inside the params struct):",
        &method.parameters,
        |p| &p.type_,
        |p| p.required,
        "optional (Option<T>)",
        |_| false,
    );
    lines.push(String::new());
    let rt = if method.response_type.is_empty() {
        "unit".to_string()
    } else {
        method.response_type.clone()
    };
    lines.push(format!("Return Type: Result<{rt}, Error>"));
    push_nested_types_obj_object(&mut lines, &method.nested_types);
    lines.push(String::new());
    lines.push("Requirements:".to_string());
    lines.push("1. Do NOT include any use statements or imports".to_string());
    lines.push("2. Assume configuration and all types are already in scope".to_string());
    lines.push("3. Create a params struct instance with realistic values".to_string());
    lines.push("4. Use realistic parameter values (not \"example_string\" - use actual realistic values like \"acme-corp-tenant\", \"news/article\", etc.)".to_string());
    lines.push("5. Call the function with .await and use ? operator to unwrap the Result".to_string());
    lines.push("6. Use proper Rust type annotations and ownership semantics".to_string());
    lines.push("7. Demonstrate optional parameters with Some(...) where relevant".to_string());
    lines.push("8. Keep example very concise (< 25 lines)".to_string());
    lines.push("9. Use idiomatic Rust style (snake_case, proper formatting)".to_string());
    lines.push("10. Do NOT add any comments or explanations in the code".to_string());
    lines.push(String::new());
    lines.push("Return only the Rust code, no explanations or markdown formatting.".to_string());
    lines.join("\n")
}

// ------------------------------------------------------------------
// C++ prompt
// ------------------------------------------------------------------

pub fn cpp_prompt(method: &super::cpp_parser::Method) -> String {
    let mut lines: Vec<String> = Vec::new();
    let rt = if method.response_type.is_empty() {
        "void".to_string()
    } else {
        method.response_type.clone()
    };
    lines.push(format!(
        "Create an idiomatic C++ code example that calls the async method \"{}\" from the FastComments C++ SDK.",
        method.name
    ));
    lines.push(String::new());
    lines.push("The method returns:".to_string());
    lines.push(format!("pplx::task<std::shared_ptr<{rt}>>"));
    lines.push(String::new());
    push_parameters_block(
        &mut lines,
        "Function Parameters:",
        &method.parameters,
        |p| &p.type_,
        |p| p.required,
        "optional (boost::optional<T>)",
        |_| false,
    );
    lines.push(String::new());
    lines.push(format!("Return Type: pplx::task<std::shared_ptr<{rt}>>"));
    push_nested_types_with_summary(&mut lines, &method.nested_types, |td| &td.summary);
    lines.push(String::new());
    lines.push("Requirements:".to_string());
    lines.push("1. Do NOT include any #include statements or namespace declarations".to_string());
    lines.push("2. Assume an API client instance named \"api\" is already created and in scope".to_string());
    lines.push("3. Use realistic parameter values (not \"example_string\" - use actual realistic values like \"my-tenant-123\", \"user@example.com\", etc.)".to_string());
    lines.push(format!(
        "4. Call the method using api->{}(...) and chain with .then() to handle the result",
        method.name
    ));
    lines.push("5. Use proper C++ types: utility::string_t for strings, std::make_shared for shared pointers".to_string());
    lines.push("6. Demonstrate optional parameters with boost::optional where relevant".to_string());
    lines.push("7. Keep example very concise (< 25 lines)".to_string());
    lines.push("8. Use idiomatic C++ style and proper formatting".to_string());
    lines.push("9. Do NOT add any comments or explanations in the code".to_string());
    lines.push("10. The SDK uses cpprest (Microsoft C++ REST SDK), so use utility::string_t for strings".to_string());
    lines.push(String::new());
    lines.push("Return only the C++ code, no explanations or markdown formatting.".to_string());
    lines.join("\n")
}

// ------------------------------------------------------------------
// Nim prompt
// ------------------------------------------------------------------

pub fn nim_prompt(method: &super::nim_parser::Method) -> String {
    let mut lines: Vec<String> = Vec::new();
    let rt = if method.response_type.is_empty() {
        "void".to_string()
    } else {
        method.response_type.clone()
    };
    lines.push(format!(
        "Create an idiomatic Nim code example that calls the function \"{}\" from the FastComments Nim SDK.",
        method.name
    ));
    lines.push(String::new());
    lines.push("The function returns:".to_string());
    lines.push(format!("(Option[{rt}], Response)"));
    lines.push(String::new());
    push_parameters_block(
        &mut lines,
        "Function Parameters:",
        &method.parameters,
        |p| &p.type_,
        |p| p.required,
        "optional",
        // Skip the httpClient parameter (matches the legacy Node LLM client:322).
        |name| name == "httpClient",
    );
    lines.push(String::new());
    lines.push(format!("Return Type: (Option[{rt}], Response)"));
    push_nested_types_with_summary(&mut lines, &method.nested_types, |td| &td.summary);
    lines.push(String::new());
    lines.push("Requirements:".to_string());
    lines.push("1. Do NOT include any import statements".to_string());
    lines.push("2. Assume an HttpClient instance named \"client\" is already created and in scope".to_string());
    lines.push("3. CRITICAL: You MUST use named arguments for ALL function parameters (e.g., client.getCommentsPublic(tenantId = \"...\", urlId = \"...\", page = 0, ...))".to_string());
    lines.push("4. Use realistic parameter values (not \"example_string\" - use actual realistic values like \"my-tenant-123\", \"news/article-title\", etc.)".to_string());
    // Note: Node leaves the literal `${method.name}` text in the
    // prompt string (it's NOT a template literal inside that string).
    lines.push("5. Call the function and destructure the result tuple like: let (response, httpResponse) = client.${method.name}(...)".to_string());
    lines.push("6. Check if response.isSome and access the value with response.get()".to_string());
    lines.push("7. Use proper Nim types: string, int, bool, seq[string] for arrays".to_string());
    lines.push("8. For optional/default parameters, pass appropriate default values (0 for int, \"\" for string, false for bool, @[] for seq, etc.)".to_string());
    lines.push("9. Keep example very concise (< 30 lines)".to_string());
    lines.push("10. Use idiomatic Nim style (camelCase for variables, proper indentation with 2 spaces)".to_string());
    lines.push("11. Do NOT add any comments or explanations in the code".to_string());
    lines.push("12. IMPORTANT: ALWAYS use named arguments like paramName = value for every parameter".to_string());
    lines.push(String::new());
    lines.push("Return only the Nim code, no explanations or markdown formatting.".to_string());
    lines.join("\n")
}

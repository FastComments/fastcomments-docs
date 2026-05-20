//! Per-language prompt builders. Each is a verbatim port of the
//! matching `build{Lang}Prompt` in
//! `src/sdk-doc-generators/openai-client.js:151-359`. Byte-for-byte
//! string output is required so the cache key SHA256 matches the
//! Node-computed value and we hit the existing `sdk-ai-cache/` files.

use super::typescript_parser::{Method, NestedType, ParamInfo};

pub fn typescript_prompt(method: &Method) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "Create a TypeScript code example that calls the async function \"{}\".",
        method.name
    ));
    lines.push(String::new());
    lines.push("Pretend this function is globally available (do not import it or create an API instance).".to_string());
    lines.push(String::new());
    lines.push("Function Parameters:".to_string());
    if method.parameters.is_empty() {
        lines.push("  (none)".to_string());
    } else {
        for (name, info) in &method.parameters {
            let required = if info.required { "required" } else { "optional" };
            lines.push(format!(
                "  - {name}: {ty} ({required})",
                ty = info.type_
            ));
        }
    }
    lines.push(String::new());
    let rt = if method.response_type.is_empty() {
        "void".to_string()
    } else {
        method.response_type.clone()
    };
    lines.push(format!("Return Type: {rt}"));

    if !method.nested_types.is_empty() {
        lines.push(String::new());
        lines.push("Type Definitions:".to_string());
        for (type_name, type_def) in &method.nested_types {
            // The TypeScript prompt template stringifies the NESTED
            // type def directly (Object), which produces "[object Object]"
            // when concatenated. Node's actual output is the literal
            // `${typeDef}` of an object — which IS `[object Object]`.
            // So we emit that string verbatim to match.
            let _ = type_def;
            lines.push(format!("  {type_name}: [object Object]"));
        }
    }

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

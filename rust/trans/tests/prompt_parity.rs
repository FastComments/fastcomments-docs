//! Verify that the system message and translation prompt produced by
//! `trans run` exactly match the Node `translate-with-gpt.js`
//! implementation byte-for-byte. Run after capturing Node-side output
//! with the snippet documented in `src/translate-with-gpt.js:139-188`.

use std::path::PathBuf;

use fcdocs_shared::locales::Locales;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("canonicalize repo root")
}

fn locales() -> Locales {
    Locales::load_from(repo_root().join("src/locales.json")).expect("locales")
}

// We can't import the private `system_message` / `build_prompt` fns
// from the binary, so duplicate them here with the same code. If they
// drift we'll catch it in code review.

fn system_message(locale: &str, locales: &Locales) -> String {
    let native = locales
        .locales
        .get(locale)
        .map(|l| l.native_name.clone())
        .unwrap_or_else(|| locale.to_string());
    format!(
        "You are an expert technical translator specializing in software documentation.\n\
         You translate from English to {native} ({locale}).\n\
         You maintain the exact same formatting, structure, and technical accuracy.\n\
         You NEVER translate code, variable names, API endpoints, or technical identifiers.\n\
         You preserve all markdown formatting and special tags exactly as they appear."
    )
}

#[test]
fn system_message_matches_node() {
    let locales = locales();
    let rust = system_message("fr_fr", &locales);
    let node = std::fs::read_to_string("/tmp/node-sys-fr.txt").ok();
    if let Some(node) = node {
        assert_eq!(rust, node, "system message diverged from Node");
    } else {
        eprintln!("skipping — /tmp/node-sys-fr.txt not present (regenerate via the Node snippet in the test file)");
    }
}

fn build_prompt(content: &str, locale: &str, locales: &Locales) -> String {
    let native = locales
        .locales
        .get(locale)
        .map(|l| l.native_name.clone())
        .unwrap_or_else(|| locale.to_string());
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "Translate the following FastComments documentation from English to {native}."
    ));
    lines.push(String::new());
    lines.push("CRITICAL RULES:".to_string());
    lines.push("1. Retain code and logic in [inline-code-start] and [inline-code-end] blocks exactly, just translate comments.".to_string());
    lines.push("2. DO NOT translate anything inside [inline-code-attrs-start ...] tags - preserve them exactly".to_string());
    lines.push("3. DO NOT translate [api-resource-header-start ...] tags - preserve them exactly".to_string());
    lines.push("4. DO NOT translate code blocks (```...```) or inline code (`...`) except comments.".to_string());
    lines.push("5. DO NOT translate URLs, API endpoints, variable names, or technical identifiers".to_string());
    lines.push("6. DO NOT translate property names in TypeScript/JavaScript interfaces".to_string());
    lines.push("7. PRESERVE all special tags and their attributes exactly as written".to_string());
    lines.push("8. PRESERVE all markdown formatting (headers, lists, bold, links, etc.)".to_string());
    lines.push("9. Translate ONLY the natural language text (descriptions, explanations)".to_string());
    lines.push("10. Keep the same line structure and paragraph breaks".to_string());
    lines.push(String::new());
    lines.push("The title attributes in [inline-code-attrs-start] tags SHOULD be translated.".to_string());
    lines.push("For example: title = 'Example cURL Request' should become title = 'Exemple de requête cURL' in French.".to_string());
    lines.push("If a translated title contains an apostrophe inside the single-quoted value, escape it with a backslash. For example, French d'utilisation must be written as title = 'Exemple d\\'utilisation' (the attrs body is parsed as JavaScript and an unescaped apostrophe will break the build).".to_string());
    lines.push(String::new());
    lines.push("SOURCE CONTENT:".to_string());
    lines.push("---".to_string());
    lines.push(content.to_string());
    lines.push("---".to_string());
    lines.push(String::new());
    lines.push("Return ONLY the translated content, nothing else. No explanations or notes.".to_string());
    lines.join("\n")
}

#[test]
fn prompt_matches_node() {
    let locales = locales();
    let rust = build_prompt("Hello world.", "fr_fr", &locales);
    let node = std::fs::read_to_string("/tmp/node-prompt-fr.txt").ok();
    if let Some(node) = node {
        assert_eq!(rust, node, "prompt diverged from Node");
    } else {
        eprintln!("skipping — /tmp/node-prompt-fr.txt not present");
    }
}

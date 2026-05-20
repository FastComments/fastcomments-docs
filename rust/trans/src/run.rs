//! `trans run` — port of `src/translate-with-gpt.js`.
//!
//! Walks the same per-locale items tree as `trans check`, identifies
//! missing or stale translations, and calls OpenAI for each. Prompts
//! and cache keys mirror the Node implementation byte-for-byte so
//! existing `translation-cache.json` entries remain valid hits.
//!
//! Scope of the port:
//! - Markdown file translations (the main use case — `processTranslations`).
//! - Cache load/save in the same JSON shape Node uses.
//! - Inline-code-attrs sanitization (`sanitizeInlineCodeAttrs`).
//!
//! Out of scope here (covered separately if/when needed):
//! - UI strings translation (`translations.json`) — covered by the
//!   `processUITranslations` path in the Node version.
//! - meta.json translation — covered by `processMetaJsonTranslations`.
//! Both are mechanical to port using the same LlmClient; deferred to
//! a follow-up.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use fcdocs_shared::locales::Locales;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::snapshot::hash_content;

const DEFAULT_MODEL: &str = "gpt-5-mini";

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
struct CacheMap(BTreeMap<String, String>);

pub async fn run() -> Result<()> {
    let repo = repo_root()?;
    let guides_dir = repo.join("src/content/guides");
    let cache_path = repo.join("src/translation-cache.json");
    let locales_path = repo.join("src/locales.json");
    let locales = Locales::load_from(&locales_path)?;
    let mut cache: CacheMap = if cache_path.exists() {
        serde_json::from_slice(&std::fs::read(&cache_path)?).unwrap_or_default()
    } else {
        CacheMap::default()
    };

    let tasks = build_task_list(&guides_dir, &locales, &cache)?;
    info!(count = tasks.len(), "missing translation tasks");
    if tasks.is_empty() {
        info!("nothing to translate");
        return Ok(());
    }

    let api_key = std::env::var("OPENAI_API_KEY").ok();
    if api_key.is_none() {
        warn!(
            "OPENAI_API_KEY not set — `trans run` cannot call OpenAI. \
             Run with the key set, or use `node src/translate-with-gpt.js` \
             which has the same behavior."
        );
        anyhow::bail!("OPENAI_API_KEY required");
    }

    let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(600))
        .build()?;
    let api_key = api_key.unwrap();

    let mut success = 0usize;
    let mut failed = 0usize;
    let mut skipped = 0usize;

    // Serial worker — Node uses concurrency 5 by default; we run
    // sequentially to keep error handling simple. If throughput is
    // ever an issue, swap for a tokio JoinSet with a semaphore.
    for task in &tasks {
        let source_path = guides_dir
            .join(&task.guide_id)
            .join("items")
            .join(&locales.default_locale)
            .join(&task.filename);
        let Ok(source) = std::fs::read_to_string(&source_path) else {
            warn!(path = %source_path.display(), "source missing");
            failed += 1;
            continue;
        };

        if source.trim().len() < 10 {
            info!(
                "[skipped] {}/{}/{} (too small)",
                task.guide_id, task.locale, task.filename
            );
            cache.0.insert(
                cache_key(&task.guide_id, &task.locale, &task.filename),
                task.source_hash.clone(),
            );
            save_cache(&cache_path, &cache)?;
            skipped += 1;
            continue;
        }

        // en_us special-case: copy source verbatim. Mirrors
        // translate-with-gpt.js:362-366.
        let translation = if task.locale == "en_us" {
            source.clone()
        } else {
            let prompt = build_prompt(&source, &task.locale, &locales);
            let system = system_message(&task.locale, &locales);
            match call_openai(&client, &api_key, &model, &system, &prompt, &task.filename).await {
                Ok(t) => sanitize_inline_code_attrs(&t),
                Err(e) => {
                    warn!(
                        "[error] {}/{}/{}: {}",
                        task.guide_id, task.locale, task.filename, e
                    );
                    failed += 1;
                    continue;
                }
            }
        };

        // Validate inline-code count parity. Mirrors translate-with-gpt.js:299-311.
        if task.locale != "en_us" {
            let src_counts = count_inline_code(&source);
            let tr_counts = count_inline_code(&translation);
            if src_counts != tr_counts {
                warn!(
                    "[warning] inline-code mismatch in {}/{}/{}: expected {:?}, got {:?}",
                    task.guide_id, task.locale, task.filename, src_counts, tr_counts
                );
            }
        }

        // Save translated file.
        let target_dir = guides_dir
            .join(&task.guide_id)
            .join("items")
            .join(&task.locale);
        std::fs::create_dir_all(&target_dir)?;
        let target_path = target_dir.join(&task.filename);
        std::fs::write(&target_path, &translation)
            .with_context(|| format!("write {target_path:?}"))?;

        cache.0.insert(
            cache_key(&task.guide_id, &task.locale, &task.filename),
            task.source_hash.clone(),
        );
        save_cache(&cache_path, &cache)?;
        success += 1;
    }

    info!(success, failed, skipped, "trans run complete");
    if failed > 0 {
        anyhow::bail!("{failed} translation task(s) failed");
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct Task {
    guide_id: String,
    locale: String,
    filename: String,
    source_hash: String,
}

fn build_task_list(
    guides_dir: &Path,
    locales: &Locales,
    cache: &CacheMap,
) -> Result<Vec<Task>> {
    let mut tasks = Vec::new();
    for entry in std::fs::read_dir(guides_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let guide_id = entry.file_name().to_string_lossy().into_owned();
        let items_path = entry.path().join("items");
        if !items_path.exists() {
            continue;
        }
        let default_items = items_path.join(&locales.default_locale);
        if !default_items.exists() {
            continue;
        }
        for src_entry in std::fs::read_dir(&default_items)?.flatten() {
            let p = src_entry.path();
            if p.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            let filename = p
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_string();
            let Ok(content) = std::fs::read_to_string(&p) else {
                continue;
            };
            let source_hash = hash_content(&content);

            for (locale, _) in &locales.locales {
                if locale == &locales.default_locale {
                    continue;
                }
                let target = items_path.join(locale).join(&filename);
                let cache_key = cache_key(&guide_id, locale, &filename);
                let cached_hash = cache.0.get(&cache_key);
                let needs = !target.exists() || cached_hash != Some(&source_hash);
                if needs {
                    tasks.push(Task {
                        guide_id: guide_id.clone(),
                        locale: locale.clone(),
                        filename: filename.clone(),
                        source_hash: source_hash.clone(),
                    });
                }
            }
        }
    }
    Ok(tasks)
}

fn cache_key(guide_id: &str, locale: &str, filename: &str) -> String {
    format!("{guide_id}/{locale}/{filename}")
}

fn save_cache(path: &Path, cache: &CacheMap) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(&cache.0)?;
    std::fs::write(path, bytes).with_context(|| format!("write {path:?}"))?;
    Ok(())
}

/// Verbatim port of `getSystemMessage` at translate-with-gpt.js:139-148.
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

/// Verbatim port of `buildPrompt` at translate-with-gpt.js:156-188.
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

/// Verbatim port of `sanitizeInlineCodeAttrs` at translate-with-gpt.js:45-62.
fn sanitize_inline_code_attrs(text: &str) -> String {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?s)\[inline-code-attrs-start ([\s\S]*?) inline-code-attrs-end\]")
            .expect("regex")
    });
    static TOK_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?s)^(\s*\w+\s*=\s*)'([\s\S]*)'(\s*)$").expect("regex")
    });
    RE.replace_all(text, |caps: &regex::Captures| {
        let body = &caps[1];
        let tokens: Vec<&str> = body.split("; ").collect();
        let fixed: Vec<String> = tokens
            .iter()
            .map(|tok| {
                if let Some(m) = TOK_RE.captures(tok) {
                    let prefix = m[1].to_string();
                    let value = m[2].to_string();
                    let suffix = m[3].to_string();
                    // Replace unescaped `'` with `\'`, leaving already-escaped
                    // `\'` untouched. Use NUL as a placeholder during the swap.
                    let escaped = value
                        .replace("\\'", "\u{0000}")
                        .replace('\'', "\\'")
                        .replace('\u{0000}', "\\'");
                    format!("{prefix}'{escaped}'{suffix}")
                } else {
                    tok.to_string()
                }
            })
            .collect();
        format!(
            "[inline-code-attrs-start {} inline-code-attrs-end]",
            fixed.join("; ")
        )
    })
    .into_owned()
}

fn count_inline_code(content: &str) -> (usize, usize) {
    (
        content.matches("[inline-code-start]").count(),
        content.matches("[inline-code-end]").count(),
    )
}

async fn call_openai(
    client: &reqwest::Client,
    api_key: &str,
    model: &str,
    system: &str,
    prompt: &str,
    filename: &str,
) -> Result<String> {
    let body = serde_json::json!({
        "model": model,
        "messages": [
            {"role": "system", "content": system},
            {"role": "user", "content": prompt},
        ],
        "max_completion_tokens": 16000,
    });
    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .with_context(|| format!("POST OpenAI for {filename}"))?;
    let status = resp.status();
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("OpenAI API error {status}: {text}");
    }
    let value: serde_json::Value = resp.json().await?;
    let text = value
        .pointer("/choices/0/message/content")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .trim()
        .to_string();
    if text.is_empty() {
        anyhow::bail!("OpenAI returned empty content");
    }
    Ok(text)
}

fn repo_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let mut cur: &Path = cwd.as_path();
    loop {
        if cur.join("package.json").exists() && cur.join("src/locales.json").exists() {
            return Ok(cur.to_path_buf());
        }
        match cur.parent() {
            Some(p) => cur = p,
            None => anyhow::bail!("could not locate repo root from {cwd:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_replaces_unescaped_apostrophes() {
        let input = "[inline-code-attrs-start title = 'Exemple d'utilisation'; type = 'html' inline-code-attrs-end]";
        let out = sanitize_inline_code_attrs(input);
        assert_eq!(
            out,
            "[inline-code-attrs-start title = 'Exemple d\\'utilisation'; type = 'html' inline-code-attrs-end]"
        );
    }

    #[test]
    fn sanitize_preserves_already_escaped() {
        let input = "[inline-code-attrs-start title = 'Already \\'escaped\\'' inline-code-attrs-end]";
        let out = sanitize_inline_code_attrs(input);
        assert_eq!(
            out,
            "[inline-code-attrs-start title = 'Already \\'escaped\\'' inline-code-attrs-end]"
        );
    }

    #[test]
    fn system_message_format() {
        let locales = Locales {
            default_locale: "en".into(),
            locales: indexmap::IndexMap::from([(
                "fr_fr".to_string(),
                fcdocs_shared::locales::Locale {
                    name: "French (France)".into(),
                    native_name: "Français (France)".into(),
                    hreflang: "fr-FR".into(),
                    flag: Some("🇫🇷".into()),
                },
            )]),
        };
        let sm = system_message("fr_fr", &locales);
        assert!(sm.contains("Français (France) (fr_fr)"));
    }
}

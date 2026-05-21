//! Verify the Handlebars templates load and render with realistic fixture
//! data. Catches feature mismatches (`{{#each}}`, `{{@key}}`,
//! escaped vs unescaped variables, etc.) between handlebars.js and the
//! `handlebars` Rust crate.

use std::path::PathBuf;

use fcdocs_shared::templates::TemplateRegistry;
use serde_json::json;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("canonicalize repo root")
}

#[test]
fn loads_all_four_templates() {
    let reg = TemplateRegistry::load_from(repo_root().join("src/templates")).unwrap();
    // Render each with minimal context just to confirm parse.
    for name in ["page", "guide-layout", "index", "code"] {
        let _ = reg.render(name, &json!({
            "lang": "en",
            "title": "x",
            "content": "<p>x</p>",
            "intro": "",
            "conclusion": "",
            "buildId": "b",
            "lastUpdateDate": "now",
            "locale": "en",
            "t": {"DOCUMENTATION_TITLE": "Docs", "LOADING": "Loading", "LLM_KIT_BANNER": "k"},
            "availableLocales": [],
            "alternateLocales": [],
            "guides": [],
            "gettingStartedGuides": [],
            "installationGuides": [],
            "sdkGuides": [],
            "items": [],
            "itemsBySubCat": {},
            "isFallback": false,
            "id": "x",
            "name": "x",
            "icon": null,
            "url": "x.html",
            "metaJSONPath": "",
            "itemsPath": "",
            "indexTemplatePath": "",
            "conclusionPath": "",
            "introPath": "",
            "pageHeader": "",
        }));
    }
}

#[test]
fn index_renders_with_translations_and_guides() {
    let reg = TemplateRegistry::load_from(repo_root().join("src/templates")).unwrap();
    let ctx = json!({
        "lang": "en",
        "buildId": "abc123",
        "lastUpdateDate": "now",
        "locale": "en",
        "t": {
            "DOCUMENTATION_TITLE": "FastComments Docs",
            "DOCUMENTATION_DESCRIPTION": "Docs",
            "GETTING_STARTED": "Getting Started",
            "ALL_GUIDES": "All Guides",
            "OTHER_PLATFORMS": "Other Platforms",
            "SDKS": "SDKs",
            "SEARCH_PLACEHOLDER": "Search",
            "NO_RESULTS": "No results",
            "LOADING": "Loading",
            "LLM_KIT_BANNER": "LLM kit",
            "DOCUMENTATION_LAST_UPDATED": "Last updated:",
            "CONTRIBUTE": "Contribute",
        },
        "availableLocales": [
            {"code": "en", "name": "English", "nativeName": "English", "flag": "🇺🇸", "url": "index.html", "current": true},
            {"code": "fr_fr", "name": "French", "nativeName": "Français", "flag": "🇫🇷", "url": "index-fr_fr.html", "current": false},
        ],
        "gettingStartedGuides": [
            {"id": "installation", "name": "Installation", "url": "guide-installation.html", "icon": "/images/guide-icons/install.png"},
        ],
        "guides": [
            {"id": "api", "name": "API", "url": "guide-api.html", "icon": null},
        ],
        "installationGuides": [],
        "sdkGuides": [],
    });
    let out = reg.render("index", &ctx).unwrap();
    assert!(out.contains("<title>FastComments - FastComments Docs</title>"));
    assert!(out.contains("Getting Started"));
    assert!(out.contains("guide-installation.html"));
    assert!(out.contains("guide-api.html"));
    assert!(out.contains("🇺🇸"));
    assert!(out.contains("Français"));
    // {{lang}} substitution
    assert!(out.contains(r#"<html lang="en">"#));
}

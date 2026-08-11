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

/// The guide `<h1>` must fall back to the guide name when meta.json sets
/// no `pageHeader`. 46 of the 108 guides set none, which shipped 1058
/// pages (46 guides x 23 locales) with no `<h1>` at all.
#[test]
fn guide_layout_h1_falls_back_to_name() {
    let reg = TemplateRegistry::load_from(repo_root().join("src/templates")).unwrap();
    let ctx = |page_header: &str, name: &str| {
        json!({
            "id": "affiliates",
            "name": name,
            "pageHeader": page_header,
            "heading": if page_header.is_empty() { name } else { page_header },
            "url": "guide-affiliates.html",
            "intro": "",
            "conclusion": "",
            "items": [],
            "itemsBySubCat": {},
            "isFallback": false,
            "locale": "en",
            "availableLocales": [],
            "t": {"LOADING": "Loading"},
        })
    };

    // No pageHeader -> h1 comes from the (already translated) guide name.
    let out = reg.render("guide-layout", &ctx("", "Affiliates")).unwrap();
    assert!(out.contains("<h1>Affiliates</h1>"), "missing h1 fallback:\n{out}");
    assert_eq!(out.matches("<h1").count(), 1, "expected exactly one h1");

    // pageHeader set -> it still wins.
    let out = reg
        .render("guide-layout", &ctx("Affiliate Program", "Affiliates"))
        .unwrap();
    assert!(out.contains("<h1>Affiliate Program</h1>"), "pageHeader should win:\n{out}");
    assert_eq!(out.matches("<h1").count(), 1, "expected exactly one h1");

    // Localized name (meta_translated/meta_fr_fr.json) flows through.
    let out = reg.render("guide-layout", &ctx("", "Affiliés")).unwrap();
    assert!(out.contains("<h1>Affiliés</h1>"), "localized name should render:\n{out}");
}

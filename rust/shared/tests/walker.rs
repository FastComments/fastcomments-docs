//! Integration test: walk the real content tree and confirm the types match.
//! Runs against the live `src/content/guides/` directory so any meta.json
//! shape drift surfaces immediately.

use std::path::PathBuf;

use fcdocs_shared::guides::GuidesRoot;
use fcdocs_shared::locales::Locales;

fn repo_root() -> PathBuf {
    // tests live at rust/shared/tests/, repo root is 3 up.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("canonicalize repo root")
}

#[test]
fn loads_locales_json() {
    let locales = Locales::load_from(repo_root().join("src/locales.json")).unwrap();
    assert_eq!(locales.default_locale, "en");
    assert!(locales.locales.contains_key("en"));
    assert!(locales.locales.contains_key("fr_fr"));
    assert!(locales.locales.contains_key("zh_cn"));
    assert!(locales.locales.len() >= 28);
}

#[test]
fn walks_default_locale_guides() {
    let root = GuidesRoot::new(repo_root().join("src/content/guides"), "en");
    let guides = root.walk("en").unwrap();
    assert!(guides.len() > 50, "expected many guides, got {}", guides.len());
    // Spot check a known guide.
    let installation = guides.iter().find(|g| g.id == "installation").expect("installation guide present");
    assert!(!installation.meta.items_ordered.is_empty());
    assert_eq!(installation.meta.page_header.as_deref(), Some("Add Comments To a Website"));
}

#[test]
fn resolves_item_path_with_fallback() {
    let root = GuidesRoot::new(repo_root().join("src/content/guides"), "en");
    let (path, is_fallback) = root.resolve_item_path("installation", "wordpress.md", "en");
    assert!(path.exists(), "wordpress.md should exist for default locale");
    assert!(!is_fallback);

    // Non-English locale falls back to en when no translation present.
    let (path2, _) = root.resolve_item_path("installation", "wordpress.md", "fr_fr");
    assert!(path2.exists());
}

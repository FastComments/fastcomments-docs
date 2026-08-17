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

/// `sso` is the live example of a nav-only redirect stub: its meta.json
/// has a `url` into the customizations guide and zero items. It must
/// still be walked (the homepage renders its card) but must report as a
/// stub so sitegen skips generating `guide-sso*.html` for it.
#[test]
fn sso_is_a_redirect_stub_but_still_walked() {
    let root = GuidesRoot::new(repo_root().join("src/content/guides"), "en");
    let guides = root.walk("en").unwrap();

    let sso = guides.iter().find(|g| g.id == "sso").expect("sso guide is still listed");
    assert!(sso.meta.items_ordered.is_empty(), "sso has no items of its own");
    assert_eq!(
        sso.meta.url.as_deref(),
        Some("/guide-customizations-and-configuration.html#sso")
    );
    assert!(sso.meta.is_redirect_stub());

    // A guide with real content must never be classified as a stub.
    let installation =
        guides.iter().find(|g| g.id == "installation").expect("installation guide present");
    assert!(!installation.meta.is_redirect_stub());
}

/// Guides with neither items nor a `url` (`authentication`, `wordpress`)
/// are dropped by the walk entirely, so they never reached the page
/// loop in the first place. Pinned so the stub filter isn't mistaken
/// for what excludes them.
#[test]
fn contentless_guides_without_a_url_are_not_walked() {
    let root = GuidesRoot::new(repo_root().join("src/content/guides"), "en");
    let guides = root.walk("en").unwrap();
    for id in ["authentication", "wordpress"] {
        assert!(
            !guides.iter().any(|g| g.id == id),
            "{id} has no items and no url, so it should not be walked"
        );
    }
}

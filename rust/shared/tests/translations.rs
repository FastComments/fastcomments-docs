//! Verify translations.json loads and locale fallback works.

use std::path::PathBuf;

use fcdocs_shared::translations::Translations;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("canonicalize repo root")
}

#[test]
fn loads_known_locales() {
    let t = Translations::load_from(repo_root().join("src/translations.json"), "en").unwrap();
    let en = t.for_locale("en");
    let en = &en.map;
    assert_eq!(en.get("DOCUMENTATION_TITLE").map(String::as_str), Some("Documentation"));
    assert_eq!(en.get("LOADING").map(String::as_str), Some("Loading..."));
}

#[test]
fn falls_back_to_default_locale() {
    let t = Translations::load_from(repo_root().join("src/translations.json"), "en").unwrap();
    // A nonexistent locale should fall back to en.
    let fake = t.for_locale("xx_yy_zz");
    let fake = &fake.map;
    assert_eq!(fake.get("DOCUMENTATION_TITLE").map(String::as_str), Some("Documentation"));
}

#[test]
fn real_locales_have_distinct_translations() {
    let t = Translations::load_from(repo_root().join("src/translations.json"), "en").unwrap();
    let en = t.for_locale("en");
    let bg = t.for_locale("bg_bg");
    let en = &en.map;
    let bg = &bg.map;
    assert_ne!(en.get("DOCUMENTATION_TITLE"), bg.get("DOCUMENTATION_TITLE"));
    assert_eq!(bg.get("DOCUMENTATION_TITLE").map(String::as_str), Some("Документация"));
}

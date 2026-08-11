//! `trans validate-images` - the build gate for image parity.
//!
//! Walks every guide's default-locale items, and for each translated
//! locale file that exists on disk, compares its image references
//! against the source's (see [`crate::images`]). Any mismatch is a
//! hard build failure: an image the translator dropped, duplicated, or
//! whose path it "translated" renders as a missing or wrong picture on
//! the localized page, and a mangled `[app-screenshot-*]` attribute
//! block makes `sitegen` skip the screenshot entirely.
//!
//! This runs as its own build.sh phase AFTER `trans check` / `trans
//! run`, so it validates freshly-written translations too. It's also
//! reachable from `trans check` (via [`audit`]) so a mismatch marks
//! the affected files for re-translation instead of only being
//! reported at the end.

use std::path::Path;

use anyhow::Result;
use fcdocs_shared::locales::Locales;
use tracing::info;

use crate::discover::default_locale_files;
use crate::images::{image_diff, ImageDiff};

/// One translated file whose images don't match the source's.
#[derive(Debug, Clone)]
pub struct Mismatch {
    pub guide_id: String,
    pub locale: String,
    pub filename: String,
    pub diff: ImageDiff,
}

impl Mismatch {
    pub fn describe(&self) -> String {
        format!(
            "{}/{}/{}: {}",
            self.guide_id,
            self.locale,
            self.filename,
            self.diff.describe()
        )
    }
}

/// Compare every existing translated item against its default-locale
/// source. Files that don't exist yet are NOT reported here - that's
/// the "missing translation" gap `check`/`run` already handle, and
/// reporting it twice would just be noise.
pub fn audit(guides_dir: &Path, locales: &Locales) -> Vec<Mismatch> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(guides_dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let Ok(ft) = entry.file_type() else { continue };
        if !ft.is_dir() {
            continue;
        }
        let guide_id = entry.file_name().to_string_lossy().into_owned();
        let guide_dir = entry.path();
        let items_dir = guide_dir.join("items");
        if !items_dir.exists() {
            continue;
        }
        for src in default_locale_files(&guide_dir, &locales.default_locale) {
            let Ok(source) = std::fs::read_to_string(&src.source_path) else {
                continue;
            };
            // Note: no "source has no images -> skip" shortcut here.
            // A translation that INVENTS an image (or duplicates a
            // whole section) has to fail too, and the full walk costs
            // well under a second across the entire content tree.
            for (locale, _) in &locales.locales {
                if locale == &locales.default_locale {
                    continue;
                }
                let target = items_dir.join(locale).join(&src.filename);
                let Ok(translated) = std::fs::read_to_string(&target) else {
                    continue; // not translated yet - `check` owns that gap
                };
                if let Some(diff) = image_diff(&source, &translated) {
                    out.push(Mismatch {
                        guide_id: guide_id.clone(),
                        locale: locale.clone(),
                        filename: src.filename.clone(),
                        diff,
                    });
                }
            }
        }
    }
    out.sort_by(|a, b| {
        (&a.guide_id, &a.filename, &a.locale).cmp(&(&b.guide_id, &b.filename, &b.locale))
    });
    out
}

/// Subcommand entry point. Exits non-zero (failing the build) when any
/// translated file's images don't match the source's.
pub async fn run() -> Result<()> {
    let repo = fcdocs_shared::repo::repo_root()?;
    let guides_dir = repo.join("src/content/guides");
    let locales = Locales::load_from(&repo.join("src/locales.json"))?;

    let mismatches = audit(&guides_dir, &locales);
    if mismatches.is_empty() {
        info!("image parity OK - every translated item has the same images as its source");
        return Ok(());
    }
    // Print all of them, not a sample: this fails the build, so the
    // log has to contain everything needed to fix it.
    for m in &mismatches {
        tracing::error!("[image-mismatch] {}", m.describe());
    }
    tracing::error!(
        count = mismatches.len(),
        "translated items have image references that differ from the default locale"
    );
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use fcdocs_shared::locales::Locale;
    use indexmap::IndexMap;

    fn locales_en_fr_de() -> Locales {
        let mut m: IndexMap<String, Locale> = IndexMap::new();
        for k in ["en", "fr_fr", "de_de"] {
            m.insert(
                k.to_string(),
                Locale {
                    name: k.into(),
                    native_name: k.into(),
                    hreflang: k.into(),
                    flag: None,
                },
            );
        }
        Locales {
            default_locale: "en".into(),
            locales: m,
        }
    }

    fn write(root: &Path, rel: &str, body: &str) {
        let p = root.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(&p, body).unwrap();
    }

    #[test]
    fn matching_translations_produce_no_mismatches() {
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write(g, "a/items/en/x.md", "<img src=\"/i/a.png\" alt=\"A\">");
        write(g, "a/items/fr_fr/x.md", "<img src=\"/i/a.png\" alt=\"Le A\">");
        write(g, "a/items/de_de/x.md", "<img src=\"/i/a.png\" alt=\"Das A\">");
        assert!(audit(g, &locales_en_fr_de()).is_empty());
    }

    #[test]
    fn dropped_image_in_one_locale_is_reported() {
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write(
            g,
            "a/items/en/x.md",
            "<img src=\"/i/a.png\">\n<img src=\"/i/b.png\">",
        );
        write(g, "a/items/fr_fr/x.md", "<img src=\"/i/a.png\">");
        write(
            g,
            "a/items/de_de/x.md",
            "<img src=\"/i/a.png\">\n<img src=\"/i/b.png\">",
        );
        let ms = audit(g, &locales_en_fr_de());
        assert_eq!(ms.len(), 1);
        assert_eq!(ms[0].locale, "fr_fr");
        assert_eq!(ms[0].diff.missing.len(), 1);
    }

    #[test]
    fn untranslated_file_is_not_an_image_mismatch() {
        // fr_fr has no copy at all. That's `check`'s "missing
        // translation" gap, not an image mismatch - reporting it here
        // would fail the build for a file that simply hasn't been
        // translated yet.
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write(g, "a/items/en/x.md", "<img src=\"/i/a.png\">");
        write(g, "a/items/de_de/x.md", "<img src=\"/i/a.png\">");
        let ms = audit(g, &locales_en_fr_de());
        assert!(ms.is_empty(), "{ms:?}");
    }

    #[test]
    fn root_level_intro_is_covered() {
        // intro.md lives at the guide root but its translations land
        // under items/<locale>/. discover::default_locale_files
        // handles the fallback; make sure the audit follows it.
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write(g, "a/items/en/x.md", "no images here");
        write(g, "a/intro.md", "<img src=\"/i/intro.png\">");
        write(g, "a/items/fr_fr/intro.md", "pas d'image");
        let ms = audit(g, &locales_en_fr_de());
        assert_eq!(ms.len(), 1);
        assert_eq!(ms[0].filename, "intro.md");
        assert_eq!(ms[0].locale, "fr_fr");
    }

    #[test]
    fn image_invented_in_a_translation_is_reported() {
        // Source has no images; the fr_fr copy grew one. Caught only
        // because the walk has no "source has no images -> skip"
        // shortcut.
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write(g, "a/items/en/x.md", "Just prose.");
        write(g, "a/items/fr_fr/x.md", "Du texte.\n<img src=\"/i/x.png\">");
        write(g, "a/items/de_de/x.md", "Nur Text.");
        let ms = audit(g, &locales_en_fr_de());
        assert_eq!(ms.len(), 1);
        assert_eq!(ms[0].locale, "fr_fr");
        assert_eq!(ms[0].diff.extra.len(), 1);
    }

    #[test]
    fn guide_without_items_dir_is_skipped() {
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write(g, "a/meta.json", "{}");
        assert!(audit(g, &locales_en_fr_de()).is_empty());
    }

    #[test]
    fn malformed_screenshot_attrs_are_reported() {
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write(
            g,
            "a/items/en/x.md",
            "[app-screenshot-start url='/u'; selector = '.c'; title='T' app-screenshot-end]",
        );
        write(
            g,
            "a/items/fr_fr/x.md",
            "[app-screenshot-start url='/u'; selector = '.c' title='Le T' app-screenshot-end]",
        );
        write(
            g,
            "a/items/de_de/x.md",
            "[app-screenshot-start url='/u'; selector = '.c'; title='Das T' app-screenshot-end]",
        );
        let ms = audit(g, &locales_en_fr_de());
        assert_eq!(ms.len(), 1);
        assert_eq!(ms[0].locale, "fr_fr");
    }
}

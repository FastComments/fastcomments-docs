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
use crate::images::{extract_screenshot_bodies, image_diff, ImageDiff};

/// Why a translated file fails the gate.
#[derive(Debug, Clone)]
pub enum Problem {
    /// Its image references differ from the source's.
    Images(ImageDiff),
    /// An `[app-screenshot-*]` block doesn't evaluate as JavaScript -
    /// usually an unescaped apostrophe in a translated `title` / `alt`,
    /// which terminates the quoted value early. `sitegen` reacts to
    /// this by logging `skip item ... error=eval app-screenshot config`
    /// and dropping the ENTIRE page from the build, so it has to be a
    /// hard failure here rather than a warning nobody reads.
    UnparseableScreenshot { body: String, error: String },
}

impl Problem {
    pub fn describe(&self) -> String {
        match self {
            Problem::Images(diff) => diff.describe(),
            Problem::UnparseableScreenshot { body, error } => {
                format!("app-screenshot block does not parse ({error}): [app-screenshot-start{body}app-screenshot-end]")
            }
        }
    }
}

/// One translated file that fails the gate.
#[derive(Debug, Clone)]
pub struct Mismatch {
    pub guide_id: String,
    pub locale: String,
    pub filename: String,
    pub problem: Problem,
}

impl Mismatch {
    pub fn describe(&self) -> String {
        format!(
            "{}/{}/{}: {}",
            self.guide_id,
            self.locale,
            self.filename,
            self.problem.describe()
        )
    }
}

/// The single definition of "this translated file is broken", shared by
/// the gate, `trans check`, and `trans run`'s task discovery. They MUST
/// agree: if `run` used a narrower rule it would skip a file the gate
/// rejects, and the build would fail forever with nothing trying to fix
/// it.
pub fn file_problem(source: &str, translated: &str) -> Option<Problem> {
    if let Some(diff) = image_diff(source, translated) {
        return Some(Problem::Images(diff));
    }
    // Only the translation is evaluated. A source block that doesn't
    // parse is an authoring bug, not a translation bug, and flagging it
    // per-locale would report the same problem 28 times.
    for body in extract_screenshot_bodies(translated) {
        if let Err(e) = fcdocs_shared::markers::eval_marker_sync(
            fcdocs_shared::sidecar::MarkerKind::ApiResourceHeader,
            &body,
        ) {
            return Some(Problem::UnparseableScreenshot {
                body,
                error: format!("{e}").lines().next().unwrap_or("").to_string(),
            });
        }
    }
    None
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
                if let Some(problem) = file_problem(&source, &translated) {
                    out.push(Mismatch {
                        guide_id: guide_id.clone(),
                        locale: locale.clone(),
                        filename: src.filename.clone(),
                        problem,
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

/// Rewrite every translated item's `[app-screenshot-*]` blocks from its
/// source (see [`crate::images::merge_screenshot_blocks`]) and report
/// how many files changed.
///
/// `trans run` does this to fresh LLM output, so this is for the
/// back-catalog: translations written before the merge existed still
/// carry corrupted URLs and unescaped apostrophes. Idempotent - a
/// second run changes nothing.
fn fix_screenshot_blocks(guides_dir: &Path, locales: &Locales) -> Result<usize> {
    let mut fixed = 0usize;
    let Ok(entries) = std::fs::read_dir(guides_dir) else {
        return Ok(0);
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
            if crate::images::extract_screenshot_bodies(&source).is_empty() {
                continue;
            }
            for (locale, _) in &locales.locales {
                if locale == &locales.default_locale {
                    continue;
                }
                let target = items_dir.join(locale).join(&src.filename);
                let Ok(translated) = std::fs::read_to_string(&target) else {
                    continue;
                };
                let merged = crate::images::merge_screenshot_blocks(&source, &translated);
                if merged != translated {
                    std::fs::write(&target, &merged)?;
                    fixed += 1;
                    info!("[fixed] {guide_id}/{locale}/{}", src.filename);
                }
            }
        }
    }
    Ok(fixed)
}

/// Subcommand entry point. Exits non-zero (failing the build) when any
/// translated file's images don't match the source's.
///
/// `--fix` first rebuilds every translated `[app-screenshot-*]` block
/// from its source. That is a maintenance action, never part of the
/// build: build.sh calls this with no arguments so it only ever reports.
pub async fn run_with<I: IntoIterator<Item = String>>(args: I) -> Result<()> {
    let mut fix = false;
    for arg in args {
        match arg.as_str() {
            "--fix" => fix = true,
            other => anyhow::bail!("unknown arg: {other:?} (only --fix is supported)"),
        }
    }
    let repo = fcdocs_shared::repo::repo_root()?;
    let guides_dir = repo.join("src/content/guides");
    let locales = Locales::load_from(&repo.join("src/locales.json"))?;

    if fix {
        let n = fix_screenshot_blocks(&guides_dir, &locales)?;
        info!(files = n, "rebuilt app-screenshot blocks from source");
    }

    let mismatches = audit(&guides_dir, &locales);
    if mismatches.is_empty() {
        info!("image parity OK - every translated item has the same images as its source, and every app-screenshot block parses");
        return Ok(());
    }
    // Print all of them, not a sample: this fails the build, so the
    // log has to contain everything needed to fix it.
    for m in &mismatches {
        tracing::error!("[image-mismatch] {}", m.describe());
    }
    tracing::error!(
        count = mismatches.len(),
        "translated items have broken images (see [image-mismatch] lines above)"
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
        match &ms[0].problem {
            Problem::Images(d) => assert_eq!(d.missing.len(), 1),
            other => panic!("expected an image diff, got {other:?}"),
        }
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
        match &ms[0].problem {
            Problem::Images(d) => assert_eq!(d.extra.len(), 1),
            other => panic!("expected an image diff, got {other:?}"),
        }
    }

    #[test]
    fn unparseable_screenshot_block_is_reported() {
        // Unescaped apostrophe in the translated alt: the images match
        // (alt isn't compared), but the body no longer evaluates as JS,
        // so sitegen would silently drop the whole page.
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write(
            g,
            "a/items/en/x.md",
            "[app-screenshot-start url='/w'; alt='Advanced options'; title='T' app-screenshot-end]",
        );
        write(
            g,
            "a/items/fr_fr/x.md",
            "[app-screenshot-start url='/w'; alt='Options d'affichage'; title='Le T' app-screenshot-end]",
        );
        write(
            g,
            "a/items/de_de/x.md",
            "[app-screenshot-start url='/w'; alt='Erweiterte Optionen'; title='Das T' app-screenshot-end]",
        );
        let ms = audit(g, &locales_en_fr_de());
        assert_eq!(ms.len(), 1, "{ms:?}");
        assert_eq!(ms[0].locale, "fr_fr");
        assert!(
            matches!(ms[0].problem, Problem::UnparseableScreenshot { .. }),
            "{:?}",
            ms[0].problem
        );
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

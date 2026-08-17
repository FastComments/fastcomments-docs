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
use crate::links::LinkDiff;

/// Why a translated file fails the gate.
#[derive(Debug, Clone)]
pub enum Problem {
    /// Its image references differ from the source's.
    Images(ImageDiff),
    /// Its link targets differ from the source's. URLs are technical
    /// identifiers, so a translation that alters one ships a 404 that
    /// only an external crawl ever notices - see [`crate::links`].
    Links(LinkDiff),
    /// An `[app-screenshot-*]` block doesn't evaluate as JavaScript -
    /// usually an unescaped apostrophe in a translated `title` / `alt`,
    /// which terminates the quoted value early. `sitegen` reacts to
    /// this by logging `skip item ... error=eval app-screenshot config`
    /// and dropping the ENTIRE page from the build, so it has to be a
    /// hard failure here rather than a warning nobody reads.
    UnparseableScreenshot { body: String, error: String },
    /// The "translation" is the English source, unchanged - the model echoed
    /// its input back. Nothing downstream can tell that apart from a good
    /// translation, so the page ships an English description that collides
    /// with the English page and with every other locale that failed the
    /// same way. 13 `meta-desc.txt` files were sitting like this, and the
    /// cache had them stamped as fresh, so nothing was ever going to retry.
    Untranslated,
    /// The translation is not in the locale's writing system - `sr_rs`
    /// (Serbian Cyrillic) coming back in Latin, which makes it byte-identical
    /// to `sr_latn_rs`. See [`crate::script`].
    WrongScript { expected: &'static str },
}

impl Problem {
    /// Short stable label for grouping a large mismatch list by failure mode.
    pub fn class(&self) -> &'static str {
        match self {
            Problem::Images(_) => "images",
            Problem::Links(_) => "link targets",
            Problem::UnparseableScreenshot { .. } => "unparseable app-screenshot",
            Problem::Untranslated => "untranslated (English echoed back)",
            Problem::WrongScript { .. } => "wrong script",
        }
    }

    pub fn describe(&self) -> String {
        match self {
            Problem::Images(diff) => diff.describe(),
            Problem::Links(diff) => diff.describe(),
            Problem::UnparseableScreenshot { body, error } => {
                format!("app-screenshot block does not parse ({error}): [app-screenshot-start{body}app-screenshot-end]")
            }
            Problem::Untranslated => {
                "identical to the English source - the model echoed its input back".to_string()
            }
            Problem::WrongScript { expected } => {
                format!("contains no {expected} characters - translated into the wrong script")
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
pub fn file_problem(source: &str, translated: &str, locale: &str) -> Option<Problem> {
    if let Some(problem) = language_problem(source, translated, locale) {
        return Some(problem);
    }
    if let Some(diff) = image_diff(source, translated) {
        return Some(Problem::Images(diff));
    }
    if let Some(diff) = crate::links::link_diff(source, translated) {
        return Some(Problem::Links(diff));
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

/// A source needs at least this much prose before "did the language actually
/// change?" is answerable. Under it, an unchanged translation is normal: a
/// stub whose only words are product names, or an item that is one code
/// block and a caption, legitimately survives translation byte-for-byte.
const MIN_PROSE_FOR_LANGUAGE_CHECKS: usize = 40;

/// Did the text come back in the wrong language, or not translated at all?
///
/// Deliberately conservative. Both checks are skipped unless the source
/// carries real prose, and skipped entirely for the two cases `run` copies
/// verbatim on purpose (`en_us`, and generated reference indexes - see
/// [`crate::snapshot::source_is_reference_index`]). A false positive here is
/// not a cosmetic bug: it queues a re-translation that can never succeed, on
/// every build, forever.
fn language_problem(source: &str, translated: &str, locale: &str) -> Option<Problem> {
    if locale == "en_us" || crate::snapshot::source_is_reference_index(source) {
        return None;
    }
    if prose_only(source).chars().count() < MIN_PROSE_FOR_LANGUAGE_CHECKS {
        return None;
    }
    // Compare normalized: translated files come back wrapped in `---` fences
    // and re-wrapped inconsistently, so raw equality would miss the echo.
    if normalized(translated) == normalized(source) {
        return Some(Problem::Untranslated);
    }
    let script = crate::script::expected_for(locale)?;
    if !script.present_in(&prose_only(translated)) {
        return Some(Problem::WrongScript {
            expected: script.name,
        });
    }
    None
}

/// `text` with the `---` fences translators add and all whitespace runs
/// collapsed, so two spellings of the same content compare equal.
fn normalized(text: &str) -> String {
    text.lines()
        .filter(|l| !FENCE_LINE.is_match(l))
        .flat_map(|l| l.split_whitespace())
        .collect::<Vec<_>>()
        .join(" ")
}

/// `text` with everything that survives translation unchanged removed: code
/// fences, marker blocks, inline code, HTML tags, and URLs.
///
/// What is left is the natural-language text, which is the only part either
/// language check can say anything about. Without this, a page that is mostly
/// `curl` examples looks like untranslated English no matter how good the
/// translation of its two prose sentences is.
fn prose_only(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_fence = false;
    let mut in_marker = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        // Marker blocks span lines: `[inline-code-start]` .. `[inline-code-end]`.
        if MARKER_START.is_match(trimmed) {
            in_marker = true;
        }
        let marker_ends_here = MARKER_END.is_match(trimmed);
        if in_marker {
            if marker_ends_here {
                in_marker = false;
            }
            continue;
        }
        if FENCE_LINE.is_match(trimmed) {
            continue;
        }
        let stripped = INLINE_NOISE.replace_all(trimmed, " ");
        out.push_str(stripped.trim());
        out.push(' ');
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

static FENCE_LINE: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"^\s*-{3,}\s*$").expect("regex"));

static MARKER_START: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"\[[a-z-]+-start\b").expect("regex"));

static MARKER_END: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"[a-z-]+-end\]").expect("regex"));

/// Inline code spans, HTML tags, and bare URLs - none of them translated.
static INLINE_NOISE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
    regex::Regex::new(r"`[^`]*`|<[^>]+>|https?://\S+").expect("regex")
});

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
                if let Some(problem) = file_problem(&source, &translated, locale) {
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

/// Rewrite every translated item's `[app-screenshot-*]` blocks and
/// `<img src>` values from its source (see
/// [`crate::images::merge_screenshot_blocks`] and
/// [`crate::images::merge_image_srcs`]) and report how many files changed.
///
/// `trans run` does this to fresh LLM output, so this is for the
/// back-catalog: translations written before the merge existed still
/// carry corrupted URLs and unescaped apostrophes, and translations of a
/// generated item still carry whatever `src` the source had when they were
/// written. Idempotent - a second run changes nothing.
fn fix_translated_files(guides_dir: &Path, locales: &Locales) -> Result<usize> {
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
            let has_screenshots = !crate::images::extract_screenshot_bodies(&source).is_empty();
            let has_html_images = source.contains("<img");
            let has_links = !crate::links::extract_link_urls(&source).is_empty();
            if !has_screenshots && !has_html_images && !has_links {
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
                let merged = crate::images::merge_image_srcs(&source, &merged);
                let merged = crate::links::merge_link_urls(&source, &merged);
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

/// Which class of problem a gate invocation reports. Both share one walker
/// and one definition of "broken" ([`file_problem`]), so `check` and `run`
/// stay in agreement with them; they differ only in what they print and
/// what fails the build, which keeps each build.sh phase's failure message
/// pointing at the thing that actually broke.
#[derive(Clone, Copy, PartialEq)]
pub enum Gate {
    Images,
    Links,
}

impl Gate {
    fn covers(&self, problem: &Problem) -> bool {
        match (self, problem) {
            (Gate::Images, Problem::Images(_) | Problem::UnparseableScreenshot { .. }) => true,
            // Count mismatches still re-translate (they reach `run` through
            // `file_problem`), they just don't fail the build.
            (Gate::Links, Problem::Links(diff)) => diff.has_altered_target(),
            _ => false,
        }
    }

    fn tag(&self) -> &'static str {
        match self {
            Gate::Images => "image-mismatch",
            Gate::Links => "link-mismatch",
        }
    }

    fn ok_message(&self) -> &'static str {
        match self {
            Gate::Images => "image parity OK - every translated item has the same images as its source, and every app-screenshot block parses",
            Gate::Links => "link parity OK - every translated item links exactly where its source links",
        }
    }

    fn fail_message(&self) -> &'static str {
        match self {
            Gate::Images => "translated items have broken images (see [image-mismatch] lines above)",
            Gate::Links => "translated items link somewhere their source does not (see [link-mismatch] lines above)",
        }
    }
}

/// Subcommand entry point. Exits non-zero (failing the build) when any
/// translated file fails `gate`.
///
/// `--fix` first restores every translated file's `[app-screenshot-*]`
/// blocks, `<img src>` values, and link targets from its source. That is a
/// maintenance action, never part of the build: build.sh calls this with no
/// arguments so it only ever reports.
pub async fn run_with<I: IntoIterator<Item = String>>(gate: Gate, args: I) -> Result<()> {
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
        let n = fix_translated_files(&guides_dir, &locales)?;
        info!(files = n, "restored images and link targets from source");
    }

    let mismatches: Vec<Mismatch> = audit(&guides_dir, &locales)
        .into_iter()
        .filter(|m| gate.covers(&m.problem))
        .collect();
    if mismatches.is_empty() {
        info!("{}", gate.ok_message());
        return Ok(());
    }
    // Print all of them, not a sample: this fails the build, so the
    // log has to contain everything needed to fix it.
    for m in &mismatches {
        tracing::error!("[{}] {}", gate.tag(), m.describe());
    }
    tracing::error!(count = mismatches.len(), "{}", gate.fail_message());
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

    /// The exact file that shipped on docs.fastcomments.com: the model
    /// handed back its input, the cache stamped it fresh, and three URLs
    /// carried the same English meta description.
    const GATSBY_EN: &str = "Add comments to a Gatsby site. A complete working example \
                             project you can copy, showing where the widget goes and how \
                             to pass the page ID.";

    #[test]
    fn echoed_source_is_untranslated() {
        assert!(matches!(
            file_problem(GATSBY_EN, GATSBY_EN, "ja_jp"),
            Some(Problem::Untranslated)
        ));
    }

    #[test]
    fn echoed_source_is_caught_through_the_fence_wrapper() {
        let wrapped = format!("---\n{GATSBY_EN}\n---\n");
        assert!(matches!(
            file_problem(GATSBY_EN, &wrapped, "bg_bg"),
            Some(Problem::Untranslated)
        ));
    }

    #[test]
    fn a_real_translation_passes() {
        let ja = "Gatsby サイトにコメントを追加します。ウィジェットを配置する場所と\
                  ページ ID の渡し方を示す、コピーできる完全な動作サンプルプロジェクトです。";
        assert!(file_problem(GATSBY_EN, ja, "ja_jp").is_none());
    }

    #[test]
    fn serbian_latin_output_fails_for_the_cyrillic_locale() {
        let latin = "Dodajte komentare na Gatsby sajt. Potpuni radni primer projekta koji \
                     možete kopirati, pokazujući gde se postavlja widget.";
        assert!(matches!(
            file_problem(GATSBY_EN, latin, "sr_rs"),
            Some(Problem::WrongScript { expected: "Cyrillic" })
        ));
        // Same bytes are correct for the Latin locale.
        assert!(file_problem(GATSBY_EN, latin, "sr_latn_rs").is_none());
    }

    #[test]
    fn en_us_is_copied_verbatim_on_purpose() {
        assert!(file_problem(GATSBY_EN, GATSBY_EN, "en_us").is_none());
    }

    #[test]
    fn a_source_without_prose_is_not_judged() {
        // All code and identifiers: an unchanged translation is correct here,
        // and flagging it would queue a re-translation that never converges.
        let src = "```bash\nnpm i fastcomments\n```\n";
        assert!(file_problem(src, src, "ja_jp").is_none());
    }

    #[test]
    fn prose_is_measured_with_code_removed() {
        // Enough bytes overall, but almost all of it is a fenced command.
        let src = "Run it.\n\n```bash\ncurl -X POST https://example.com/api/v1/comments\n```\n";
        assert!(file_problem(src, src, "ja_jp").is_none());
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

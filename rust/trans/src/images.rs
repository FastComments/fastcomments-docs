//! Image parity between a default-locale source file and its
//! translations.
//!
//! An LLM translating markdown occasionally drops an `<img>`, emits it
//! twice, "translates" a path inside `src=`, or mangles the attribute
//! separators in an `[app-screenshot-*]` block. None of those are
//! caught by the inline-code count check, and all of them silently
//! change what the rendered page shows (a dropped screenshot, a 404
//! image, or - for a malformed screenshot config - `build.rs` logging
//! "skip malformed app-screenshot config" and rendering nothing).
//!
//! This module extracts every image reference from a markdown item and
//! compares the translated file's multiset against the source's.
//! Three reference kinds are recognized:
//!
//!   * `<img ... src="...">` - only the `src` is compared. `alt` and
//!     `title` are natural language and SHOULD be translated. An
//!     `<img>` with no `src` at all (e.g. the literal `` `<img>` ``
//!     mentioned in prose or inside a code sample) is not an image
//!     reference and is ignored.
//!   * `![alt](url)` - only the URL is compared, same reasoning.
//!   * `[app-screenshot-start ... app-screenshot-end]` - every
//!     attribute except `title` is compared, since `url`,
//!     `selector`, and `clickSelector(s)` determine which screenshot
//!     gets captured.
//!
//! Comparison is an order-insensitive multiset so a legitimate
//! re-ordering doesn't fail, while a drop / duplicate / edit does.

use once_cell::sync::Lazy;
use regex::Regex;

/// One image reference found in a markdown item, normalized down to
/// the parts that must survive translation byte-for-byte.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImageRef {
    /// `<img ... src="X" ...>` - X.
    Html(String),
    /// `![alt](X)` - X.
    Markdown(String),
    /// `[app-screenshot-start ... app-screenshot-end]` - the attribute
    /// list minus `title`, normalized by [`normalize_screenshot_attrs`].
    Screenshot(String),
}

impl ImageRef {
    /// Human-readable form used in build logs + LLM repair prompts.
    /// Deliberately looks like the source syntax so a translator model
    /// reading the repair prompt knows exactly what to put back.
    pub fn describe(&self) -> String {
        match self {
            ImageRef::Html(src) => format!("<img src=\"{src}\">"),
            ImageRef::Markdown(url) => format!("![...]({url})"),
            ImageRef::Screenshot(attrs) => {
                format!("[app-screenshot-start {attrs} app-screenshot-end]")
            }
        }
    }
}

static IMG_TAG_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<img\b[^>]*>").expect("img tag regex"));
static IMG_SRC_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?is)\bsrc\s*=\s*(?:"([^"]*)"|'([^']*)')"#).expect("img src regex")
});
static MD_IMAGE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"!\[[^\]]*\]\(\s*([^)\s]+)").expect("markdown image regex"));
static SCREENSHOT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?s)\[app-screenshot-start(.*?)app-screenshot-end\]").expect("screenshot regex")
});

/// Extract every image reference from `content`, sorted so two files
/// can be compared as multisets.
pub fn extract_image_refs(content: &str) -> Vec<ImageRef> {
    let mut out = Vec::new();
    for tag in IMG_TAG_RE.find_iter(content) {
        // No `src` -> not an image reference. `<img>` shows up in
        // prose ("either an `<img>` element directly") and inside
        // code samples, where translations legitimately add or
        // remove mentions.
        if let Some(caps) = IMG_SRC_RE.captures(tag.as_str()) {
            let src = caps
                .get(1)
                .or_else(|| caps.get(2))
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();
            out.push(ImageRef::Html(src));
        }
    }
    for caps in MD_IMAGE_RE.captures_iter(content) {
        out.push(ImageRef::Markdown(caps[1].to_string()));
    }
    for caps in SCREENSHOT_RE.captures_iter(content) {
        out.push(ImageRef::Screenshot(normalize_screenshot_attrs(&caps[1])));
    }
    out.sort();
    out
}

/// Normalize an `[app-screenshot-*]` attribute body for comparison:
/// split on top-level `;`, drop the `title` attribute (the one field
/// that's meant to be translated), whitespace-normalize each remaining
/// `key = 'value'` pair, and sort.
///
/// Splitting is quote-aware so a translated `title` containing a `;`
/// doesn't shear the token stream and produce a phantom mismatch.
/// A token that doesn't parse as `key = value` is kept verbatim - that
/// is exactly the malformed case worth failing on (a missing `;`
/// separator swallows the next attribute into the previous value, and
/// `build.rs` then skips the screenshot entirely).
pub fn normalize_screenshot_attrs(body: &str) -> String {
    let mut parts: Vec<String> = Vec::new();
    for token in split_top_level_semicolons(body) {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        match token.split_once('=') {
            Some((key, value)) if key.trim().eq_ignore_ascii_case("title") => {
                let _ = value; // translated on purpose; never compared
            }
            Some((key, value)) => {
                parts.push(format!(
                    "{}={}",
                    key.trim(),
                    value.split_whitespace().collect::<Vec<_>>().join(" ")
                ));
            }
            None => parts.push(token.split_whitespace().collect::<Vec<_>>().join(" ")),
        }
    }
    parts.sort();
    parts.join("; ")
}

/// Split on `;` that are not inside a single-quoted value. Backslash
/// escapes (`\'`, which `sanitize_inline_code_attrs` emits for
/// apostrophes in translated titles) do not close the quote.
fn split_top_level_semicolons(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut escaped = false;
    for ch in body.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        match ch {
            '\\' if in_quote => {
                current.push(ch);
                escaped = true;
            }
            '\'' => {
                in_quote = !in_quote;
                current.push(ch);
            }
            ';' if !in_quote => {
                out.push(std::mem::take(&mut current));
            }
            _ => current.push(ch),
        }
    }
    out.push(current);
    out
}

/// What changed between the source's images and the translation's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageDiff {
    /// Present in the source, absent (or fewer copies) in the translation.
    pub missing: Vec<ImageRef>,
    /// Present in the translation but not in the source (or duplicated).
    pub extra: Vec<ImageRef>,
}

impl ImageDiff {
    /// One-line summary for build logs.
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();
        if !self.missing.is_empty() {
            parts.push(format!(
                "missing {}",
                self.missing
                    .iter()
                    .map(ImageRef::describe)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if !self.extra.is_empty() {
            parts.push(format!(
                "unexpected {}",
                self.extra
                    .iter()
                    .map(ImageRef::describe)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        parts.join("; ")
    }
}

/// Compare a translated file against its default-locale source.
/// `None` means the images match exactly (as a multiset).
pub fn image_diff(source: &str, translated: &str) -> Option<ImageDiff> {
    diff_refs(extract_image_refs(source), extract_image_refs(translated))
}

/// Multiset difference over two already-sorted `ImageRef` lists.
fn diff_refs(mut expected: Vec<ImageRef>, mut actual: Vec<ImageRef>) -> Option<ImageDiff> {
    if expected == actual {
        return None;
    }
    // Both are sorted by extract_image_refs; a merge walk gives the
    // multiset difference in one pass and keeps duplicates counted.
    let mut missing = Vec::new();
    let mut extra = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);
    while i < expected.len() && j < actual.len() {
        match expected[i].cmp(&actual[j]) {
            std::cmp::Ordering::Equal => {
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Less => {
                missing.push(expected[i].clone());
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                extra.push(actual[j].clone());
                j += 1;
            }
        }
    }
    missing.extend(expected.drain(i..));
    extra.extend(actual.drain(j..));
    if missing.is_empty() && extra.is_empty() {
        return None;
    }
    Some(ImageDiff { missing, extra })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_content_has_no_diff() {
        let s = "<img class=\"screenshot-image\" src=\"/images/a.png\" alt=\"A\" />";
        assert_eq!(image_diff(s, s), None);
    }

    #[test]
    fn translated_alt_text_is_not_a_mismatch() {
        // The real fr_fr affiliates/dashboard.md shape: same src,
        // translated alt. Must pass.
        let en = "<img class=\"screenshot-image\" src=\"/images/affiliates-dashboard.png\" alt=\"Affiliates Dashboard\" />";
        let fr = "<img class=\"screenshot-image\" src=\"/images/affiliates-dashboard.png\" alt=\"Tableau de bord des affiliés\" />";
        assert_eq!(image_diff(en, fr), None);
    }

    #[test]
    fn dropped_image_is_missing() {
        let en = "a\n<img src=\"/images/a.png\">\nb\n<img src=\"/images/b.png\">";
        let fr = "a\n<img src=\"/images/a.png\">\nb";
        let d = image_diff(en, fr).expect("mismatch expected");
        assert_eq!(d.missing, vec![ImageRef::Html("/images/b.png".into())]);
        assert!(d.extra.is_empty());
    }

    #[test]
    fn duplicated_image_is_extra() {
        // The real ja_jp installation-clickfunnels bug: the model
        // emitted every image twice.
        let en = "<img src=\"/images/a.png\">";
        let ja = "<img src=\"/images/a.png\">\n<img src=\"/images/a.png\">";
        let d = image_diff(en, ja).expect("mismatch expected");
        assert!(d.missing.is_empty());
        assert_eq!(d.extra, vec![ImageRef::Html("/images/a.png".into())]);
    }

    #[test]
    fn translated_src_path_is_flagged() {
        let en = "<img src=\"/images/installation-guides/step-1.png\">";
        let de = "<img src=\"/images/installations-anleitungen/schritt-1.png\">";
        let d = image_diff(en, de).expect("mismatch expected");
        assert_eq!(d.missing.len(), 1);
        assert_eq!(d.extra.len(), 1);
    }

    #[test]
    fn img_without_src_is_ignored() {
        // The real image-chat/configuration-options.md false positive:
        // `<img>` mentioned in prose, plus a code comment in the
        // translation that mentions it again.
        let en = "The first parameter can be either an `<img>` element or a container.";
        let fr = "Le premier paramètre peut être un élément `<img>`.\n// Élément <img> direct";
        assert_eq!(image_diff(en, fr), None);
    }

    #[test]
    fn markdown_image_url_is_compared_but_alt_is_not() {
        let en = "![npm](https://img.shields.io/npm/v/x?logo=npm)";
        let fr = "![paquet npm](https://img.shields.io/npm/v/x?logo=npm)";
        assert_eq!(image_diff(en, fr), None);

        let bad = "![npm](https://img.shields.io/npm/v/y?logo=npm)";
        assert!(image_diff(en, bad).is_some());
    }

    #[test]
    fn screenshot_title_may_be_translated() {
        let en = "[app-screenshot-start url='/auth/import'; selector = '.content'; title='Import Job Status' app-screenshot-end]";
        let bg = "[app-screenshot-start url='/auth/import'; selector = '.content'; title='Статус на задачата' app-screenshot-end]";
        assert_eq!(image_diff(en, bg), None);
    }

    #[test]
    fn screenshot_missing_semicolon_separator_is_flagged() {
        // The real bg_bg migrations/importing-data.md bug: the `;`
        // between `selector` and `title` was dropped, so the whole
        // attrs block fails to parse at build time and the screenshot
        // silently disappears.
        let en = "[app-screenshot-start url='/auth/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]";
        let bg = "[app-screenshot-start url='/auth/import?demo=true'; selector = '.content' title='Статус на задачата' app-screenshot-end]";
        assert!(image_diff(en, bg).is_some());
    }

    #[test]
    fn screenshot_url_change_is_flagged() {
        let en = "[app-screenshot-start url='/auth/import'; selector = '.content'; title='T' app-screenshot-end]";
        let de = "[app-screenshot-start url='/auth/importieren'; selector = '.content'; title='T' app-screenshot-end]";
        assert!(image_diff(en, de).is_some());
    }

    #[test]
    fn screenshot_attr_reordering_is_not_a_mismatch() {
        let en = "[app-screenshot-start url='/a'; selector = '.b'; title='T' app-screenshot-end]";
        let fr = "[app-screenshot-start selector = '.b'; url='/a'; title='Le T' app-screenshot-end]";
        assert_eq!(image_diff(en, fr), None);
    }

    #[test]
    fn semicolon_inside_translated_title_does_not_shear_tokens() {
        let en = "[app-screenshot-start url='/a'; selector = '.b'; title='Options' app-screenshot-end]";
        let fr = "[app-screenshot-start url='/a'; selector = '.b'; title='Options; avancées' app-screenshot-end]";
        assert_eq!(image_diff(en, fr), None);
    }

    #[test]
    fn escaped_apostrophe_in_title_does_not_break_quote_tracking() {
        let en = "[app-screenshot-start url='/a'; selector = '.b'; title='Usage Example' app-screenshot-end]";
        let fr = "[app-screenshot-start url='/a'; selector = '.b'; title='Exemple d\\'utilisation; suite' app-screenshot-end]";
        assert_eq!(image_diff(en, fr), None);
    }

    #[test]
    fn dropped_screenshot_is_missing() {
        let en = "[app-screenshot-start url='/a'; title='A' app-screenshot-end]\n[app-screenshot-start url='/b'; title='B' app-screenshot-end]";
        let fr = "[app-screenshot-start url='/a'; title='A' app-screenshot-end]";
        let d = image_diff(en, fr).expect("mismatch expected");
        assert_eq!(d.missing, vec![ImageRef::Screenshot("url='/b'".into())]);
    }

    #[test]
    fn ordering_of_images_does_not_matter() {
        let en = "<img src=\"/a.png\">\n<img src=\"/b.png\">";
        let fr = "<img src=\"/b.png\">\n<img src=\"/a.png\">";
        assert_eq!(image_diff(en, fr), None);
    }

    #[test]
    fn single_quoted_src_is_handled() {
        let en = "<img src='/a.png'>";
        let fr = "<img src=\"/a.png\">";
        assert_eq!(image_diff(en, fr), None);
    }

    #[test]
    fn image_invented_by_the_translator_is_flagged() {
        // Source has no images at all. The gate still has to catch a
        // translation that hallucinates one, so neither the audit nor
        // the task discovery may short-circuit on "source has no
        // images".
        let en = "Just prose, no pictures.";
        let fr = "Que du texte.\n<img src=\"/i/made-up.png\">";
        let d = image_diff(en, fr).expect("mismatch expected");
        assert!(d.missing.is_empty());
        assert_eq!(d.extra, vec![ImageRef::Html("/i/made-up.png".into())]);
    }

    #[test]
    fn describe_lists_both_sides() {
        let en = "<img src=\"/a.png\">";
        let fr = "<img src=\"/b.png\">";
        let d = image_diff(en, fr).unwrap();
        let s = d.describe();
        assert!(s.contains("missing <img src=\"/a.png\">"), "{s}");
        assert!(s.contains("unexpected <img src=\"/b.png\">"), "{s}");
    }
}

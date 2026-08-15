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

/// Every `[app-screenshot-*]` attribute body in `content`, in document
/// order and unmodified. Used by the gate to check each one still
/// evaluates as JavaScript, and by [`merge_screenshot_blocks`].
pub fn extract_screenshot_bodies(content: &str) -> Vec<String> {
    SCREENSHOT_RE
        .captures_iter(content)
        .map(|c| c[1].to_string())
        .collect()
}

/// Every `<img src>` value in `content`, in document order. `<img>` tags
/// without a `src` are skipped, matching [`extract_image_refs`].
fn html_img_srcs(content: &str) -> Vec<String> {
    IMG_TAG_RE
        .find_iter(content)
        .filter_map(|tag| IMG_SRC_RE.captures(tag.as_str()))
        .map(|caps| {
            caps.get(1)
                .or_else(|| caps.get(2))
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default()
        })
        .collect()
}

/// Rewrite every `<img src>` in `translated` to the value at the same
/// position in `source`, leaving `alt` / `title` and the surrounding prose
/// untouched.
///
/// A `src` is a technical identifier that must match the source byte-for-byte
/// (translation rule 11), so when the source's path changes - `sdkgen` now
/// rewrites relative README `<img src>` to a local copy under
/// `images/sdk-images/` - every locale has to follow. Re-translating 23 files
/// to move a path is both expensive and a fresh chance for the model to drift,
/// and the parity gate would otherwise fail the build until it happened.
///
/// If the counts don't match, the translation dropped or duplicated an image;
/// there's no safe pairing, so leave it alone and let the gate report it.
pub fn merge_image_srcs(source: &str, translated: &str) -> String {
    let source_srcs = html_img_srcs(source);
    if source_srcs.is_empty() || source_srcs.len() != html_img_srcs(translated).len() {
        return translated.to_string();
    }
    let mut i = 0usize;
    IMG_TAG_RE
        .replace_all(translated, |caps: &regex::Captures| {
            let tag = &caps[0];
            let Some(src_caps) = IMG_SRC_RE.captures(tag) else {
                return tag.to_string();
            };
            let want = &source_srcs[i];
            i += 1;
            let found = src_caps.get(1).or_else(|| src_caps.get(2));
            // Already correct: return verbatim so attribute spacing survives
            // and a second `--fix` run is a no-op.
            if found.is_some_and(|m| m.as_str().trim() == want) {
                return tag.to_string();
            }
            let quote = if src_caps.get(1).is_some() { '"' } else { '\'' };
            IMG_SRC_RE
                .replace(tag, |_: &regex::Captures| format!("src={quote}{want}{quote}"))
                .into_owned()
        })
        .into_owned()
}

static TRANSLATABLE_ATTR_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?is)^(\s*(title|alt)\s*=\s*)'([\s\S]*)'(\s*)$").expect("attr regex")
});

/// Rebuild every `[app-screenshot-*]` block in `translated` from the
/// corresponding block in `source`, carrying over ONLY the translated
/// `title` / `alt` values (with apostrophes escaped).
///
/// Nothing else in these blocks is translatable - `url`, `selector`,
/// `clickSelector(s)`, `actions`, `cacheBuster`, `linkUrl` are all
/// technical - yet the translator rewrites the whole block, and in
/// practice it corrupts them: it has silently dropped a word from the
/// English sample text inside a percent-encoded `url=` query string
/// (11 locales of mentions-notifications.md), replaced a 2KB fixture
/// URL with `'...'` (moderation-via-email.md), and left apostrophes
/// unescaped in translated Hebrew `alt=` text, which makes the body
/// fail to parse and costs the ENTIRE page (`sitegen` logs `skip item
/// ... error=eval app-screenshot config`). Prompt rules asking for all
/// three did not prevent any of them.
///
/// So don't ask - reconstruct. This is the same class of deterministic
/// post-processing as `sanitize_inline_code_attrs`, applied to the
/// marker whose body has the tightest contract.
///
/// If the block counts don't match, the translation dropped or
/// duplicated a screenshot; there's no safe pairing, so leave it alone
/// and let the parity gate report it.
pub fn merge_screenshot_blocks(source: &str, translated: &str) -> String {
    let source_bodies = extract_screenshot_bodies(source);
    if source_bodies.is_empty() || source_bodies.len() != extract_screenshot_bodies(translated).len()
    {
        return translated.to_string();
    }
    let mut i = 0usize;
    SCREENSHOT_RE
        .replace_all(translated, |caps: &regex::Captures| {
            let merged = merge_one_screenshot(&source_bodies[i], &caps[1]);
            i += 1;
            format!("[app-screenshot-start{merged}app-screenshot-end]")
        })
        .into_owned()
}

fn merge_one_screenshot(source_body: &str, translated_body: &str) -> String {
    // The translated body may be unparseable (that's half the point),
    // so split it naively on "; " rather than with the quote-aware
    // splitter - an unescaped apostrophe would desync quote tracking.
    // A translated value containing "; " simply won't match, and the
    // source's value is kept; that's lossy in a way nobody has hit,
    // and it's always valid.
    let mut translated_attrs: Vec<(String, String)> = Vec::new();
    for token in translated_body.split("; ") {
        if let Some(m) = TRANSLATABLE_ATTR_RE.captures(token) {
            translated_attrs.push((m[2].to_ascii_lowercase(), escape_apostrophes(&m[3])));
        }
    }
    // Rebuild from the SOURCE token list: same order, same separators,
    // same technical attributes, byte for byte.
    let rebuilt: Vec<String> = split_top_level_semicolons(source_body)
        .iter()
        .map(|token| match TRANSLATABLE_ATTR_RE.captures(token) {
            Some(m) => {
                let key = m[2].to_ascii_lowercase();
                match translated_attrs.iter().find(|(k, _)| *k == key) {
                    Some((_, value)) => format!("{}'{}'{}", &m[1], value, &m[4]),
                    None => token.clone(),
                }
            }
            None => token.clone(),
        })
        .collect();
    rebuilt.join(";")
}

/// Escape unescaped `'` as `\'`, leaving already-escaped ones alone.
/// Same NUL-placeholder trick as `run::sanitize_inline_code_attrs`.
fn escape_apostrophes(value: &str) -> String {
    value
        .replace("\\'", "\u{0000}")
        .replace('\'', "\\'")
        .replace('\u{0000}', "\\'")
}

/// Normalize an `[app-screenshot-*]` attribute body for comparison:
/// split on top-level `;`, drop the `title` and `alt` attributes (the
/// two fields that are meant to be translated), whitespace-normalize
/// each remaining `key = 'value'` pair, and sort.
///
/// Splitting is quote-aware so a translated `title` or `alt` containing
/// a `;` doesn't shear the token stream and produce a phantom mismatch.
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
            Some((key, value))
                if key.trim().eq_ignore_ascii_case("title")
                    || key.trim().eq_ignore_ascii_case("alt") =>
            {
                let _ = value; // translated on purpose; never compared
            }
            Some((key, value)) => {
                parts.push(format!(
                    "{}={}",
                    key.trim(),
                    normalize_value(value)
                ));
            }
            None => parts.push(normalize_value(token)),
        }
    }
    parts.sort();
    parts.join("; ")
}

/// Whitespace-normalize an attribute value, then undo `\"` escaping.
///
/// The attrs body is evaluated as JavaScript, where `'a\"b'` and `'a"b'`
/// are the same string - a translator that adds those backslashes has
/// changed nothing the browser will see, so failing the build over it
/// would be a false positive. (`\'` is NOT normalized: inside a
/// single-quoted value the backslash is what keeps the string from
/// terminating early, so removing it would change meaning.)
fn normalize_value(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace("\\\"", "\"")
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
    fn screenshot_alt_may_be_translated() {
        let en = "[app-screenshot-start url='/auth/import'; selector = '.content'; alt='Import page with the provider field'; title='Import Job Status' app-screenshot-end]";
        let bg = "[app-screenshot-start url='/auth/import'; selector = '.content'; alt='Страница за импортиране'; title='Статус на задачата' app-screenshot-end]";
        assert_eq!(image_diff(en, bg), None);
    }

    #[test]
    fn screenshot_dropped_alt_is_not_a_mismatch() {
        // `alt` is translated prose, so its presence is never compared.
        // A url change in the same block must still be caught.
        let en = "[app-screenshot-start url='/a'; selector = '.b'; alt='A'; title='T' app-screenshot-end]";
        let fr = "[app-screenshot-start url='/a'; selector = '.b'; title='Le T' app-screenshot-end]";
        assert_eq!(image_diff(en, fr), None);
        let de = "[app-screenshot-start url='/other'; selector = '.b'; alt='A'; title='T' app-screenshot-end]";
        assert!(image_diff(en, de).is_some());
    }

    #[test]
    fn semicolon_inside_translated_alt_does_not_shear_tokens() {
        let en = "[app-screenshot-start url='/a'; selector = '.b'; alt='Options'; title='T' app-screenshot-end]";
        let fr = "[app-screenshot-start url='/a'; selector = '.b'; alt='Options; avancées'; title='Le T' app-screenshot-end]";
        assert_eq!(image_diff(en, fr), None);
    }

    #[test]
    fn escaped_double_quotes_in_an_attr_value_are_equivalent() {
        // Real he/webhooks-local-dev.md case: the model wrote
        // `input[name=\"comment-created-url\"]` where the source has
        // `input[name="comment-created-url"]`. The attrs body is
        // evaluated as JS, so both produce the same selector string —
        // failing the build over it is a false positive.
        let en = "[app-screenshot-start url='/w'; actions=[{type: 'set-value', selector: 'input[name=\"x\"]', value: 'y'}] app-screenshot-end]";
        let he = "[app-screenshot-start url='/w'; actions=[{type: 'set-value', selector: 'input[name=\\\"x\\\"]', value: 'y'}] app-screenshot-end]";
        assert_eq!(image_diff(en, he), None);
    }

    #[test]
    fn merge_image_srcs_realigns_translated_srcs_and_keeps_alt() {
        // Real lib-react-native-sdk case: sdkgen started rewriting the
        // README's relative `<img src>` to a local copy, so every locale's
        // stale path has to follow without re-translating the prose.
        let en = "<img src=\"images/sdk-images/a.png\" width=\"260\" alt=\"Light\"/>";
        let fr = "<img src=\"./demo-screenshots/light.png\" width=\"260\" alt=\"Clair\"/>";
        let merged = merge_image_srcs(en, fr);
        assert_eq!(
            merged,
            "<img src=\"images/sdk-images/a.png\" width=\"260\" alt=\"Clair\"/>"
        );
        assert_eq!(image_diff(en, &merged), None);
        assert_eq!(merge_image_srcs(en, &merged), merged, "not idempotent");
    }

    #[test]
    fn merge_image_srcs_keeps_quote_style_and_skips_srcless_imgs() {
        let en = "Use `<img>`.\n<img src='/i/a.png'>\n<img src=\"/i/b.png\">";
        let de = "Nutze `<img>`.\n<img src='/alt/a.png'>\n<img src=\"/alt/b.png\">";
        assert_eq!(
            merge_image_srcs(en, de),
            "Nutze `<img>`.\n<img src='/i/a.png'>\n<img src=\"/i/b.png\">"
        );
    }

    #[test]
    fn merge_image_srcs_leaves_mismatched_counts_for_the_gate() {
        // A dropped image has no safe pairing - the gate must report it.
        let en = "<img src=\"/i/a.png\">\n<img src=\"/i/b.png\">";
        let ja = "<img src=\"/old/a.png\">";
        assert_eq!(merge_image_srcs(en, ja), ja);
        assert!(image_diff(en, ja).is_some());
    }

    #[test]
    fn merge_image_srcs_leaves_files_without_html_images_alone() {
        let en = "![npm](https://img.shields.io/npm/v/x)";
        let fr = "![npm](https://img.shields.io/npm/v/x)";
        assert_eq!(merge_image_srcs(en, fr), fr);
    }

    #[test]
    fn truncated_url_is_flagged() {
        // Real fr_fr/moderation-via-email.md case: the model replaced a
        // very long test-fixture URL with '...'. The screenshot would
        // point at a nonexistent page.
        let en = "[app-screenshot-start url='/test-e2e/email/digest?stats=%7B%22a%22%3A1%7D'; selector='.content'; title='T' app-screenshot-end]";
        let fr = "[app-screenshot-start url='...'; selector='.content'; title='Le T' app-screenshot-end]";
        assert!(image_diff(en, fr).is_some());
    }

    #[test]
    fn text_edited_inside_a_url_is_flagged() {
        // Real mentions-notifications.md case across 11 locales: the
        // model "fixed" the English sample text embedded in a
        // percent-encoded query parameter, dropping `you%20`.
        let en = "[app-screenshot-start url='/e?comment=Hey%20I%20wanted%20you%20to%20see%20this.'; title='T' app-screenshot-end]";
        let fr = "[app-screenshot-start url='/e?comment=Hey%20I%20wanted%20to%20see%20this.'; title='Le T' app-screenshot-end]";
        assert!(image_diff(en, fr).is_some());
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

    // ---- merge_screenshot_blocks: the three real CI failures ----

    #[test]
    fn merge_restores_a_url_the_translator_edited() {
        // 11 locales of mentions-notifications.md: the model dropped
        // `you%20` from the English sample text inside the query string.
        let en = "[app-screenshot-start url='/e?c=I%20wanted%20you%20to%20see%20this.'; selector='.content'; title='Mention Email' app-screenshot-end]";
        let fr = "[app-screenshot-start url='/e?c=I%20wanted%20to%20see%20this.'; selector='.content'; title='E-mail de mention' app-screenshot-end]";
        let merged = merge_screenshot_blocks(en, fr);
        assert_eq!(image_diff(en, &merged), None, "url restored: {merged}");
        assert!(merged.contains("title='E-mail de mention'"), "{merged}");
    }

    #[test]
    fn merge_restores_a_truncated_url() {
        // moderation-via-email.md: a 2KB fixture URL became '...'.
        let en = "[app-screenshot-start url='/test-e2e/email/digest?stats=%7B%22a%22%3A1%7D'; linkUrl=false; selector = '.content'; alt='Digest email'; title='Digest' app-screenshot-end]";
        let fr = "[app-screenshot-start url='...'; linkUrl=false; selector = '.content'; alt='E-mail de résumé'; title='Résumé' app-screenshot-end]";
        let merged = merge_screenshot_blocks(en, fr);
        assert_eq!(image_diff(en, &merged), None, "{merged}");
        assert!(merged.contains("alt='E-mail de résumé'"), "{merged}");
        assert!(merged.contains("title='Résumé'"), "{merged}");
    }

    #[test]
    fn merge_escapes_apostrophes_in_translated_alt() {
        // 16 he/ + 1 tr_tr/ files: an unescaped apostrophe in the
        // translated alt ends the quoted value early, the body stops
        // parsing as JS, and sitegen drops the whole page.
        let en = "[app-screenshot-start url='/w'; alt='Advanced options'; title='Use Absolute Dates' app-screenshot-end]";
        let he = "[app-screenshot-start url='/w'; alt='אפשרויות הווידג'ט'; title='Use Absolute Dates' app-screenshot-end]";
        let merged = merge_screenshot_blocks(en, he);
        assert!(
            merged.contains(r"alt='אפשרויות הווידג\'ט'"),
            "apostrophe must be escaped: {merged}"
        );
        assert!(merged.contains("url='/w'"), "{merged}");
    }

    #[test]
    fn merge_keeps_already_escaped_apostrophes_single() {
        let en = "[app-screenshot-start url='/w'; title='Usage' app-screenshot-end]";
        let fr = "[app-screenshot-start url='/w'; title='Exemple d\\'utilisation' app-screenshot-end]";
        let merged = merge_screenshot_blocks(en, fr);
        assert!(merged.contains(r"title='Exemple d\'utilisation'"), "{merged}");
        assert!(!merged.contains(r"d\\'utilisation"), "no double-escape: {merged}");
    }

    #[test]
    fn merge_is_a_noop_when_the_translation_is_clean() {
        let en = "text\n[app-screenshot-start url='/w'; selector='.c'; title='T' app-screenshot-end]\nmore";
        let fr = "texte\n[app-screenshot-start url='/w'; selector='.c'; title='Le T' app-screenshot-end]\nplus";
        let merged = merge_screenshot_blocks(en, fr);
        assert_eq!(merged, fr);
    }

    #[test]
    fn merge_preserves_prose_around_the_block() {
        let en = "before\n[app-screenshot-start url='/w'; title='T' app-screenshot-end]\nafter";
        let fr = "avant\n[app-screenshot-start url='/BAD'; title='Le T' app-screenshot-end]\napres";
        let merged = merge_screenshot_blocks(en, fr);
        assert!(merged.starts_with("avant\n"), "{merged}");
        assert!(merged.ends_with("\napres"), "{merged}");
        assert!(merged.contains("url='/w'"), "{merged}");
    }

    #[test]
    fn merge_leaves_a_mismatched_block_count_alone() {
        // Dropped or duplicated screenshot: there's no safe pairing, so
        // the parity gate has to report it rather than us guessing.
        let en = "[app-screenshot-start url='/a'; title='A' app-screenshot-end]\n[app-screenshot-start url='/b'; title='B' app-screenshot-end]";
        let fr = "[app-screenshot-start url='/a'; title='Le A' app-screenshot-end]";
        assert_eq!(merge_screenshot_blocks(en, fr), fr);
        assert!(image_diff(en, fr).is_some(), "gate still catches it");
    }

    #[test]
    fn merge_handles_content_with_no_screenshots() {
        let en = "just prose";
        let fr = "juste du texte";
        assert_eq!(merge_screenshot_blocks(en, fr), fr);
    }

    #[test]
    fn merge_keeps_source_value_when_the_translation_dropped_the_attr() {
        let en = "[app-screenshot-start url='/w'; alt='Advanced options'; title='T' app-screenshot-end]";
        let fr = "[app-screenshot-start url='/w'; title='Le T' app-screenshot-end]";
        let merged = merge_screenshot_blocks(en, fr);
        assert!(merged.contains("alt='Advanced options'"), "{merged}");
        assert!(merged.contains("title='Le T'"), "{merged}");
    }

    #[test]
    fn merge_preserves_multi_attr_ordering_and_spacing() {
        let en = "[app-screenshot-start url='/w'; clickSelectors = ['.a', '.b']; selector = '.c'; alt='A'; title='T' app-screenshot-end]";
        let fr = "[app-screenshot-start title='Le T'; alt='Le A'; url='/WRONG' app-screenshot-end]";
        let merged = merge_screenshot_blocks(en, fr);
        assert_eq!(
            merged,
            "[app-screenshot-start url='/w'; clickSelectors = ['.a', '.b']; selector = '.c'; alt='Le A'; title='Le T' app-screenshot-end]"
        );
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

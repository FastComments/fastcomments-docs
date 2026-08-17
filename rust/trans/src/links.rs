//! Link-target parity between a default-locale source file and its
//! translations.
//!
//! URLs are technical identifiers: a translated page must link exactly where
//! the English page links. The translator does not reliably respect that. In
//! the current content tree it has dropped `_status_` out of the middle of a
//! path (`sdk-nim/ko_kr/updateusernotificationpagesubscriptionstatus`),
//! injected a zero-width space into a hostname (`sdk-nim/he/getv1pagelikes`,
//! which yields `fastcomment\u{200b}s-nim`), and truncated paths down to
//! `docs/Models/` (a dozen locales of the cpp and nim README guides). Every
//! one of those ships a 404 that only an external crawl ever notices.
//!
//! Same lesson, and same shape, as [`crate::images`]: don't ask the prompt,
//! reconstruct deterministically. `run` merges source URLs back into fresh
//! output, and the gate catches the back-catalog.
//!
//! Two reference kinds are compared, both by target only - the link *text* is
//! natural language and SHOULD be translated:
//!
//!   * `[text](url)` - markdown links. Images (`![alt](url)`) are excluded:
//!     they belong to [`crate::images`], and reporting them here too would
//!     double up on every finding.
//!   * `href="url"` - guides hand-write `<a href>` wherever markdown can't
//!     express `target="_blank"`.
//!
//! Comparison is order-SENSITIVE, unlike the image multiset. Link targets are
//! positional here: the merge restores the i-th target from the i-th source
//! target, so a reordering would silently swap two links' destinations.
//!
//! Fenced code blocks, `[inline-code-*]` bodies, and `[app-screenshot-*]`
//! bodies are masked out first. The lib-hugo / lib-11ty / lib-jekyll guides
//! document markdown link syntax as sample user markup, and screenshot bodies
//! carry `url=` attributes that [`crate::images`] already owns.

use once_cell::sync::Lazy;
use regex::Regex;

/// Markdown link, excluding images. The leading group is the character before
/// `[`, which must not be `!`.
///
/// The label allows one level of nested brackets. Generated SDK pages are full
/// of ``[`Option[T]`](url)`` and ``[`[T]`](url)``, and a label pattern that
/// stops at the first `]` matches none of them - which is exactly how the
/// zero-width space in `sdk-nim/he/getv1pagelikes` survived a first pass at
/// this check.
static MD_LINK_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(^|[^!])(\[(?:[^\[\]\n]|\[[^\[\]\n]*\])*\]\()([^)\s]+)(\))")
        .expect("md link regex")
});

/// `href="..."` / `href='...'`.
static HREF_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)(href\s*=\s*)("([^"]*)"|'([^']*)')"#).expect("href regex"));

static FENCE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?ms)^(```|~~~).*?^(```|~~~)").expect("fence regex"));
static INLINE_CODE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?s)\[inline-code-start\].*?\[inline-code-end\]").expect("inline code regex")
});
static SCREENSHOT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?s)\[app-screenshot-start.*?app-screenshot-end\]").expect("screenshot regex")
});

/// Byte ranges whose contents are not prose and must not be compared.
fn masked_ranges(content: &str) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for re in [&*FENCE_RE, &*INLINE_CODE_RE, &*SCREENSHOT_RE] {
        for m in re.find_iter(content) {
            ranges.push((m.start(), m.end()));
        }
    }
    ranges.sort_unstable();
    ranges
}

fn is_masked(ranges: &[(usize, usize)], at: usize) -> bool {
    ranges.iter().any(|(s, e)| at >= *s && at < *e)
}

/// Every link target in `content`, in source order.
pub fn extract_link_urls(content: &str) -> Vec<String> {
    let masked = masked_ranges(content);
    let mut out: Vec<(usize, String)> = Vec::new();
    for caps in MD_LINK_RE.captures_iter(content) {
        let url = caps.get(3).expect("url group");
        if !is_masked(&masked, url.start()) {
            out.push((url.start(), url.as_str().to_string()));
        }
    }
    for caps in HREF_RE.captures_iter(content) {
        let value = caps.get(3).or_else(|| caps.get(4)).expect("href value");
        if !is_masked(&masked, value.start()) {
            out.push((value.start(), value.as_str().to_string()));
        }
    }
    out.sort_by_key(|(at, _)| *at);
    out.into_iter().map(|(_, u)| u).collect()
}

/// A translated file whose link targets don't match its source's.
#[derive(Debug, Clone)]
pub struct LinkDiff {
    pub expected: Vec<String>,
    pub actual: Vec<String>,
}

impl LinkDiff {
    /// True when the translation links to a target the source never had.
    ///
    /// This is the class that ships a 404, and after `merge_link_urls` runs
    /// over fresh output it cannot survive a translation, so failing the
    /// build on it can't deadlock. A count mismatch is a different animal:
    /// the translator dropped or invented a whole link, which is a content
    /// defect worth re-translating for but not a broken URL, and there is no
    /// deterministic repair for it - gating on it would wedge the build
    /// behind an LLM that may never produce the right count.
    pub fn has_altered_target(&self) -> bool {
        self.expected.len() == self.actual.len()
    }

    pub fn describe(&self) -> String {
        if self.expected.len() != self.actual.len() {
            return format!(
                "link count differs: source has {}, translation has {}",
                self.expected.len(),
                self.actual.len()
            );
        }
        let changed: Vec<String> = self
            .expected
            .iter()
            .zip(&self.actual)
            .filter(|(e, a)| e != a)
            .map(|(e, a)| format!("{a:?} should be {e:?}"))
            .collect();
        format!("link target(s) altered: {}", changed.join("; "))
    }
}

/// `None` when the translation links exactly where the source links.
pub fn link_diff(source: &str, translated: &str) -> Option<LinkDiff> {
    let expected = extract_link_urls(source);
    let actual = extract_link_urls(translated);
    if expected == actual {
        return None;
    }
    Some(LinkDiff { expected, actual })
}

/// Restore the source's link targets into `translated`, positionally,
/// keeping the translated link text.
///
/// A count mismatch means the translation dropped or invented a link, and
/// there is no safe pairing, so leave it alone and let the gate report it -
/// same contract as [`crate::images::merge_image_srcs`]. Idempotent.
pub fn merge_link_urls(source: &str, translated: &str) -> String {
    let source_urls = extract_link_urls(source);
    if source_urls.is_empty() || source_urls.len() != extract_link_urls(translated).len() {
        return translated.to_string();
    }
    let masked = masked_ranges(translated);

    // Collect every replacement site first, then apply back-to-front so
    // earlier byte offsets stay valid.
    let mut sites: Vec<(usize, usize, String)> = Vec::new();
    for caps in MD_LINK_RE.captures_iter(translated) {
        let url = caps.get(3).expect("url group");
        if !is_masked(&masked, url.start()) {
            sites.push((url.start(), url.end(), String::new()));
        }
    }
    for caps in HREF_RE.captures_iter(translated) {
        let value = caps.get(3).or_else(|| caps.get(4)).expect("href value");
        if !is_masked(&masked, value.start()) {
            sites.push((value.start(), value.end(), String::new()));
        }
    }
    sites.sort_by_key(|(s, _, _)| *s);
    for (i, site) in sites.iter_mut().enumerate() {
        site.2 = source_urls[i].clone();
    }

    let mut out = translated.to_string();
    for (start, end, want) in sites.into_iter().rev() {
        if out[start..end] != want {
            out.replace_range(start..end, &want);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_markdown_links_and_hrefs_in_order_ignoring_images() {
        let doc = r#"See [a](https://one) and <a href="https://two">b</a> ![shot](https://img)"#;
        assert_eq!(extract_link_urls(doc), vec!["https://one", "https://two"]);
    }

    #[test]
    fn ignores_links_inside_code_and_screenshot_blocks() {
        let doc = "```md\n[x](https://fenced)\n```\n\n[inline-code-start]\n[y](https://inline)\n[inline-code-end]\n\n[app-screenshot-start url='https://shot'; title='T' app-screenshot-end]\n\n[real](https://kept)\n";
        assert_eq!(extract_link_urls(doc), vec!["https://kept"]);
    }

    #[test]
    fn identical_links_are_not_a_diff() {
        let src = "[Docs](https://a) and <a href='https://b'>x</a>";
        let tr = "[Dokumentation](https://a) and <a href='https://b'>y</a>";
        assert!(link_diff(src, tr).is_none());
    }

    #[test]
    fn an_altered_target_is_reported_and_merged_back() {
        // sdk-nim/ko_kr dropped `_status_` from the middle of the path.
        let src = "Returns: [`T`](https://github.com/o/r/blob/master/model_page_subscription_status_response.nim)";
        let tr = "반환: [`T`](https://github.com/o/r/blob/master/model_page_subscription_response.nim)";
        assert!(link_diff(src, tr).is_some());
        let merged = merge_link_urls(src, tr);
        assert!(merged.starts_with("반환: [`T`]("));
        assert!(merged.contains("model_page_subscription_status_response.nim"));
        assert!(link_diff(src, &merged).is_none());
        // Idempotent.
        assert_eq!(merge_link_urls(src, &merged), merged);
    }

    #[test]
    fn a_zero_width_space_in_a_host_is_repaired() {
        // sdk-nim/he: `fastcomment\u{200b}s-nim`.
        let src = "[`T`](https://github.com/FastComments/fastcomments-nim/blob/master/a.nim)";
        let tr = "[`T`](https://github.com/FastComments/fastcomment\u{200b}s-nim/blob/master/a.nim)";
        assert!(link_diff(src, tr).is_some());
        assert_eq!(merge_link_urls(src, tr), src);
    }

    #[test]
    fn href_quote_style_is_preserved() {
        let src = r#"<a href="https://new" target="_blank">x</a>"#;
        let tr = r#"<a href='https://old' target="_blank">y</a>"#;
        assert_eq!(
            merge_link_urls(src, tr),
            r#"<a href='https://new' target="_blank">y</a>"#
        );
    }

    #[test]
    fn a_dropped_link_is_reported_but_not_merged() {
        let src = "[a](https://one) [b](https://two)";
        let tr = "[a](https://one) b";
        let diff = link_diff(src, tr).expect("diff");
        assert!(diff.describe().contains("count differs"));
        // No safe pairing, so the translation is left for re-translation,
        // and the gate must not fail the build on it.
        assert!(!diff.has_altered_target());
        assert_eq!(merge_link_urls(src, tr), tr);
    }

    #[test]
    fn a_bracketed_label_still_matches() {
        // ``[`Option[T]`](url)`` is the shape of every generated SDK
        // `Returns:` line; a label pattern stopping at the first `]` sees none
        // of them and reports a clean file.
        let doc = "Returns: [`Option[GetV1PageLikes]`](https://one)";
        assert_eq!(extract_link_urls(doc), vec!["https://one"]);
        let tr = "מחזיר: [`Option[GetV1PageLikes]`](https://two)";
        assert!(link_diff(doc, tr).expect("diff").has_altered_target());
        assert_eq!(merge_link_urls(doc, tr), "מחזיר: [`Option[GetV1PageLikes]`](https://one)");
    }

    #[test]
    fn multiple_targets_are_restored_positionally() {
        let src = "[a](https://one) [b](https://two)";
        let tr = "[ä](https://uno) [b](https://dos)";
        assert_eq!(merge_link_urls(src, tr), "[ä](https://one) [b](https://two)");
    }
}


//! Port of `src/link-validator.js`. Pure-logic validation of inline
//! markdown links: anchor fragments against the registered guide-item ids,
//! relative `./...md` paths, and absolute `/guide-...html` links.

use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;

static MD_LINK: Lazy<Regex> = Lazy::new(|| {
    // Matches `[label](href)`. Mirrors MARKDOWN_LINK_REGEX in
    // src/link-validator.js:1.
    Regex::new(r"\[([^\]]*)\]\(([^)]+)\)").expect("regex")
});
static CODE_BLOCK: Lazy<Regex> = Lazy::new(|| {
    // Strip fenced and inline code blocks before scanning, mirroring
    // CODE_BLOCK_REGEX at line 2 of the JS.
    Regex::new(r"(?s)```.*?```|`[^`]+`").expect("regex")
});

const IMAGE_EXTENSIONS: &[&str] = &[
    ".png", ".jpg", ".jpeg", ".gif", ".svg", ".webp", ".ico",
];
const EXTERNAL_PREFIXES: &[&str] = &["http://", "https://", "mailto:", "tel:"];

#[derive(Debug, Error, Clone)]
#[error("invalid link at {file_path}:{line}\n  Link: [{text}]({href})\n  {issue}\n  Available: {available}")]
pub struct LinkError {
    pub file_path: String,
    pub line: usize,
    pub text: String,
    pub href: String,
    pub issue: String,
    pub available: String,
}

#[derive(Debug, Default, Clone)]
pub struct LinkValidator {
    /// For each guide id, the set of item ids known to exist on its page
    /// (meta.itemsOrdered file basenames without `.md`).
    guide_items: HashMap<String, HashSet<String>>,
}

impl LinkValidator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_guide_items<I, S>(&mut self, guide_id: &str, item_files: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let set: HashSet<String> = item_files
            .into_iter()
            .map(|f| f.as_ref().trim_end_matches(".md").to_string())
            .collect();
        self.guide_items.insert(guide_id.to_string(), set);
    }

    /// Validate every markdown link in `content`. Returns the list of
    /// problems found (empty == OK). Mirrors `validateContent` in
    /// src/link-validator.js:19-41.
    ///
    /// Performance: walks the original `content` directly (no
    /// allocated copy unless a code block is actually present) and
    /// uses a precomputed line index for O(log lines) line-number
    /// lookups per match — old impl was O(links × lines × line_len).
    pub fn validate(&self, content: &str, file_path: &str, guide_id: &str) -> Vec<LinkError> {
        let mut errors = Vec::new();
        // Precompute code-block byte ranges so we can skip matches that
        // sit inside them without paying for an owned String of the
        // code-stripped content.
        let code_ranges: Vec<std::ops::Range<usize>> = CODE_BLOCK
            .find_iter(content)
            .map(|m| m.start()..m.end())
            .collect();
        let line_starts = compute_line_starts(content);

        for cap in MD_LINK.captures_iter(content) {
            let full = cap.get(0).unwrap();
            if in_any_range(full.start(), &code_ranges) {
                continue;
            }
            let text = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let href = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            if should_skip(href) {
                continue;
            }
            let line = line_of(full.start(), &line_starts);
            let (path, anchor) = parse_link(href);

            if path.is_none() && anchor.is_some() {
                self.check_anchor(file_path, guide_id, anchor.as_deref().unwrap(), text, href, line, &mut errors);
            } else if let Some(p) = path.as_deref() {
                if p.starts_with("./") || p.starts_with("../") {
                    self.check_relative(file_path, guide_id, p, anchor.as_deref(), text, href, line, &mut errors);
                } else if p.starts_with("/guide-") {
                    self.check_absolute(file_path, p, anchor.as_deref(), text, href, line, &mut errors);
                }
            }
        }

        errors
    }

    fn check_anchor(
        &self,
        file_path: &str,
        guide_id: &str,
        anchor: &str,
        text: &str,
        href: &str,
        line: usize,
        out: &mut Vec<LinkError>,
    ) {
        let items = self.guide_items.get(guide_id);
        if !items.map_or(false, |s| s.contains(anchor)) {
            out.push(LinkError {
                file_path: file_path.to_string(),
                line,
                text: text.to_string(),
                href: href.to_string(),
                issue: format!("Anchor '{anchor}' not found in guide '{guide_id}'"),
                available: format_available(items),
            });
        }
    }

    fn check_relative(
        &self,
        file_path: &str,
        guide_id: &str,
        link_path: &str,
        anchor: Option<&str>,
        text: &str,
        href: &str,
        line: usize,
        out: &mut Vec<LinkError>,
    ) {
        // ./foo.md or ../foo.md -> foo
        let target = link_path
            .trim_start_matches("./")
            .trim_start_matches("../")
            .trim_end_matches(".md");
        let items = self.guide_items.get(guide_id);
        if !items.map_or(false, |s| s.contains(target)) {
            out.push(LinkError {
                file_path: file_path.to_string(),
                line,
                text: text.to_string(),
                href: href.to_string(),
                issue: format!("Item '{target}' not found in guide '{guide_id}'"),
                available: format_available(items),
            });
        } else {
            push_missing_anchor(items, anchor, file_path, text, href, line, guide_id, out);
        }
    }

    fn check_absolute(
        &self,
        file_path: &str,
        link_path: &str,
        anchor: Option<&str>,
        text: &str,
        href: &str,
        line: usize,
        out: &mut Vec<LinkError>,
    ) {
        // /guide-installation.html -> installation
        // /guide-installation-fr.html -> installation (strip 2-letter locale suffix)
        let mut guide_id = link_path
            .trim_start_matches("/guide-")
            .trim_end_matches(".html")
            .to_string();
        if let Some(stripped) = strip_locale_suffix(&guide_id) {
            guide_id = stripped;
        }

        let items = self.guide_items.get(&guide_id);
        if items.is_none() {
            let available: Vec<&String> = self.guide_items.keys().take(5).collect();
            let avail_str = available
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            let extra = if self.guide_items.len() > 5 { ", ..." } else { "" };
            out.push(LinkError {
                file_path: file_path.to_string(),
                line,
                text: text.to_string(),
                href: href.to_string(),
                issue: format!("Guide '{guide_id}' not found"),
                available: format!("{avail_str}{extra}"),
            });
            return;
        }
        push_missing_anchor(items, anchor, file_path, text, href, line, &guide_id, out);
    }
}

/// Push a "Anchor 'X' not found in guide 'Y'" error iff the link has
/// an anchor and the guide's item-set doesn't contain it. The
/// guide-existence + item-existence check is the caller's job; this
/// only handles the trailing anchor-membership check that both
/// `check_relative` and `check_absolute` carried inline.
#[allow(clippy::too_many_arguments)]
fn push_missing_anchor(
    items: Option<&HashSet<String>>,
    anchor: Option<&str>,
    file_path: &str,
    text: &str,
    href: &str,
    line: usize,
    guide_id: &str,
    out: &mut Vec<LinkError>,
) {
    let (Some(a), Some(set)) = (anchor, items) else {
        return;
    };
    if !set.contains(a) {
        out.push(LinkError {
            file_path: file_path.to_string(),
            line,
            text: text.to_string(),
            href: href.to_string(),
            issue: format!("Anchor '{a}' not found in guide '{guide_id}'"),
            available: format_available(items),
        });
    }
}

fn should_skip(href: &str) -> bool {
    let lower = href.to_ascii_lowercase();
    for prefix in EXTERNAL_PREFIXES {
        if lower.starts_with(prefix) {
            return true;
        }
    }
    for ext in IMAGE_EXTENSIONS {
        if lower.ends_with(ext) {
            return true;
        }
    }
    false
}

fn parse_link(href: &str) -> (Option<String>, Option<String>) {
    if let Some(idx) = href.find('#') {
        if idx == 0 {
            return (None, Some(href[1..].to_string()));
        }
        return (Some(href[..idx].to_string()), Some(href[idx + 1..].to_string()));
    }
    (Some(href.to_string()), None)
}

/// Byte offset of the first byte of each line. `line_starts[0] = 0`;
/// `line_starts[i]` is the byte right after the `\n` that ends line i-1.
fn compute_line_starts(content: &str) -> Vec<usize> {
    let mut out = Vec::with_capacity(content.len() / 32 + 1);
    out.push(0);
    for (i, b) in content.bytes().enumerate() {
        if b == b'\n' {
            out.push(i + 1);
        }
    }
    out
}

/// 1-based line number for a byte offset, via binary search of the
/// precomputed line index.
fn line_of(offset: usize, line_starts: &[usize]) -> usize {
    match line_starts.binary_search(&offset) {
        Ok(idx) => idx + 1,
        Err(idx) => idx, // insertion point: lines are 1-based
    }
}

fn in_any_range(offset: usize, ranges: &[std::ops::Range<usize>]) -> bool {
    // ranges from `find_iter` are non-overlapping + sorted by start.
    let idx = match ranges.binary_search_by_key(&offset, |r| r.start) {
        Ok(i) => i,
        Err(0) => return false,
        Err(i) => i - 1,
    };
    ranges
        .get(idx)
        .map(|r| offset < r.end)
        .unwrap_or(false)
}

fn format_available(items: Option<&HashSet<String>>) -> String {
    match items {
        None => "none".to_string(),
        Some(s) => {
            let mut v: Vec<&String> = s.iter().collect();
            v.sort();
            let head = v
                .iter()
                .take(5)
                .map(|x| x.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            if v.len() > 5 {
                format!("{head}, ...")
            } else {
                head
            }
        }
    }
}

/// `installation-fr` -> `installation` (strip trailing `-XX` where XX is
/// two ASCII lowercase letters). Mirrors the regex `^(.+)-([a-z]{2})$` at
/// `src/link-validator.js:99`.
fn strip_locale_suffix(guide_id: &str) -> Option<String> {
    let bytes = guide_id.as_bytes();
    if bytes.len() < 4 {
        return None;
    }
    let n = bytes.len();
    if bytes[n - 3] == b'-'
        && bytes[n - 2].is_ascii_lowercase()
        && bytes[n - 1].is_ascii_lowercase()
    {
        Some(guide_id[..n - 3].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vt(content: &str) -> Vec<LinkError> {
        let mut v = LinkValidator::new();
        v.register_guide_items("installation", ["wordpress.md", "vanillajs.md"]);
        v.register_guide_items("api", ["comments-get.md"]);
        v.validate(content, "test.md", "installation")
    }

    #[test]
    fn external_links_pass() {
        let errs = vt("see [docs](https://example.com)");
        assert!(errs.is_empty());
    }

    #[test]
    fn anchor_to_existing_item_passes() {
        let errs = vt("see [it](#wordpress)");
        assert!(errs.is_empty());
    }

    #[test]
    fn anchor_to_missing_item_fails() {
        let errs = vt("see [it](#nope)");
        assert_eq!(errs.len(), 1);
        assert!(errs[0].issue.contains("Anchor 'nope'"));
    }

    #[test]
    fn relative_link_to_existing_item_passes() {
        let errs = vt("see [w](./wordpress.md)");
        assert!(errs.is_empty());
    }

    #[test]
    fn relative_link_to_missing_item_fails() {
        let errs = vt("see [x](./nope.md)");
        assert_eq!(errs.len(), 1);
        assert!(errs[0].issue.contains("Item 'nope'"));
    }

    #[test]
    fn absolute_guide_link_passes() {
        let errs = vt("see [it](/guide-api.html#comments-get)");
        assert!(errs.is_empty());
    }

    #[test]
    fn absolute_guide_link_with_locale_strips() {
        let errs = vt("see [it](/guide-api-fr.html#comments-get)");
        assert!(errs.is_empty());
    }

    #[test]
    fn absolute_guide_link_to_missing_guide_fails() {
        let errs = vt("see [it](/guide-nonexistent.html)");
        assert_eq!(errs.len(), 1);
        assert!(errs[0].issue.contains("Guide 'nonexistent'"));
    }

    #[test]
    fn image_links_skipped() {
        let errs = vt("see [pic](./somewhere/image.png)");
        assert!(errs.is_empty());
    }

    #[test]
    fn code_blocks_not_scanned() {
        let errs = vt("```\n[bad](./nope.md)\n```\n");
        assert!(errs.is_empty(), "got: {errs:?}");
    }

    #[test]
    fn strip_locale_suffix_works() {
        assert_eq!(strip_locale_suffix("installation"), None);
        assert_eq!(strip_locale_suffix("installation-fr"), Some("installation".to_string()));
        assert_eq!(strip_locale_suffix("api-en"), Some("api".to_string()));
        assert_eq!(strip_locale_suffix("api-FR"), None); // uppercase, not stripped
        assert_eq!(strip_locale_suffix("api-fra"), None); // 3 letters, not stripped
    }
}

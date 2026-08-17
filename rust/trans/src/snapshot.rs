//! Content-hashing helper for the translation-cache + a tiny
//! "is this file even worth translating?" predicate that BOTH
//! `check.rs` (gap detection) and `run.rs` (skip-at-runtime) share.
//!
//! Originally also held a `Snapshot` type that mirrored Node's
//! `translation-snapshot.json` shape, but the Rust paths in
//! check.rs / run.rs / ui.rs / meta_json.rs ended up using flat
//! `BTreeMap<String, String>` caches instead, so the Snapshot
//! type was dead-code-warned and got removed. Only the MD5 helper
//! survived because every cache compares against it.

/// Minimum source-file size (after `.trim()`) below which translation
/// is a no-op. Mirrors the historic Node skip at
/// `the legacy Node translator` for tiny `intro.md` / `conclusion.md` files
/// that don't carry meaningful text.
///
/// Used in TWO places that MUST agree:
///   - `check.rs` — when gauging "is this file missing?", skip files
///     whose source is too small. Otherwise we enqueue them on every
///     run only for `run.rs` to skip them with no work done; the
///     target file never gets created and `check` flags them as
///     missing AGAIN next time. Infinite re-translate loop.
///   - `run.rs` — same predicate, called per-task to short-circuit
///     the LLM round-trip.
pub const MIN_SOURCE_LEN_FOR_TRANSLATION: usize = 10;

/// Should this source file be skipped because it's too small to bother
/// translating? Trim before measuring (matches Node behavior).
pub fn source_is_too_small_to_translate(source: &str) -> bool {
    source.trim().len() < MIN_SOURCE_LEN_FOR_TRANSLATION
}

/// A generated reference index - an API endpoint table or a model list -
/// carries no meaningful prose, and asking an LLM to reproduce hundreds of
/// link rows verbatim reliably fails.
///
/// `sdk-cpp/documentation-for-fastcomments-readme-generated.md` is 592 lines:
/// 211 table rows, 360 `- [Type](url)` bullets, 8 headings, and **4 lines of
/// actual prose**. Translations of it came back with between 0 and 259 of the
/// 360 links. That is unfixable by any merge (there is nothing to pair up),
/// so the file fails the link-parity check forever, which makes `run`
/// re-translate all 66KB of it, for every locale, on every build, forever.
///
/// So don't translate them. Copy the source verbatim, the way `en_us` is
/// handled. Shipping an English model index beats shipping a truncated one.
///
/// The threshold is deliberately far from any real page: it needs 50+
/// non-blank lines AND under 5% of them to be prose. Across the whole content
/// tree that selects exactly five files (the cpp and nim endpoint indexes and
/// three sdk-php ones) with no near misses - the next-closest page is above
/// 12%.
pub fn source_is_reference_index(source: &str) -> bool {
    const MIN_LINES: usize = 50;
    const MAX_PROSE_RATIO_DENOM: usize = 20; // prose * 20 < non_blank  ==>  <5%

    let mut non_blank = 0usize;
    let mut prose = 0usize;
    for line in source.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        non_blank += 1;
        if t.starts_with('#') || t.starts_with("<a name") {
            continue; // heading / anchor
        }
        if t.starts_with('|') || is_api_table_row(t) {
            continue; // table row
        }
        if is_bullet_link_only(t) {
            continue; // `- [Type](url)` index entry
        }
        prose += 1;
    }
    non_blank >= MIN_LINES && prose * MAX_PROSE_RATIO_DENOM < non_blank
}

/// `*DefaultApi* | **addPage** | **POST** /api/v1/pages | |` - openapi-generator
/// emits the first column unpiped, so it isn't caught by a leading `|`.
fn is_api_table_row(t: &str) -> bool {
    let Some(rest) = t.strip_prefix('*') else {
        return false;
    };
    let Some((name, after)) = rest.split_once('*') else {
        return false;
    };
    !name.is_empty()
        && name.chars().all(|c| c.is_ascii_alphanumeric())
        && after.trim_start().starts_with('|')
}

/// A line that is nothing but one markdown link bullet.
fn is_bullet_link_only(t: &str) -> bool {
    let Some(rest) = t.strip_prefix(['-', '*']) else {
        return false;
    };
    let rest = rest.trim_start();
    if !rest.starts_with('[') {
        return false;
    }
    let Some(close) = rest.find("](") else {
        return false;
    };
    rest[close + 2..].find(')').is_some_and(|end| {
        rest[close + 2 + end + 1..].trim().is_empty()
    })
}

/// MD5 hex digest of `content`. Matches Node `crypto.createHash('md5').update(content).digest('hex')`
/// so existing `translation-cache.json` entries remain valid hits.
pub fn hash_content(content: &str) -> String {
    use md5::{Digest, Md5};
    let mut h = Md5::new();
    h.update(content.as_bytes());
    let d = h.finalize();
    let mut hex = String::with_capacity(32);
    for b in d {
        hex.push_str(&format!("{b:02x}"));
    }
    hex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_generated_endpoint_index_is_not_worth_translating() {
        let mut doc = String::from("## Documentation for API Endpoints\n\nAll URIs are relative to *https://fastcomments.com*\n\n| Class | Method |\n");
        for i in 0..60 {
            doc.push_str(&format!("*DefaultApi* | **method{i}** | **POST** /api/v1/x |  |\n"));
            doc.push_str(&format!("- [Model{i}](https://github.com/o/r/blob/main/Model{i}.md)\n"));
        }
        assert!(source_is_reference_index(&doc));
    }

    #[test]
    fn a_real_guide_page_is_still_translated() {
        // Prose with a normal sprinkling of links must never be exempted.
        let mut doc = String::new();
        for i in 0..60 {
            doc.push_str(&format!(
                "Paragraph {i} explaining how the widget behaves, with [a link](https://example.com/{i}) in it.\n"
            ));
        }
        assert!(!source_is_reference_index(&doc));
    }

    #[test]
    fn a_short_link_list_is_still_translated() {
        // Below the line threshold, so the ratio never gets consulted.
        let doc = "- [A](https://a)\n- [B](https://b)\n";
        assert!(!source_is_reference_index(doc));
    }

    #[test]
    fn md5_matches_node() {
        // Cross-checked via:
        //   node -e "console.log(require('crypto').createHash('md5').update('hello').digest('hex'))"
        assert_eq!(hash_content("hello"), "5d41402abc4b2a76b9719d911017c592");
        assert_eq!(hash_content(""), "d41d8cd98f00b204e9800998ecf8427e");
    }
}

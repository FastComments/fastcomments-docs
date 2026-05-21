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
/// `translate-with-gpt.js` for tiny `intro.md` / `conclusion.md` files
/// that don't carry meaningful text.
///
/// Used in TWO places that MUST agree:
///   - `check.rs` — when gauging "is this file missing?", skip files
///     whose source is too small. Otherwise we enqueue them on every
///     run only for `run.rs` to skip them with no work done; the
///     target file never gets created and `check` flags them as
///     missing AGAIN next time. Infinite re-translate loop.
///   - `run.rs` — same predicate, called per-task to short-circuit
///     the OpenAI round-trip.
pub const MIN_SOURCE_LEN_FOR_TRANSLATION: usize = 10;

/// Should this source file be skipped because it's too small to bother
/// translating? Trim before measuring (matches Node behavior).
pub fn source_is_too_small_to_translate(source: &str) -> bool {
    source.trim().len() < MIN_SOURCE_LEN_FOR_TRANSLATION
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
    fn md5_matches_node() {
        // Cross-checked via:
        //   node -e "console.log(require('crypto').createHash('md5').update('hello').digest('hex'))"
        assert_eq!(hash_content("hello"), "5d41402abc4b2a76b9719d911017c592");
        assert_eq!(hash_content(""), "d41d8cd98f00b204e9800998ecf8427e");
    }
}

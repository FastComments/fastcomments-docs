//! Content-hashing helper for the translation-cache.
//!
//! Originally also held a `Snapshot` type that mirrored Node's
//! `translation-snapshot.json` shape, but the Rust paths in
//! check.rs / run.rs / ui.rs / meta_json.rs ended up using flat
//! `BTreeMap<String, String>` caches instead, so the Snapshot
//! type was dead-code-warned and got removed. Only the MD5 helper
//! survived because every cache compares against it.

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

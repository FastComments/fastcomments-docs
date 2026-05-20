//! Port of `src/translation-snapshot.js`. MD5-based source-change
//! tracking so we can detect when a previously-translated file needs to
//! be re-translated. The on-disk JSON shape must match Node's so existing
//! `src/translation-snapshot.json` keeps working.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub hash: String,
    #[serde(default, rename = "translatedLocales")]
    pub translated_locales: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Snapshot(pub BTreeMap<String, Entry>);

impl Snapshot {
    pub fn load_or_default(path: &Path) -> Self {
        match std::fs::read(path) {
            Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
            Err(_) => Snapshot::default(),
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let bytes = serde_json::to_vec_pretty(&self.0)?;
        std::fs::write(path, bytes).with_context(|| format!("write {path:?}"))?;
        Ok(())
    }

    pub fn needs_translation(
        &self,
        guide_id: &str,
        filename: &str,
        locale: &str,
        current_hash: &str,
    ) -> bool {
        let key = source_key(guide_id, filename);
        match self.0.get(&key) {
            None => true,
            Some(entry) => {
                entry.hash != current_hash
                    || !entry.translated_locales.iter().any(|l| l == locale)
            }
        }
    }

    pub fn mark_translated(
        &mut self,
        guide_id: &str,
        filename: &str,
        locale: &str,
        hash: &str,
    ) {
        let key = source_key(guide_id, filename);
        let e = self.0.entry(key).or_default();
        e.hash = hash.to_string();
        if !e.translated_locales.iter().any(|l| l == locale) {
            e.translated_locales.push(locale.to_string());
            e.translated_locales.sort();
        }
    }
}

pub fn source_key(guide_id: &str, filename: &str) -> String {
    format!("{guide_id}/{filename}")
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

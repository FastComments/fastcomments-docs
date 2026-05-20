//! Image cache parity with the Node version at
//! `src/app-screenshot-generator.js:27-68`. Cache file format v2:
//!
//! ```json
//! { "version": 2, "timestamp": <ms>, "contentJSON": "<JSON.stringify(args)>" }
//! ```

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

const CACHE_FORMAT_VERSION: u32 = 2;
const ONE_WEEK_MS: u128 = 6.048e+8 as u128;

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheRecord {
    pub version: u32,
    pub timestamp: i64,
    #[serde(rename = "contentJSON")]
    pub content_json: String,
}

pub struct ImageCache {
    pub folder: PathBuf,
}

impl ImageCache {
    pub fn new(folder: impl Into<PathBuf>) -> Self {
        Self {
            folder: folder.into(),
        }
    }

    /// Mirrors `isImageCacheStale` in src/app-screenshot-generator.js:27-56.
    pub fn is_stale(&self, args_json: &str, full_path: &Path, file_name: &str) -> bool {
        if !full_path.exists() {
            return true;
        }
        if !self.folder.exists() {
            let _ = std::fs::create_dir_all(&self.folder);
            return true;
        }
        let meta_path = self.folder.join(format!("{file_name}.json"));
        if !meta_path.exists() {
            return true;
        }
        let Ok(bytes) = std::fs::read(&meta_path) else {
            return true;
        };
        let Ok(rec): Result<CacheRecord, _> = serde_json::from_slice(&bytes) else {
            return true;
        };
        if rec.version != CACHE_FORMAT_VERSION {
            return true;
        }
        let now = now_ms();
        if (now as u128).saturating_sub(rec.timestamp as u128) > ONE_WEEK_MS {
            return true;
        }
        rec.content_json != args_json
    }

    /// Mirrors `updateImageCache` in src/app-screenshot-generator.js:58-68.
    pub fn update(&self, args_json: &str, file_name: &str) -> std::io::Result<()> {
        if !self.folder.exists() {
            std::fs::create_dir_all(&self.folder)?;
        }
        let meta_path = self.folder.join(format!("{file_name}.json"));
        let rec = CacheRecord {
            version: CACHE_FORMAT_VERSION,
            timestamp: now_ms(),
            content_json: args_json.to_string(),
        };
        let bytes = serde_json::to_vec(&rec).expect("serialize cache record");
        std::fs::write(meta_path, bytes)
    }
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

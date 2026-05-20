//! chromiumoxide wrapper used by `fcdocs-sitegen` for `[app-screenshot-*]`
//! and `[flow-diagram-*]` markers. Replaces puppeteer.
//!
//! Mirrors `src/app-screenshot-generator.js` behavior:
//! - Single browser per call (pooling intentionally disabled, same as
//!   the Node version at lines 121-156).
//! - Login-as-demo flow before each screenshot.
//! - JSON sidecar cache (`<file>.json`) with version 2, 1-week TTL, args hash.
//!
//! Stub: real implementation lands as part of task #15.

pub mod cache;

use std::path::PathBuf;

/// Locate the Chromium binary. Honors `CHROME_BIN` first, then falls back
/// to the puppeteer-downloaded one already on the build server.
pub fn chrome_binary() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("CHROME_BIN") {
        let pb = PathBuf::from(p);
        if pb.exists() {
            return Some(pb);
        }
    }
    let pup = PathBuf::from(
        "node_modules/puppeteer/.local-chromium/linux-848005/chrome-linux/chrome",
    );
    if pup.exists() {
        return Some(pup);
    }
    None
}

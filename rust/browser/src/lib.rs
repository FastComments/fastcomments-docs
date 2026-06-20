//! chromiumoxide wrapper used by `fcdocs-sitegen` for `[app-screenshot-*]`
//! and `[flow-diagram-*]` markers. Replaces puppeteer.
//!
//! Behavior parity reference: `src/app-screenshot-generator.js`.

pub mod cache;
pub mod pool;
pub mod screenshot;

pub use pool::BrowserPool;

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
    // puppeteer >= v19 downloads Chrome to
    // $HOME/.cache/puppeteer/chrome/linux-<version>/chrome-linux64/chrome.
    // Pick the newest version directory present.
    if let Some(home) = std::env::var_os("HOME") {
        let base = PathBuf::from(home).join(".cache/puppeteer/chrome");
        if let Ok(entries) = std::fs::read_dir(&base) {
            let mut dirs: Vec<PathBuf> =
                entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
            dirs.sort();
            for d in dirs.into_iter().rev() {
                let chrome = d.join("chrome-linux64").join("chrome");
                if chrome.exists() {
                    return Some(chrome);
                }
            }
        }
    }
    // Older puppeteer layouts + system installs.
    let candidates = [
        "node_modules/puppeteer/.local-chromium/linux-848005/chrome-linux/chrome",
        "node_modules/puppeteer/.cache/chrome/linux-stable/chrome",
        "/usr/bin/chromium-browser",
        "/usr/bin/chromium",
        "/usr/bin/google-chrome",
    ];
    for c in candidates {
        let pb = PathBuf::from(c);
        if pb.exists() {
            return Some(pb);
        }
    }
    None
}

pub use cache::{CacheRecord, ImageCache};
pub use screenshot::{Action, ScreenshotArgs, ScreenshotHost};

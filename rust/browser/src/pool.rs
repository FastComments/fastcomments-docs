//! Reusable browser session pool.
//!
//! The old sitegen flow launched chromium *per screenshot*: ~500ms of
//! browser startup per call, repeated across every cache-stale image.
//! This pool holds one logged-in browser per-(width) and lazily restarts
//! it if a navigation kills the underlying process.
//!
//! `MAX_BROWSERS` env var caps concurrency (default 1, matching Node's
//! actual single-browser behavior at app-screenshot-generator.js:121-156).
//! Workers acquire a permit, then hold the per-width mutex while they
//! capture.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use anyhow::{Context, Result};
use chromiumoxide::{Browser, Page};
use tokio::sync::{Mutex, Semaphore};

use crate::screenshot::{self, ScreenshotHost};

/// Boxed future returned by user callbacks. Using a boxed future avoids
/// higher-ranked-lifetime trouble that you hit with `FnOnce(&Page, &Host)
/// -> Fut`, because `Fut` would need a lifetime parameter tied to the
/// borrows.
pub type PagedFuture<'a, R> = Pin<Box<dyn Future<Output = Result<R>> + Send + 'a>>;

pub struct BrowserPool {
    host: ScreenshotHost,
    by_width: Mutex<HashMap<u32, Arc<Mutex<Option<Session>>>>>,
    permits: Arc<Semaphore>,
}

struct Session {
    browser: Browser,
    page: Page,
    handler_task: tokio::task::JoinHandle<()>,
}

impl BrowserPool {
    pub fn new(host: ScreenshotHost) -> Self {
        let max_browsers: usize = std::env::var("MAX_BROWSERS")
            .ok()
            .and_then(|s| s.parse().ok())
            .filter(|n: &usize| *n > 0)
            .unwrap_or(1);
        Self {
            host,
            by_width: Mutex::new(HashMap::new()),
            permits: Arc::new(Semaphore::new(max_browsers)),
        }
    }

    /// Run `f` against a logged-in page. The session is lazily created
    /// on first use and reused across calls. If `f` fails or the
    /// underlying page errors, the session is dropped and the next call
    /// re-launches.
    pub async fn with_page<F, R>(&self, width: u32, f: F) -> Result<R>
    where
        F: for<'a> FnOnce(&'a Page, &'a ScreenshotHost) -> PagedFuture<'a, R>,
    {
        // Bound total concurrent sessions across the build.
        let _permit = self.permits.clone().acquire_owned().await.unwrap();

        let slot = {
            let mut by_width = self.by_width.lock().await;
            by_width
                .entry(width)
                .or_insert_with(|| Arc::new(Mutex::new(None)))
                .clone()
        };
        let mut guard = slot.lock().await;

        if guard.is_none() {
            let (browser, page, handler_task) = screenshot::launch_logged_in(
                width,
                screenshot::DEFAULT_HEIGHT,
                &self.host,
            )
            .await
            .context("launch logged-in browser")?;
            *guard = Some(Session {
                browser,
                page,
                handler_task,
            });
        }

        let session = guard.as_ref().expect("just initialized");
        let res = f(&session.page, &self.host).await;
        if res.is_err() {
            // Tear down on error — chromiumoxide can wedge a page into a
            // bad state, so safer to recycle than reuse.
            if let Some(mut s) = guard.take() {
                let _ = s.browser.close().await;
                s.handler_task.abort();
            }
        }
        res
    }

    /// Best-effort shutdown for any cached browsers (called on build
    /// teardown).
    pub async fn shutdown(&self) {
        let mut by_width = self.by_width.lock().await;
        for slot in by_width.drain().map(|(_, s)| s) {
            if let Ok(mut guard) = slot.try_lock() {
                if let Some(mut s) = guard.take() {
                    let _ = s.browser.close().await;
                    s.handler_task.abort();
                }
            }
        }
    }
}

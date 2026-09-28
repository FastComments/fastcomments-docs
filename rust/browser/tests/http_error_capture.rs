//! A screenshot target that answers with an HTTP error must fail right
//! away instead of waiting out the selector timeout.
//!
//! The FastComments test-e2e email route returned 500 for one docs marker
//! in every locale; each attempt sat on the 30s `.content` wait, three
//! attempts per locale, which stretched the docs build past half an hour.
//!
//! Requires a chromium binary; skipped when none is present.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::{Duration, Instant};

use fcdocs_browser::screenshot::{self, PageHttpError, ScreenshotArgs, ScreenshotHost};

/// Serve every request with `status` and a body that lacks the selector.
fn serve_status(status: u16) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
    let addr = listener.local_addr().expect("local addr");
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { continue };
            let mut buf = [0u8; 4096];
            let _ = stream.read(&mut buf);
            let body = "{\"status\":\"failed\"}";
            let _ = write!(
                stream,
                "HTTP/1.1 {status} Error\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
        }
    });
    format!("http://{addr}")
}

#[tokio::test]
async fn http_error_fails_fast_without_retry() {
    if fcdocs_browser::chrome_binary().is_none() {
        eprintln!("no chromium binary; skipping");
        return;
    }

    let host = ScreenshotHost {
        host: serve_status(500),
        ..ScreenshotHost::default()
    };
    let args = ScreenshotArgs {
        url: "/test-e2e/email/commenter-verify-post".to_string(),
        selector: ".content".to_string(),
        ..ScreenshotArgs::default()
    };
    let target = std::env::temp_dir().join(format!(
        "fcdocs-http-error-capture-{}.png",
        std::process::id()
    ));

    let (mut browser, handler) = screenshot::launch(1280, 720).await.expect("launch chromium");
    let page = browser.new_page("about:blank").await.expect("open page");

    let started = Instant::now();
    let res = screenshot::capture(&page, &args, &target, &host).await;
    let elapsed = started.elapsed();

    let _ = browser.close().await;
    handler.abort();

    let err = res.expect_err("a 500 page must not produce a screenshot");
    assert!(
        elapsed < Duration::from_secs(10),
        "capture took {elapsed:?}; it should fail on the status, not the selector timeout"
    );
    let http = err
        .downcast_ref::<PageHttpError>()
        .unwrap_or_else(|| panic!("expected a PageHttpError, got: {err:#}"));
    assert_eq!(http.status, 500);
    assert!(!http.is_transient(), "a 500 from the app is not worth retrying");
    assert!(!target.exists(), "no image should be written");
}

#[test]
fn gateway_errors_are_transient() {
    // A deploy restarting the app mid-build surfaces as a gateway error;
    // those still deserve the retry.
    for status in [502, 503, 504] {
        let e = PageHttpError { status, url: String::new() };
        assert!(e.is_transient(), "{status} should be transient");
    }
    for status in [400, 401, 404, 500] {
        let e = PageHttpError { status, url: String::new() };
        assert!(!e.is_transient(), "{status} should not be transient");
    }
}

//! Regression test for below-the-fold element screenshots.
//!
//! chromiumoxide 0.7's `Element::screenshot` double-applies the page
//! scroll offset, so any element the browser had to scroll to reach
//! captured as a blank rectangle. That silently emptied ~55 docs
//! screenshots (every marker targeting the lower half of
//! `/auth/my-account/customize-widget/new`).
//!
//! Requires a chromium binary; skipped when none is present.

use fcdocs_browser::screenshot;

/// A page where the target sits far below the fold, painted a colour
/// nothing else on the page uses.
const PAGE_HTML: &str = "<html><body style='margin:0;background:#ffffff'>\
<div style='height:4000px'></div>\
<div id='target' style='width:200px;height:60px;background:rgb(255,0,0)'></div>\
<div style='height:4000px'></div>\
</body></html>";

/// Distinct colours, capped so a photo doesn't build a huge set.
fn distinct_colors(pixels: &[[u8; 3]]) -> usize {
    let mut seen = std::collections::HashSet::new();
    for px in pixels {
        seen.insert(*px);
        if seen.len() > 64 {
            break;
        }
    }
    seen.len()
}

/// Decode to (width, height, RGB pixels). Chromium emits 8-bit RGB or
/// RGBA depending on whether the capture has transparency, so normalize
/// rather than assuming a channel count.
fn decode(png_bytes: &[u8]) -> (u32, u32, Vec<[u8; 3]>) {
    let mut decoder = png::Decoder::new(png_bytes);
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().expect("read png info");
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).expect("decode png frame");
    let channels = info.color_type.samples();
    assert!(
        channels >= 3,
        "expected an RGB(A) capture, got {:?}",
        info.color_type
    );
    let pixels = buf[..info.buffer_size()]
        .chunks_exact(channels)
        .map(|px| [px[0], px[1], px[2]])
        .collect();
    (info.width, info.height, pixels)
}

#[tokio::test]
async fn captures_element_below_the_fold() {
    if fcdocs_browser::chrome_binary().is_none() {
        eprintln!("no chromium binary; skipping");
        return;
    }

    let html_path = std::env::temp_dir().join(format!(
        "fcdocs-element-capture-{}.html",
        std::process::id()
    ));
    std::fs::write(&html_path, PAGE_HTML).expect("write fixture page");

    let (mut browser, handler) = screenshot::launch(1280, 720).await.expect("launch chromium");
    let page = browser
        .new_page(format!("file://{}", html_path.display()))
        .await
        .expect("open fixture page");

    let capture = screenshot::capture_element_png(&page, "#target").await;

    let _ = browser.close().await;
    handler.abort();
    let _ = std::fs::remove_file(&html_path);

    let png_bytes = capture.expect("capture #target");
    let (width, height, pixels) = decode(&png_bytes);
    assert_eq!((width, height), (200, 60), "clip should match the element box");

    // Every pixel of the target is pure red. A capture that misses the
    // element lands on empty page and comes back white.
    let red = pixels
        .iter()
        .filter(|px| px[0] > 200 && px[1] < 50 && px[2] < 50)
        .count();
    assert_eq!(
        red,
        pixels.len(),
        "expected the whole clip to be the red target, got {red} red pixels out of {} \
         ({} distinct colours - an all-white capture means the clip missed the element)",
        pixels.len(),
        distinct_colors(&pixels)
    );
}

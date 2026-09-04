//! Screenshot capture via chromiumoxide. Mirrors the behavior of
//! `src/app-screenshot-generator.js`:
//!
//! 1. Launch chromium with `--no-sandbox --disable-setuid-sandbox
//!    --enable-font-antialiasing`.
//! 2. Open a new page, set viewport, navigate to
//!    `<host>/auth/login?fromDocs=true`.
//! 3. Type `demo` into `input[name="username"]` and
//!    `demo@fastcomments.com` into `input[name="email"]`, submit.
//! 4. Wait for `body`.
//! 5. Navigate to the target URL, perform optional `actions`, optional
//!    `clickSelectors`, then wait for `selector`.
//! 6. Optional delay, then take an element screenshot of `selector` to
//!    the target PNG path.

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use chromiumoxide::browser::{Browser, BrowserConfigBuilder, HeadlessMode};
use chromiumoxide::cdp::browser_protocol::page::{
    CaptureScreenshotFormat, CaptureScreenshotParams, Viewport,
};
use chromiumoxide::Page;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

pub const HOST: &str = "https://fastcomments.com";
pub const DEFAULT_WIDTH: u32 = 1920;
pub const DEFAULT_HEIGHT: u32 = 1080;

/// Mirrors the marker config shape parsed from
/// `[app-screenshot-start ... app-screenshot-end]` blocks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScreenshotArgs {
    pub url: String,
    #[serde(default, rename = "linkUrl")]
    pub link_url: Option<serde_json::Value>,
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default, rename = "clickSelector")]
    pub click_selector: Option<String>,
    #[serde(default, rename = "clickSelectors")]
    pub click_selectors: Option<Vec<String>>,
    pub selector: String,
    #[serde(default)]
    pub title: String,
    /// Accessible description of the captured image. Falls back to
    /// `title` when unset. Deliberately absent from `target_file_name`
    /// and from the sitegen image-cache key so editing it never
    /// re-captures the screenshot.
    #[serde(default)]
    pub alt: String,
    #[serde(default, rename = "addProxySelect")]
    pub add_proxy_select: bool,
    #[serde(default)]
    pub delay: Option<u64>,
    #[serde(default, rename = "cacheBuster")]
    pub cache_buster: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Action {
    Wait { selector: String },
    Click { selector: String },
    #[serde(rename = "set-value")]
    SetValue { selector: String, value: String },
}

/// Host configuration for the demo-user login. Constructed once and
/// passed to `launch_logged_in`. Also carries the optional
/// `proxy-select` assets (script + style) so they can be injected on
/// pages where the marker config sets `addProxySelect=true` — mirrors
/// Node's `addProxySelectToPage` in `src/app-screenshot-generator.js:16-21`.
#[derive(Debug, Clone)]
pub struct ScreenshotHost {
    pub host: String,
    pub username: String,
    pub email: String,
    /// JavaScript loaded from `src/static/js/proxy-select.js`. Wrapped
    /// in `Arc` because every per-task capture clones the host config,
    /// and the script is ~11 KiB; sharing avoids per-screenshot copies.
    pub proxy_script: Option<Arc<String>>,
    /// CSS loaded from `src/static/css/proxy-select.css`.
    pub proxy_style: Option<Arc<String>>,
}

impl Default for ScreenshotHost {
    fn default() -> Self {
        Self {
            host: HOST.to_string(),
            username: "demo".to_string(),
            email: "demo@fastcomments.com".to_string(),
            proxy_script: None,
            proxy_style: None,
        }
    }
}

/// Launch chromium. Split out of `launch_logged_in` so tests (and any
/// caller that doesn't need the demo account) can open their own page
/// without the login navigation.
pub async fn launch(
    width: u32,
    height: u32,
) -> Result<(Browser, tokio::task::JoinHandle<()>)> {
    let chrome = crate::chrome_binary().context(
        "no chromium binary located. Set CHROME_BIN or install chromium-browser.",
    )?;

    let mut builder = BrowserConfigBuilder::default();
    builder = builder
        .chrome_executable(chrome)
        .arg("--no-sandbox")
        .arg("--disable-setuid-sandbox")
        .arg("--enable-font-antialiasing")
        .viewport(chromiumoxide::handler::viewport::Viewport {
            width,
            height,
            device_scale_factor: None,
            emulating_mobile: false,
            is_landscape: false,
            has_touch: false,
        })
        .window_size(width, height);
    builder = builder.headless_mode(HeadlessMode::True);
    // Unique profile dir per launch: chromiumoxide's default user-data-dir is a
    // shared fixed path, so concurrent Chrome instances (and stale locks from a
    // crashed prior run) collide on the ProcessSingleton lock and abort.
    use std::sync::atomic::{AtomicU64, Ordering};
    static LAUNCH_SEQ: AtomicU64 = AtomicU64::new(0);
    let seq = LAUNCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let profile_dir =
        std::env::temp_dir().join(format!("fcdocs-chrome-{}-{}", std::process::id(), seq));
    builder = builder.user_data_dir(profile_dir);
    let cfg = builder.build().map_err(|e| anyhow::anyhow!(e))?;

    let (browser, mut handler) = Browser::launch(cfg).await.context("Browser::launch")?;
    let handler_task = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if let Err(e) = h {
                tracing::debug!(error = %e, "chromiumoxide handler tick");
            }
        }
    });

    Ok((browser, handler_task))
}

/// Launch chromium with a fresh logged-in session and return the browser
/// along with the page handle. Caller is responsible for dropping the
/// browser when done.
pub async fn launch_logged_in(
    width: u32,
    height: u32,
    host_cfg: &ScreenshotHost,
) -> Result<(Browser, Page, tokio::task::JoinHandle<()>)> {
    let (browser, handler_task) = launch(width, height).await?;

    // fromDocs opts this headless login out of the captcha, which the server honours only for the
    // demo credentials below.
    let page = browser
        .new_page(format!("{}/auth/login?fromDocs=true", host_cfg.host))
        .await
        .context("open login page")?;

    // Wait for the login form to render. The Node version uses
    // page.waitForSelector('form').
    page.wait_for_navigation()
        .await
        .context("wait for login navigation")?;
    let _ = page.find_element("form").await; // ignore if not found, retry below
    // Type username + email.
    type_into(&page, "input[name=\"username\"]", &host_cfg.username).await?;
    type_into(&page, "input[name=\"email\"]", &host_cfg.email).await?;
    // Click submit and wait for the post-login body.
    let submit = page
        .find_element("button[type=\"submit\"]")
        .await
        .context("find submit button")?;
    submit.click().await.context("click submit")?;
    let _ = page.wait_for_navigation().await;
    let _ = page.find_element("body").await;

    // A rejected login (captcha, missing demo user) re-renders the login form. Fail here so a
    // broken login costs one clear error instead of every authenticated screenshot timing out on
    // selectors that only exist behind it.
    let landed = page.url().await.ok().flatten().unwrap_or_default();
    if landed.contains("/auth/login") {
        anyhow::bail!(
            "login did not succeed, still on {landed} - check the fromDocs captcha opt-out and demo credentials"
        );
    }

    Ok((browser, page, handler_task))
}

async fn type_into(page: &Page, selector: &str, value: &str) -> Result<()> {
    let el = page
        .find_element(selector)
        .await
        .with_context(|| format!("find {selector}"))?;
    el.click().await.with_context(|| format!("focus {selector}"))?;
    el.type_str(value)
        .await
        .with_context(|| format!("type into {selector}"))?;
    Ok(())
}

/// Capture an element screenshot per the marker's `args`. Mirrors the
/// branching in `app-screenshot-generator.js:199-304`.
pub async fn capture(
    page: &Page,
    args: &ScreenshotArgs,
    target_path: &Path,
    host_cfg: &ScreenshotHost,
) -> Result<()> {
    let url = ensure_host(&args.url, &host_cfg.host);
    goto_with_retry(page, &url).await?;

    if args.add_proxy_select {
        // Mirrors Node `addProxySelectToPage` (src/app-screenshot-generator.js:16-21):
        // inject the proxy-select script then the CSS. Without this,
        // native `<select>` dropdowns rendered by the OS are invisible
        // to Chromium's screenshot APIs, so a screenshot taken with an
        // OPEN dropdown captures only the closed select. The script
        // replaces every `<select>` with a DOM-rendered styled list.
        match (&host_cfg.proxy_script, &host_cfg.proxy_style) {
            (Some(script), Some(css)) => {
                let _ = page.evaluate(script.as_str()).await;
                let inject_css = format!(
                    "(()=>{{const s=document.createElement('style');s.textContent={};document.head.appendChild(s);}})()",
                    serde_json::to_string(css.as_str()).unwrap_or_else(|_| "''".to_string())
                );
                let _ = page.evaluate(inject_css).await;
            }
            _ => {
                tracing::warn!(
                    url = %url,
                    "marker requested addProxySelect=true but proxy-select assets not loaded in ScreenshotHost; screenshot will lack visible <select> dropdowns"
                );
            }
        }
    }

    for action in &args.actions {
        match action {
            Action::Wait { selector } => {
                let _ = wait_for_selector(page, selector, Duration::from_secs(15)).await;
            }
            Action::Click { selector } => {
                let _ = wait_for_selector(page, selector, Duration::from_secs(15)).await;
                if let Ok(el) = page.find_element(selector).await {
                    let _ = el.click().await;
                }
            }
            Action::SetValue { selector, value } => {
                let _ = wait_for_selector(page, selector, Duration::from_secs(15)).await;
                let js = format!(
                    "(()=>{{const el=document.querySelector({});if(!el)return;el.value={};el.dispatchEvent(new Event('input',{{bubbles:true}}));el.dispatchEvent(new Event('change',{{bubbles:true}}));}})()",
                    serde_json::to_string(selector).unwrap_or_else(|_| "''".to_string()),
                    serde_json::to_string(value).unwrap_or_else(|_| "''".to_string())
                );
                let _ = page.evaluate(js).await;
            }
        }
    }

    let click_selectors: Vec<&str> = if let Some(s) = &args.click_selector {
        vec![s.as_str()]
    } else if let Some(v) = &args.click_selectors {
        v.iter().map(|s| s.as_str()).collect()
    } else {
        Vec::new()
    };
    for sel in click_selectors {
        let _ = wait_for_selector(page, sel, Duration::from_secs(15)).await;
        if let Ok(el) = page.find_element(sel).await {
            let _ = el.click().await;
        }
    }

    wait_for_selector(page, &args.selector, Duration::from_secs(30))
        .await
        .with_context(|| format!("wait_for {} on {url}", args.selector))?;

    if let Some(ms) = args.delay {
        tokio::time::sleep(Duration::from_millis(ms)).await;
    }

    if let Some(parent) = target_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let png_bytes = capture_element_png(page, &args.selector)
        .await
        .with_context(|| format!("capture {} on {url}", args.selector))?;
    if let Some(density) = png_byte_density(&png_bytes) {
        // A flat, contentless capture compresses to almost nothing;
        // anything with text or borders in it is an order of magnitude
        // denser. 988 blank screenshots shipped before this warning
        // existed, so it is worth the two reads of the IHDR header.
        if density < FLAT_PNG_BYTES_PER_PIXEL {
            tracing::warn!(
                url = %url,
                selector = %args.selector,
                "captured image looks blank (no visible content); check the selector"
            );
        }
    }
    std::fs::write(target_path, png_bytes)
        .with_context(|| format!("write {target_path:?}"))?;

    Ok(())
}

/// Below this many PNG bytes per pixel an image is a single flat colour
/// in practice. Real captures of the docs UI land above 0.05.
const FLAT_PNG_BYTES_PER_PIXEL: f64 = 0.02;

/// Encoded bytes per pixel, read straight off the IHDR header. `None`
/// when the buffer isn't a PNG or has no area.
fn png_byte_density(png_bytes: &[u8]) -> Option<f64> {
    if png_bytes.len() < 24 || &png_bytes[..8] != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    let width = u32::from_be_bytes(png_bytes[16..20].try_into().ok()?) as f64;
    let height = u32::from_be_bytes(png_bytes[20..24].try_into().ok()?) as f64;
    let area = width * height;
    if area <= 0.0 {
        return None;
    }
    Some(png_bytes.len() as f64 / area)
}

/// How many times to attempt the initial navigation before giving up.
const GOTO_ATTEMPTS: u32 = 3;

/// Navigate, retrying transient aborts.
///
/// The pool reuses one page across every capture, and a marker's clicks
/// can leave a navigation in flight; the next `goto` to the same URL
/// then comes back `net::ERR_ABORTED`. sitegen treats a failed capture
/// as "leave the image missing and warn", so a single flake used to
/// ship a 404 into the docs — 8 in one build, still 4 after a rebuild.
async fn goto_with_retry(page: &Page, url: &str) -> Result<()> {
    let mut last_err = None;
    for attempt in 1..=GOTO_ATTEMPTS {
        match page.goto(url).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                tracing::debug!(url = %url, attempt, error = %e, "goto failed; retrying");
                last_err = Some(e);
                tokio::time::sleep(Duration::from_millis(500 * attempt as u64)).await;
            }
        }
    }
    Err(anyhow::anyhow!(last_err.expect("at least one attempt")))
        .with_context(|| format!("goto {url} failed after {GOTO_ATTEMPTS} attempts"))
}

/// Element bounds in document coordinates, as returned by
/// `element_document_rect`'s injected script.
#[derive(Debug, Clone, Copy, Deserialize)]
struct DocumentRect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

/// Screenshot a single element, clipped to its box.
///
/// Deliberately does NOT use chromiumoxide's `Element::screenshot`:
/// in 0.7 it adds the page scroll offset to the bounding box and then
/// adds it a *second* time when building the clip
/// (`chromiumoxide-0.7.0/src/element.rs:416-428`), so the clip lands
/// roughly `2 * scrollY` down the page. Anything the browser had to
/// scroll to reach — i.e. every marker below the fold — captured as a
/// blank white rectangle of the right size, which is how ~55 docs
/// screenshots shipped empty. Computing the rect in the page and
/// capturing with `captureBeyondViewport` avoids the offset math
/// entirely.
pub async fn capture_element_png(page: &Page, selector: &str) -> Result<Vec<u8>> {
    let rect = element_document_rect(page, selector).await?;
    if rect.width < 1.0 || rect.height < 1.0 {
        anyhow::bail!(
            "element {selector} has no rendered size ({}x{}); it is hidden or collapsed",
            rect.width,
            rect.height
        );
    }

    let clip = Viewport {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
        scale: 1.,
    };
    let png = page
        .screenshot(
            CaptureScreenshotParams::builder()
                .format(CaptureScreenshotFormat::Png)
                .clip(clip)
                // The clip is in document coordinates, so it routinely
                // sits outside the 1920x1080 viewport. Without this the
                // out-of-viewport part comes back blank.
                .capture_beyond_viewport(true)
                .build(),
        )
        .await
        .context("captureScreenshot")?;
    Ok(png)
}

/// Scroll `selector` into view and return its bounds in document
/// coordinates. The scroll mirrors puppeteer's
/// `elementHandle.screenshot()`, which the marker corpus was authored
/// against — lazy-loaded content inside a marker's target still needs
/// to be brought on-screen before capture.
async fn element_document_rect(page: &Page, selector: &str) -> Result<DocumentRect> {
    let js = format!(
        "(()=>{{const el=document.querySelector({});if(!el)return null;\
         el.scrollIntoView({{block:'center',inline:'nearest'}});\
         const r=el.getBoundingClientRect();\
         return {{x:r.left+window.scrollX,y:r.top+window.scrollY,width:r.width,height:r.height}};}})()",
        serde_json::to_string(selector).unwrap_or_else(|_| "''".to_string())
    );
    let rect: Option<DocumentRect> = page
        .evaluate(js)
        .await
        .with_context(|| format!("measure {selector}"))?
        .into_value()
        .with_context(|| format!("decode bounds of {selector}"))?;
    rect.with_context(|| format!("no element matched {selector}"))
}

async fn wait_for_selector(page: &Page, selector: &str, timeout: Duration) -> Result<()> {
    let start = tokio::time::Instant::now();
    loop {
        if page.find_element(selector).await.is_ok() {
            return Ok(());
        }
        if start.elapsed() > timeout {
            anyhow::bail!("timed out waiting for selector {selector}");
        }
        tokio::time::sleep(Duration::from_millis(150)).await;
    }
}

fn ensure_host(url: &str, host: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("{host}{url}")
    }
}

/// MD5 of `${url}-${selector}-${title}` -> hex. Mirrors
/// `src/app-screenshot-generator.js:203` (`crypto.createHash('md5')...`).
pub fn target_file_name(url: &str, selector: &str, title: &str) -> String {
    use md5::{Digest, Md5};
    let mut hasher = Md5::new();
    hasher.update(format!("{url}-{selector}-{title}").as_bytes());
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(32);
    for b in digest {
        hex.push_str(&format!("{b:02x}"));
    }
    format!("{hex}.png")
}

/// Build the `<div class="screenshot">...</div>` HTML wrapping the
/// captured image. Mirrors the template literal at lines 209-213.
pub fn render_template(args: &ScreenshotArgs, target_file_name: &str, host: &str) -> String {
    let remote_page = ensure_host(&args.url, host);
    let link_url_html = match &args.link_url {
        // `linkUrl: false` -> no link wrapper.
        Some(serde_json::Value::Bool(false)) => String::new(),
        // `linkUrl: "..."` -> link to that URL.
        Some(serde_json::Value::String(s)) => format!(
            "<div class=\"screenshot-link\"><a href=\"{href}\" target=\"_blank\"><img src=\"/images/link-external.png\" alt=\"External Link\" title=\"Go to This Page\"></a></div>",
            href = ensure_host(s, host),
        ),
        // unset/null -> link to remote_page.
        _ => format!(
            "<div class=\"screenshot-link\"><a href=\"{href}\" target=\"_blank\"><img src=\"/images/link-external.png\" alt=\"External Link\" title=\"Go to This Page\"></a></div>",
            href = remote_page,
        ),
    };
    let alt = if args.alt.trim().is_empty() {
        &args.title
    } else {
        &args.alt
    };
    format!(
        "<div class=\"screenshot\">\n        <div class=\"title\">{title}</div>\n        {link}<img src='/images/{file}' class=\"screenshot-image\" alt=\"{alt}\">\n    </div>",
        title = args.title,
        link = link_url_html,
        file = target_file_name,
        alt = escape_attr(alt),
    )
}

/// Escape a string for use inside a double-quoted HTML attribute.
fn escape_attr(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn png_byte_density_flags_flat_images() {
        // The exact shape of the shipped blanks: a 1570x31 checkbox row
        // that encoded to 312 bytes because it had nothing in it.
        let flat = fake_png(1570, 31, 312);
        let density = png_byte_density(&flat).expect("density");
        assert!(
            density < FLAT_PNG_BYTES_PER_PIXEL,
            "an empty checkbox row should read as flat, got {density}"
        );

        // Same element, actually rendered.
        let real = fake_png(1570, 31, 5984);
        let density = png_byte_density(&real).expect("density");
        assert!(
            density > FLAT_PNG_BYTES_PER_PIXEL,
            "a rendered checkbox row should not read as flat, got {density}"
        );
    }

    /// A buffer with a valid PNG signature and IHDR dimensions, padded
    /// to `len`. Only the header and the total size matter here.
    fn fake_png(width: u32, height: u32, len: usize) -> Vec<u8> {
        let mut png = Vec::with_capacity(len);
        png.extend_from_slice(b"\x89PNG\r\n\x1a\n");
        png.extend_from_slice(&[0u8; 8]); // IHDR length + chunk type
        png.extend_from_slice(&width.to_be_bytes());
        png.extend_from_slice(&height.to_be_bytes());
        png.resize(len, 0);
        png
    }

    #[test]
    fn png_byte_density_ignores_non_png() {
        assert!(png_byte_density(b"not a png at all, really not").is_none());
    }

    #[test]
    fn ensure_host_passes_through_absolute() {
        assert_eq!(
            ensure_host("https://example.com/x", "https://fastcomments.com"),
            "https://example.com/x"
        );
    }

    #[test]
    fn ensure_host_prefixes_relative() {
        assert_eq!(
            ensure_host("/auth/login", "https://fastcomments.com"),
            "https://fastcomments.com/auth/login"
        );
    }

    #[test]
    fn target_file_name_is_stable_md5() {
        let n = target_file_name(
            "/some/url",
            ".some-selector",
            "My Title",
        );
        assert!(n.ends_with(".png"));
        assert_eq!(n.len(), 32 + 4);
        // Same inputs -> same hash.
        let n2 = target_file_name("/some/url", ".some-selector", "My Title");
        assert_eq!(n, n2);
    }

    #[test]
    fn target_file_name_matches_node() {
        // Cross-checked via:
        //   node -e "console.log(require('crypto').createHash('md5').update('/auth/me-.profile-My Title').digest('hex'))"
        let name = target_file_name("/auth/me", ".profile", "My Title");
        // Precomputed Node md5 hex via `crypto.createHash('md5').update('/auth/me-.profile-My Title').digest('hex')`.
        assert_eq!(name, "0cecaa077e1596372b904b40a633b25b.png");
    }

    #[test]
    fn render_template_with_link_false_skips_link() {
        let args = ScreenshotArgs {
            url: "/p".into(),
            link_url: Some(serde_json::json!(false)),
            title: "T".into(),
            selector: "body".into(),
            ..Default::default()
        };
        let html = render_template(&args, "abc.png", HOST);
        assert!(html.contains("<div class=\"title\">T</div>"));
        assert!(!html.contains("screenshot-link"));
        assert!(html.contains("src='/images/abc.png'"));
    }

    #[test]
    fn render_template_defaults_to_self_link() {
        let args = ScreenshotArgs {
            url: "/p".into(),
            link_url: None,
            title: "T".into(),
            selector: "body".into(),
            ..Default::default()
        };
        let html = render_template(&args, "abc.png", HOST);
        assert!(html.contains("screenshot-link"));
        assert!(html.contains("https://fastcomments.com/p"));
    }

    #[test]
    fn render_template_emits_alt() {
        let args = ScreenshotArgs {
            url: "/p".into(),
            title: "T".into(),
            alt: "Widget customization page with the search box enabled".into(),
            selector: "body".into(),
            ..Default::default()
        };
        let html = render_template(&args, "abc.png", HOST);
        assert!(html
            .contains("alt=\"Widget customization page with the search box enabled\""));
    }

    #[test]
    fn render_template_alt_falls_back_to_title() {
        let args = ScreenshotArgs {
            url: "/p".into(),
            title: "The Import Page Form".into(),
            selector: "body".into(),
            ..Default::default()
        };
        let html = render_template(&args, "abc.png", HOST);
        assert!(html.contains("alt=\"The Import Page Form\""));
    }

    #[test]
    fn render_template_escapes_alt() {
        let args = ScreenshotArgs {
            url: "/p".into(),
            title: "T".into(),
            alt: "A \"quoted\" <tag> & more".into(),
            selector: "body".into(),
            ..Default::default()
        };
        let html = render_template(&args, "abc.png", HOST);
        assert!(html.contains("alt=\"A &quot;quoted&quot; &lt;tag&gt; &amp; more\""));
    }
}

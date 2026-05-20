//! Screenshot capture via chromiumoxide. Mirrors the behavior of
//! `src/app-screenshot-generator.js`:
//!
//! 1. Launch chromium with `--no-sandbox --disable-setuid-sandbox
//!    --enable-font-antialiasing`.
//! 2. Open a new page, set viewport, navigate to `<host>/auth/login`.
//! 3. Type `demo` into `input[name="username"]` and
//!    `demo@fastcomments.com` into `input[name="email"]`, submit.
//! 4. Wait for `body`.
//! 5. Navigate to the target URL, perform optional `actions`, optional
//!    `clickSelectors`, then wait for `selector`.
//! 6. Optional delay, then take an element screenshot of `selector` to
//!    the target PNG path.

use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result};
use chromiumoxide::browser::{Browser, BrowserConfigBuilder, HeadlessMode};
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
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
/// passed to `launch_logged_in`.
#[derive(Debug, Clone)]
pub struct ScreenshotHost {
    pub host: String,
    pub username: String,
    pub email: String,
}

impl Default for ScreenshotHost {
    fn default() -> Self {
        Self {
            host: HOST.to_string(),
            username: "demo".to_string(),
            email: "demo@fastcomments.com".to_string(),
        }
    }
}

/// Launch chromium with a fresh logged-in session and return the browser
/// along with the page handle. Caller is responsible for dropping the
/// browser when done (or using `BrowserGuard` below).
pub async fn launch_logged_in(
    width: u32,
    height: u32,
    host_cfg: &ScreenshotHost,
) -> Result<(Browser, Page, tokio::task::JoinHandle<()>)> {
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
    let cfg = builder.build().map_err(|e| anyhow::anyhow!(e))?;

    let (browser, mut handler) = Browser::launch(cfg).await.context("Browser::launch")?;
    let handler_task = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if let Err(e) = h {
                tracing::debug!(error = %e, "chromiumoxide handler tick");
            }
        }
    });

    let page = browser
        .new_page(format!("{}/auth/login", host_cfg.host))
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
    proxy_script: Option<&str>,
    proxy_style: Option<&str>,
) -> Result<()> {
    let url = ensure_host(&args.url, &host_cfg.host);
    page.goto(&url).await.with_context(|| format!("goto {url}"))?;

    if args.add_proxy_select {
        if let Some(s) = proxy_script {
            let _ = page.evaluate(s).await;
        }
        if let Some(css) = proxy_style {
            let inject_css = format!(
                "(()=>{{const s=document.createElement('style');s.textContent={};document.head.appendChild(s);}})()",
                serde_json::to_string(css).unwrap_or_else(|_| "''".to_string())
            );
            let _ = page.evaluate(inject_css).await;
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

    let element = page
        .find_element(&args.selector)
        .await
        .with_context(|| format!("find {} on {url}", args.selector))?;

    if let Some(parent) = target_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let png_bytes = element
        .screenshot(CaptureScreenshotFormat::Png)
        .await
        .context("element.screenshot")?;
    std::fs::write(target_path, png_bytes)
        .with_context(|| format!("write {target_path:?}"))?;

    Ok(())
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
    format!(
        "<div class=\"screenshot\">\n        <div class=\"title\">{title}</div>\n        {link}<img src='/images/{file}' class=\"screenshot-image\" >\n    </div>",
        title = args.title,
        link = link_url_html,
        file = target_file_name,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

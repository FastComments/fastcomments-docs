//! `sitegen build` — replaces `npm run build-content`.
//!
//! Per-locale: builds every guide page, every locale's home index page,
//! the sitemap.xml, and the build-id file.
//!
//! Mirrors `src/app.js` end-to-end + `src/guides.js:175-288` for the
//! buildGuide / buildGuideFromItems chain.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use fcdocs_browser::BrowserPool;
use fcdocs_shared::guides::{Guide, GuideMeta, GuidesRoot, MetaItem};
use fcdocs_shared::locales::Locales;
use fcdocs_shared::pipeline::full::{
    self, FullPipelineConfig, ScreenshotPlaceholder,
};
use fcdocs_shared::sidecar::SidecarClient;
use fcdocs_shared::sidecar_supervisor::Sidecar;
use fcdocs_shared::templates::TemplateRegistry;
use fcdocs_shared::translations::Translations;
use futures::stream::{FuturesUnordered, StreamExt};
use serde_json::{json, Value};
use tracing::{info, warn};


pub async fn run(args: Vec<String>) -> Result<()> {
    let repo = repo_root()?;
    let guides_dir = repo.join("src/content/guides");
    let snippets_dir = repo.join("src/snippets");
    let templates_dir = repo.join("src/templates");
    let static_generated_dir = repo.join("src/static/generated");
    let translations_path = repo.join("src/translations.json");
    let locales_path = repo.join("src/locales.json");
    let guide_order_path = guides_dir.join("guide-order.json");

    if !guides_dir.exists() {
        anyhow::bail!("guides dir missing: {guides_dir:?}");
    }
    std::fs::create_dir_all(&static_generated_dir)?;

    let locales = Arc::new(Locales::load_from(&locales_path)?);
    let translations = Arc::new(Translations::load_from(&translations_path, &locales.default_locale)?);
    let templates = Arc::new(TemplateRegistry::load_from(&templates_dir)?);
    let root = Arc::new(GuidesRoot::new(&guides_dir, &locales.default_locale));
    let guide_order: Vec<String> = serde_json::from_slice(
        &std::fs::read(&guide_order_path)
            .with_context(|| format!("read {guide_order_path:?}"))?,
    )?;

    let locale_filter = parse_locale_filter(args.iter().cloned());
    let locale_keys: Vec<String> = locales
        .keys()
        .filter(|k| locale_filter.as_ref().map_or(true, |f| f.iter().any(|x| x == k)))
        .map(|s| s.to_string())
        .collect();
    info!(locales = locale_keys.len(), "starting sitegen build");

    // Start the Node sidecar (just /highlight at this point — /eval-marker
    // is handled in-process by rquickjs but still served by the sidecar
    // until phase 6 cleanup).
    let sidecar_script = repo.join("src/content-sidecar.js");
    let sidecar_proc = Sidecar::spawn(&repo, &sidecar_script).await?;
    info!(url = %sidecar_proc.url, "content sidecar up");
    let sidecar = Arc::new(SidecarClient::new(sidecar_proc.url.clone()));

    // Two pipeline configs — same shape but `write_snippets` flips per
    // locale. Sitegen does default locale first (sequentially writes all
    // snippet pages), then the rest in parallel.
    let cfg_default = Arc::new(FullPipelineConfig {
        snippets_dir: snippets_dir.clone(),
        static_generated_dir: static_generated_dir.clone(),
        template_dir: templates_dir.clone(),
        write_snippets: true,
    });
    let cfg_other = Arc::new(FullPipelineConfig {
        snippets_dir: snippets_dir.clone(),
        static_generated_dir: static_generated_dir.clone(),
        template_dir: templates_dir.clone(),
        write_snippets: false,
    });
    let static_generated_dir = Arc::new(static_generated_dir);
    let guide_order = Arc::new(guide_order);

    // Pre-walk default-locale guides for sitemap + ordering metadata.
    let default_guides = root.walk(&locales.default_locale)?;
    register_link_anchors(&default_guides);
    let default_guides = Arc::new(default_guides);

    // Shared browser pool for screenshot capture. Lazily launches the
    // chrome process on first use; reused across every page+locale that
    // needs a fresh screenshot.
    let browser_pool = Arc::new(BrowserPool::new(fcdocs_browser::ScreenshotHost::default()));

    // Concurrency cap for guide×locale tasks. Defaults to logical CPUs;
    // overridable via SITEGEN_PARALLEL. Each task does I/O, JS eval, and
    // marker handling — CPU-bound enough to want parallelism, but not
    // memory-heavy enough to need careful tuning.
    let parallelism: usize = std::env::var("SITEGEN_PARALLEL")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|n: &usize| *n > 0)
        .unwrap_or_else(num_cpus::get);
    info!(parallelism, "guide build parallelism");

    let started = std::time::Instant::now();
    let mut total_pages = 0usize;

    // Phase A: default-locale pass, parallel across guides. This is the
    // pass that writes snippet pages (`code-*.html`); the file content
    // is then fixed for the rest of the build.
    let default_locale = locales.default_locale.clone();
    {
        let mut tasks = FuturesUnordered::new();
        let mut iter = default_guides.iter().cloned();
        while tasks.len() < parallelism {
            let Some(guide) = iter.next() else { break };
            tasks.push(spawn_guide_task(
                guide,
                default_locale.clone(),
                root.clone(),
                locales.clone(),
                translations.clone(),
                templates.clone(),
                cfg_default.clone(),
                sidecar.clone(),
                guide_order.clone(),
                static_generated_dir.clone(),
                browser_pool.clone(),
            ));
        }
        while let Some(joined) = tasks.next().await {
            match joined {
                Ok(Ok(())) => total_pages += 1,
                Ok(Err(e)) => warn!(error = %format!("{e:#}"), "skipping guide (default locale)"),
                Err(e) => warn!(error = %e, "guide task panicked (default locale)"),
            }
            if let Some(guide) = iter.next() {
                tasks.push(spawn_guide_task(
                    guide,
                    default_locale.clone(),
                    root.clone(),
                    locales.clone(),
                    translations.clone(),
                    templates.clone(),
                    cfg_default.clone(),
                    sidecar.clone(),
                    guide_order.clone(),
                    static_generated_dir.clone(),
                    browser_pool.clone(),
                ));
            }
        }
    }

    // Phase B: non-default locales, parallel across (guide × locale).
    // Snippet writing is suppressed on this pass.
    {
        let mut tasks = FuturesUnordered::new();
        let non_default: Vec<String> = locale_keys
            .iter()
            .filter(|k| **k != default_locale)
            .cloned()
            .collect();
        let mut iter = default_guides
            .iter()
            .cloned()
            .flat_map(|g| {
                let locales = non_default.clone();
                locales.into_iter().map(move |loc| (g.clone(), loc))
            });
        while tasks.len() < parallelism {
            let Some((guide, locale)) = iter.next() else { break };
            tasks.push(spawn_guide_task(
                guide,
                locale,
                root.clone(),
                locales.clone(),
                translations.clone(),
                templates.clone(),
                cfg_other.clone(),
                sidecar.clone(),
                guide_order.clone(),
                static_generated_dir.clone(),
                browser_pool.clone(),
            ));
        }
        while let Some(joined) = tasks.next().await {
            match joined {
                Ok(Ok(())) => total_pages += 1,
                Ok(Err(e)) => warn!(error = %format!("{e:#}"), "skipping guide"),
                Err(e) => warn!(error = %e, "guide task panicked"),
            }
            if let Some((guide, locale)) = iter.next() {
                tasks.push(spawn_guide_task(
                    guide,
                    locale,
                    root.clone(),
                    locales.clone(),
                    translations.clone(),
                    templates.clone(),
                    cfg_other.clone(),
                    sidecar.clone(),
                    guide_order.clone(),
                    static_generated_dir.clone(),
                    browser_pool.clone(),
                ));
            }
        }
    }

    // Build-id (random short id). Mirrors Node app.js:64 where the id is
    // generated ONCE per build and passed into every per-locale index
    // template render as `{{buildId}}` (the index template embeds it as
    // `?v={{buildId}}` on get-session-info.js, search.js, and
    // version-check.js). The same id is also written to the `build-id`
    // file at the end of the build, which version-check.js polls to
    // detect new deploys.
    //
    // Critically: build_id must be computed BEFORE the index loop, not
    // after it. Previously this was generated after the loop and the
    // index template baked the literal string "BUILD_ID_PLACEHOLDER"
    // into every page — JS cache never busted and version-check.js
    // always saw a constant value.
    //
    // Format: hex(seconds-since-epoch) + 4-char nanoid. Always contains
    // digits, monotonically increasing across builds, ~12 chars total.
    // scripts/compare-html.py's BUILD_ID_TOKEN_RE relies on the digit
    // guarantee to distinguish a real id from a literal placeholder
    // like "BUILD_ID_PLACEHOLDER".
    let build_id = format!(
        "{:x}{}",
        chrono::Utc::now().timestamp(),
        nanoid::nanoid!(4)
    );

    // Index pages per locale (cheap; serial is fine).
    for locale in &locale_keys {
        if let Err(e) = build_index_page(
            locale,
            &root,
            &locales,
            &translations,
            &templates,
            &guide_order,
            &static_generated_dir,
            &build_id,
        ) {
            warn!(locale, error = %format!("{e:#}"), "skipping index page");
        }
    }

    std::fs::write(static_generated_dir.join("build-id"), &build_id)?;

    // Sitemap.
    write_sitemap(&static_generated_dir, &default_guides, &locales)?;

    browser_pool.shutdown().await;
    sidecar_proc.shutdown().await;
    info!(
        pages = total_pages,
        elapsed = ?started.elapsed(),
        "sitegen build complete"
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn spawn_guide_task(
    guide: Guide,
    locale: String,
    root: Arc<GuidesRoot>,
    locales: Arc<Locales>,
    translations: Arc<Translations>,
    templates: Arc<TemplateRegistry>,
    cfg: Arc<FullPipelineConfig>,
    sidecar: Arc<SidecarClient>,
    guide_order: Arc<Vec<String>>,
    static_generated_dir: Arc<PathBuf>,
    browser_pool: Arc<BrowserPool>,
) -> tokio::task::JoinHandle<Result<()>> {
    tokio::spawn(async move {
        build_one_guide(
            &guide,
            &locale,
            &root,
            &locales,
            &translations,
            &templates,
            &cfg,
            &sidecar,
            &guide_order,
            &static_generated_dir,
            &browser_pool,
        )
        .await
    })
}

#[allow(clippy::too_many_arguments)]
async fn build_one_guide(
    guide: &Guide,
    locale: &str,
    root: &GuidesRoot,
    locales: &Locales,
    translations: &Translations,
    templates: &TemplateRegistry,
    cfg: &FullPipelineConfig,
    sidecar: &SidecarClient,
    guide_order: &[String],
    static_generated_dir: &Path,
    browser_pool: &BrowserPool,
) -> Result<()> {
    // Load meta — prefer locale-translated meta.
    let meta = load_meta_for_locale(root, &guide.id, locale)?;

    // Build items in order.
    let mut items: Vec<Value> = Vec::new();
    let mut any_fallback = false;
    for meta_item in &meta.items_ordered {
        let (item, fallback) = match build_one_item(
            &guide.id,
            meta_item,
            locale,
            root,
            cfg,
            sidecar,
            static_generated_dir,
            browser_pool,
        )
        .await
        {
            Ok(v) => v,
            Err(e) => {
                warn!(item = %meta_item.file, error = %e, "skip item");
                continue;
            }
        };
        if fallback {
            any_fallback = true;
        }
        items.push(item);
    }

    // Intro / conclusion.
    let intro_html = read_optional_markdown(
        &intro_path(root, &guide.id, locale),
        cfg,
        sidecar,
        static_generated_dir,
        browser_pool,
    )
    .await?;
    let conclusion_html = read_optional_markdown(
        &conclusion_path(root, &guide.id, locale),
        cfg,
        sidecar,
        static_generated_dir,
        browser_pool,
    )
    .await?;

    let t = translations.for_locale(locale);
    let available_locales = build_available_locales(locales, locale, |loc| {
        guide_link(&guide.id, loc, &locales.default_locale)
    });
    let alternate_locales = build_alternate_locales(locales, locale, |loc| {
        guide_link(&guide.id, loc, &locales.default_locale)
    });

    let url_encoded = urlencoding::encode(&format!(
        "https://docs.fastcomments.com/{}",
        guide_link(&guide.id, locale, &locales.default_locale)
    ))
    .into_owned();

    let prev_next = compute_prev_next(guide_order, &guide.id, &locales.default_locale);

    let items_by_sub_cat = group_by_sub_cat(&items);

    // Render guide-layout.html.
    let layout_ctx = json!({
        "id": guide.id,
        "name": meta.name.clone().unwrap_or_default(),
        "pageHeader": meta.page_header.clone().unwrap_or_default(),
        "icon": meta.icon.as_deref().map(|i| format!("images/guide-icons/{i}")),
        "url": guide_link(&guide.id, locale, &locales.default_locale),
        "urlEncoded": url_encoded,
        "prevGuideUrl": prev_next.prev,
        "nextGuideUrl": prev_next.next,
        "intro": intro_html,
        "items": items,
        "itemsBySubCat": items_by_sub_cat,
        "conclusion": conclusion_html,
        "isFallback": any_fallback,
        "locale": locale,
        "availableLocales": available_locales,
        "t": (*t.value).clone(),
        // stableUrlId intentionally NOT passed to guide-layout — Node
        // omits it from the layout context (`src/guides.js:238-248`),
        // so the `{{#if stableUrlId}}` block in guide-layout.html always
        // takes the `else` branch.
    });
    let guide_html = templates.render("guide-layout", &layout_ctx)?;
    // If the guide has an `index.md.html`, wrap the layout HTML through
    // it. Mirrors src/guides.js:249-252 — Node runs the file through
    // marked() (which wraps `{{{content}}}` in `<p>{{{content}}}</p>`)
    // then handlebars-compiles the result with content = guide_html.
    let guide_root_html = {
        let index_path = root.guides_dir.join(&guide.id).join("index.md.html");
        if index_path.exists() {
            let raw = std::fs::read_to_string(&index_path)?;
            let after_marked = render_inline_markdown(&raw);
            // handlebars-render the result with {{{content}}}.
            let mut hb = handlebars::Handlebars::new();
            hb.register_template_string("idx", &after_marked)?;
            hb.render("idx", &serde_json::json!({"content": &guide_html}))?
        } else {
            guide_html.clone()
        }
    };

    // Render page.html.
    //
    // canonicalUrl MUST be the default-locale URL on every page,
    // including translated ones. Mirrors src/guides.js:283 (`canonicalUrl:
    // defaultUrl`). Pointing each translated page's canonical at *itself*
    // tells search engines each locale is its own primary document and
    // neutralizes the hreflang model — risk of duplicate-content
    // penalties and lost cross-locale link equity. Reviewer caught the
    // regression because scripts/compare-html.py doesn't normalize the
    // canonical link, so this slipped a parity check.
    let default_url = guide_link(&guide.id, &locales.default_locale, &locales.default_locale);
    let canonical_url = default_url.clone();
    let faq_json_ld = build_faq_json_ld(&meta);
    let page_ctx = json!({
        "title": meta.page_header.clone().unwrap_or_else(|| meta.name.clone().unwrap_or_default()),
        "content": guide_root_html,
        "ExampleTenantId": full::EXAMPLE_TENANT_ID,
        "lang": locales.locales.get(locale).map(|l| l.hreflang.clone()).unwrap_or_default(),
        "locale": locale,
        "alternateLocales": alternate_locales,
        "availableLocales": available_locales,
        "defaultUrl": default_url,
        "description": meta.description(),
        "canonicalUrl": canonical_url,
        "faq": meta.faq_value(),
        "faqJsonLd": faq_json_ld,
        "stableUrlId": format!("/{}", default_url),
    });
    let final_html = templates.render("page", &page_ctx)?;

    let out_path = static_generated_dir.join(guide_link(&guide.id, locale, &locales.default_locale));
    std::fs::write(&out_path, final_html)
        .with_context(|| format!("write {out_path:?}"))?;
    Ok(())
}

async fn build_one_item(
    guide_id: &str,
    meta_item: &MetaItem,
    locale: &str,
    root: &GuidesRoot,
    cfg: &FullPipelineConfig,
    sidecar: &SidecarClient,
    static_generated_dir: &Path,
    browser_pool: &BrowserPool,
) -> Result<(Value, bool)> {
    let (path, fallback) = root.resolve_item_path(guide_id, &meta_item.file, locale);
    if !path.exists() {
        if meta_item.file.ends_with("-generated.md") {
            warn!(file = %meta_item.file, "skip missing generated file");
            anyhow::bail!("missing generated file");
        }
        anyhow::bail!("required file not found: {path:?}");
    }
    let raw = std::fs::read_to_string(&path)?;
    let basename = meta_item.file.trim_end_matches(".md");
    let processed = full::process_markdown(&raw, basename, cfg, sidecar).await?;

    // For each screenshot placeholder, capture the screenshot via
    // fcdocs-browser and replace the placeholder in the HTML.
    let html_with_screenshots = process_screenshots(
        processed.html,
        &processed.screenshots,
        static_generated_dir,
        browser_pool,
    )
    .await?;

    let id = meta_item.file.trim_end_matches(".md").to_string();
    let full_url = format!("/{}#{}", guide_link(guide_id, locale, &root.default_locale), id);
    let item_classes = if html_with_screenshots.contains("https://fastcomments.com") {
        "has-site-link"
    } else {
        ""
    };
    Ok((
        json!({
            "id": id,
            "title": meta_item.name,
            "name": meta_item.name,
            "file": meta_item.file,
            "subCat": meta_item.sub_cat,
            "sidebarItemClasses": meta_item.sidebar_item_classes,
            "fullUrl": full_url,
            "content": html_with_screenshots,
            "itemClasses": item_classes,
            "isFallback": fallback,
        }),
        fallback,
    ))
}

async fn process_screenshots(
    html: String,
    placeholders: &[ScreenshotPlaceholder],
    static_generated_dir: &Path,
    browser_pool: &BrowserPool,
) -> Result<String> {
    if placeholders.is_empty() {
        return Ok(html);
    }
    if std::env::var("SKIP_SCREENSHOTS").map(|v| v == "1").unwrap_or(false) {
        return Ok(html);
    }
    // The pipeline already inlined the `<div class="screenshot">` markup
    // (so pulldown-cmark sees it as block HTML, not text). All this
    // function does is capture/refresh the underlying PNG when the
    // image cache is stale.
    use fcdocs_browser::{cache::ImageCache, screenshot, ScreenshotArgs};
    let images_dir = static_generated_dir.join("images");
    std::fs::create_dir_all(&images_dir)?;
    let cache = ImageCache::new(static_generated_dir.join("image-cache"));

    for ph in placeholders {
        let args: ScreenshotArgs = match serde_json::from_value(ph.config.clone()) {
            Ok(a) => a,
            Err(e) => {
                warn!(error = %e, "skip malformed app-screenshot config");
                continue;
            }
        };
        let file_name = screenshot::target_file_name(&args.url, &args.selector, &args.title);
        let target_path = images_dir.join(&file_name);
        let args_json = build_cache_key_json(&ph.config);
        if !cache.is_stale(&args_json, &target_path, &file_name) {
            continue;
        }
        let width = args.width.unwrap_or(screenshot::DEFAULT_WIDTH);
        let url_for_log = args.url.clone();
        let cap_res = browser_pool
            .with_page(width, |page, host| {
                Box::pin(async move {
                    screenshot::capture(page, &args, &target_path, host, None, None).await
                })
            })
            .await;
        match cap_res {
            Ok(()) => {
                if let Err(e) = cache.update(&args_json, &file_name) {
                    warn!(error = %e, "image cache write failed");
                }
            }
            Err(e) => {
                warn!(url = %url_for_log, error = %format!("{e:#}"), "screenshot failed; HTML still references the (possibly missing) image");
            }
        }
    }
    Ok(html)
}

/// Build the exact JSON shape Node uses for its image-cache key
/// (`{url, linkUrl, width, actions, clickSelectors, selector, title,
/// addProxySelect, cacheBuster}`), in the same insertion order, with
/// fields the script didn't set dropped entirely (mirrors V8 shorthand
/// destructuring → `undefined` → `JSON.stringify` omits the key).
fn build_cache_key_json(parsed: &serde_json::Value) -> String {
    use serde_json::Value;
    let mut map = serde_json::Map::new();
    let order = [
        "url",
        "linkUrl",
        "width",
        "actions",
        "clickSelectors",
        "selector",
        "title",
        "addProxySelect",
        "cacheBuster",
    ];
    for key in order {
        if let Some(v) = parsed.get(key) {
            map.insert(key.to_string(), v.clone());
        }
    }
    serde_json::to_string(&Value::Object(map)).unwrap_or_default()
}

async fn read_optional_markdown(
    path: &Option<PathBuf>,
    cfg: &FullPipelineConfig,
    sidecar: &SidecarClient,
    static_generated_dir: &Path,
    browser_pool: &BrowserPool,
) -> Result<String> {
    let Some(p) = path else { return Ok(String::new()) };
    let raw = std::fs::read_to_string(p)?;
    // For intro/conclusion the basename comes from the file (e.g.
    // `intro.md` -> `intro`). Markers in intro/conclusion are rare but
    // possible; this keeps snippet IDs unique per file.
    let basename = p
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("intro");
    // No hljs style block for intro/conclusion — Node calls marked()
    // directly for those at src/guides.js:204 / 214 without the
    // post-item style append.
    let processed = full::process_markdown_with(&raw, basename, cfg, sidecar, false).await?;
    let html = process_screenshots(
        processed.html,
        &processed.screenshots,
        static_generated_dir,
        browser_pool,
    )
    .await?;
    Ok(html)
}

fn build_available_locales(
    locales: &Locales,
    current: &str,
    url_for: impl Fn(&str) -> String,
) -> Value {
    let mut out = Vec::new();
    for (code, info) in &locales.locales {
        out.push(json!({
            "code": code,
            "name": info.name,
            "nativeName": info.native_name,
            "flag": info.flag.clone().unwrap_or_else(|| "🌐".to_string()),
            "url": url_for(code),
            "current": code == current,
        }));
    }
    Value::Array(out)
}

fn build_alternate_locales(
    locales: &Locales,
    current: &str,
    url_for: impl Fn(&str) -> String,
) -> Value {
    let mut out = Vec::new();
    for (code, info) in &locales.locales {
        out.push(json!({
            "hreflang": info.hreflang,
            "url": url_for(code),
            "current": code == current,
        }));
    }
    Value::Array(out)
}

fn group_by_sub_cat(items: &[Value]) -> Value {
    let mut groups: Vec<(String, Vec<Value>)> = Vec::new();
    for item in items {
        let key = item
            .get("subCat")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if let Some(g) = groups.iter_mut().find(|(k, _)| k == &key) {
            g.1.push(item.clone());
        } else {
            groups.push((key, vec![item.clone()]));
        }
    }
    let mut obj = serde_json::Map::new();
    for (k, v) in groups {
        obj.insert(k, Value::Array(v));
    }
    Value::Object(obj)
}

fn build_faq_json_ld(meta: &GuideMeta) -> Option<String> {
    let faq = meta.extra.get("faq")?;
    let arr = faq.as_array()?;
    if arr.is_empty() {
        return None;
    }
    let main_entity: Vec<Value> = arr
        .iter()
        .filter_map(|item| {
            let q = item.get("question")?.as_str()?;
            let a = item.get("answer")?.as_str()?;
            Some(json!({
                "@type": "Question",
                "name": q,
                "acceptedAnswer": {"@type": "Answer", "text": a}
            }))
        })
        .collect();
    let json = json!({
        "@context": "https://schema.org",
        "@type": "FAQPage",
        "mainEntity": main_entity,
    })
    .to_string();
    // The page template embeds this string raw inside
    // `<script type="application/ld+json">`. Replace `</` with `<\/`
    // and `<!--` with `<\!--` so a question or answer that contains
    // literal `</script>` (or HTML comment markers) can't break out
    // of the script context. Both are valid JSON escapes — the JSON
    // parser unescapes them transparently.
    Some(json.replace("</", "<\\/").replace("<!--", "<\\!--"))
}

struct PrevNext {
    prev: Option<String>,
    next: Option<String>,
}

fn compute_prev_next(order: &[String], current: &str, default_locale: &str) -> PrevNext {
    let idx = order.iter().position(|g| g == current);
    let prev = idx
        .and_then(|i| if i > 0 { Some(&order[i - 1]) } else { None })
        .map(|g| guide_link(g, default_locale, default_locale));
    let next = idx
        .and_then(|i| if i + 1 < order.len() { Some(&order[i + 1]) } else { None })
        .map(|g| guide_link(g, default_locale, default_locale));
    PrevNext { prev, next }
}

fn intro_path(root: &GuidesRoot, guide_id: &str, locale: &str) -> Option<PathBuf> {
    root.intro_path(guide_id, locale)
}

fn conclusion_path(root: &GuidesRoot, guide_id: &str, locale: &str) -> Option<PathBuf> {
    let candidates = [
        root.guides_dir
            .join(guide_id)
            .join("items")
            .join(locale)
            .join("conclusion.md"),
        root.guides_dir
            .join(guide_id)
            .join("items")
            .join(&root.default_locale)
            .join("conclusion.md"),
        root.guides_dir.join(guide_id).join("conclusion.md"),
    ];
    candidates.into_iter().find(|p| p.exists())
}

fn load_meta_for_locale(root: &GuidesRoot, guide_id: &str, locale: &str) -> Result<GuideMeta> {
    let translated = root
        .guides_dir
        .join(guide_id)
        .join("meta_translated")
        .join(format!("meta_{locale}.json"));
    let path = if translated.exists() {
        translated
    } else {
        root.guides_dir.join(guide_id).join("meta.json")
    };
    let bytes = std::fs::read(&path).with_context(|| format!("read {path:?}"))?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub fn guide_link(id: &str, locale: &str, default_locale: &str) -> String {
    if locale == default_locale {
        format!("guide-{id}.html")
    } else {
        format!("guide-{id}-{locale}.html")
    }
}

#[allow(clippy::too_many_arguments)]
fn build_index_page(
    locale: &str,
    root: &GuidesRoot,
    locales: &Locales,
    translations: &Translations,
    templates: &TemplateRegistry,
    guide_order: &[String],
    static_generated_dir: &Path,
    build_id: &str,
) -> Result<()> {
    let guides = root.walk(locale)?;
    let t = translations.for_locale(locale);
    let local_index_url = if locale == locales.default_locale {
        "index.html".to_string()
    } else {
        format!("index-{locale}.html")
    };
    let _ = local_index_url; // for symmetry/log

    let getting_started: Vec<&Guide> = guides
        .iter()
        .filter(|g| guide_order.contains(&g.id))
        .collect();
    let mut getting_started_sorted: Vec<&Guide> = getting_started.into_iter().collect();
    getting_started_sorted.sort_by_key(|g| {
        guide_order
            .iter()
            .position(|x| x == &g.id)
            .unwrap_or(usize::MAX)
    });

    fn guide_to_value(g: &Guide, url: String) -> Value {
        let icon = g
            .meta
            .icon
            .as_deref()
            .map(|i| format!("images/guide-icons/{i}"));
        json!({
            "id": g.id,
            "name": g.meta.name.clone().unwrap_or_else(|| g.id.clone()),
            "url": url,
            "icon": icon,
        })
    }

    let localize = |guides: &[&Guide]| -> Vec<Value> {
        guides
            .iter()
            .map(|g| guide_to_value(g, guide_link(&g.id, locale, &locales.default_locale)))
            .collect()
    };

    let guides_not_install: Vec<&Guide> = guides
        .iter()
        .filter(|g| {
            !g.id.starts_with("installation-")
                && !g.id.starts_with("sdk-")
                && !g.id.starts_with("lib-")
        })
        .collect();
    let installation: Vec<&Guide> =
        guides.iter().filter(|g| g.id.starts_with("installation-")).collect();
    let sdk: Vec<&Guide> = guides
        .iter()
        .filter(|g| g.id.starts_with("sdk-") || g.id.starts_with("lib-"))
        .collect();

    let available_locales = build_available_locales(locales, locale, |loc| {
        if loc == locales.default_locale {
            "index.html".to_string()
        } else {
            format!("index-{loc}.html")
        }
    });

    let last_update_date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let ctx = json!({
        "guides": localize(&guides_not_install),
        "installationGuides": localize(&installation),
        "sdkGuides": localize(&sdk),
        "gettingStartedGuides": localize(&getting_started_sorted),
        "lastUpdateDate": last_update_date,
        "buildId": build_id,
        "locale": locale,
        "lang": locales.locales.get(locale).map(|l| l.hreflang.clone()).unwrap_or_default(),
        "availableLocales": available_locales,
        "t": (*t.value).clone(),
    });
    let html = templates.render("index", &ctx)?;
    let filename = if locale == locales.default_locale {
        "index.html".to_string()
    } else {
        format!("index-{locale}.html")
    };
    std::fs::write(static_generated_dir.join(filename), html)?;
    Ok(())
}

fn write_sitemap(
    static_generated_dir: &Path,
    guides: &[Guide],
    locales: &Locales,
) -> Result<()> {
    const BASE: &str = "https://docs.fastcomments.com/";
    let mut sitemap = String::new();
    sitemap.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    sitemap.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\"\n");
    sitemap.push_str("        xmlns:xhtml=\"http://www.w3.org/1999/xhtml\">\n");

    // Homepage entries per locale.
    for (loc, _) in &locales.locales {
        let filename = if loc == &locales.default_locale {
            "index.html".to_string()
        } else {
            format!("index-{loc}.html")
        };
        sitemap.push_str("  <url>\n");
        sitemap.push_str(&format!("    <loc>{BASE}{filename}</loc>\n"));
        for (alt, info) in &locales.locales {
            let alt_file = if alt == &locales.default_locale {
                "index.html".to_string()
            } else {
                format!("index-{alt}.html")
            };
            sitemap.push_str(&format!(
                "    <xhtml:link rel=\"alternate\" hreflang=\"{}\" href=\"{BASE}{alt_file}\"/>\n",
                info.hreflang
            ));
        }
        sitemap.push_str("  </url>\n");
    }

    // Guide pages.
    for g in guides {
        if g.id.starts_with("code-") {
            continue;
        }
        for (loc, _) in &locales.locales {
            let url = guide_link(&g.id, loc, &locales.default_locale);
            sitemap.push_str("  <url>\n");
            sitemap.push_str(&format!("    <loc>{BASE}{url}</loc>\n"));
            for (alt, info) in &locales.locales {
                let alt_url = guide_link(&g.id, alt, &locales.default_locale);
                sitemap.push_str(&format!(
                    "    <xhtml:link rel=\"alternate\" hreflang=\"{}\" href=\"{BASE}{alt_url}\"/>\n",
                    info.hreflang
                ));
            }
            sitemap.push_str("  </url>\n");
        }
    }
    sitemap.push_str("</urlset>\n");

    let bytes = sitemap.as_bytes().len();
    const MAX_BYTES: usize = 50 * 1024 * 1024;
    if bytes > MAX_BYTES {
        anyhow::bail!("sitemap exceeds 50 MB ({} bytes)", bytes);
    }
    std::fs::write(static_generated_dir.join("sitemap.xml"), sitemap)?;
    Ok(())
}

fn register_link_anchors(_guides: &[Guide]) {
    // Placeholder — full link-validator wiring lands in a follow-up once
    // we decide whether to enforce or just warn during sitegen runs.
}

fn parse_locale_filter(args: impl Iterator<Item = String>) -> Option<Vec<String>> {
    let raw: Option<Vec<String>> = {
        let mut iter = args.peekable();
        let mut out = None;
        while let Some(arg) = iter.next() {
            if arg == "--locale" || arg == "--locales" {
                out = iter.next().map(|v| {
                    v.split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                });
                break;
            } else if let Some(rest) = arg.strip_prefix("--locale=") {
                out = Some(
                    rest.split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect(),
                );
                break;
            }
        }
        out
    };
    // Validate each provided locale flows into Path::join in many sites
    // — reject anything outside the locale-id allowlist before we reach
    // the filesystem. Drop invalid entries with a warning so a typo
    // doesn't accidentally widen the build to everything (which
    // returning `None` would do).
    raw.map(|locales| {
        let (ok, bad): (Vec<String>, Vec<String>) =
            locales.into_iter().partition(|l| fcdocs_shared::guides::is_valid_id(l));
        for b in &bad {
            warn!(locale = %b, "ignoring --locale entry: invalid id");
        }
        ok
    })
}

/// Run pulldown-cmark on the input. Used for `index.md.html` files
/// which are typically a single `{{{content}}}` token that marked wraps
/// in `<p>...</p>`. Matches Node's `marked()` default behavior.
fn render_inline_markdown(input: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let opts = Options::ENABLE_TABLES;
    let parser = Parser::new_ext(input, opts);
    let mut out = String::with_capacity(input.len() + 16);
    html::push_html(&mut out, parser);
    out
}

pub fn repo_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let mut cur: &Path = cwd.as_path();
    loop {
        if cur.join("package.json").exists() && cur.join("src/locales.json").exists() {
            return Ok(cur.to_path_buf());
        }
        match cur.parent() {
            Some(p) => cur = p,
            None => anyhow::bail!("could not locate repo root from {cwd:?}"),
        }
    }
}

// ------------------------------------------------------------------
// GuideMeta accessor helpers
// ------------------------------------------------------------------

trait GuideMetaExt {
    fn description(&self) -> Option<String>;
    fn faq_value(&self) -> Option<Value>;
}

impl GuideMetaExt for GuideMeta {
    fn description(&self) -> Option<String> {
        self.extra
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    fn faq_value(&self) -> Option<Value> {
        self.extra.get("faq").cloned()
    }
}

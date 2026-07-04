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
    let guide_filter = parse_guide_filter(args.iter().cloned());
    let locale_keys: Vec<String> = locales
        .keys()
        .filter(|k| locale_filter.as_ref().map_or(true, |f| f.iter().any(|x| x == k)))
        .map(|s| s.to_string())
        .collect();
    info!(
        locales = locale_keys.len(),
        guides = ?guide_filter,
        "starting sitegen build"
    );

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
    // `all_guides` always holds the FULL guide set — used by index +
    // sitemap + link validator so an incremental --guide build doesn't
    // drop links to the unfiltered guides.
    let all_guides = root.walk(&locales.default_locale)?;
    // Filtered set used for the per-guide HTML build loops. When
    // --guide is absent this is just a clone of all_guides; the
    // watcher passes the flag in dev to skip rebuilding untouched
    // guides.
    let selected_guides: Vec<Guide> = match &guide_filter {
        Some(allow) => {
            let filtered: Vec<Guide> = all_guides
                .iter()
                .filter(|g| allow.iter().any(|id| id == &g.id))
                .cloned()
                .collect();
            if filtered.is_empty() {
                warn!(
                    requested = ?allow,
                    "--guide filter matched no guides; nothing to rebuild"
                );
            } else {
                let names: Vec<&str> = filtered.iter().map(|g| g.id.as_str()).collect();
                info!(building = ?names, "--guide filter active");
            }
            filtered
        }
        None => all_guides.clone(),
    };
    // Build the link validator BEFORE any per-guide tasks run, so every
    // guide's item set is registered when validate() fires. Mirrors
    // src/guides.js:322 (`linkValidator.registerGuideItems(...)` walked
    // before the per-guide build loop). Always uses the FULL set so a
    // partial rebuild doesn't false-positive on links pointing at
    // unfiltered guides.
    let link_validator = Arc::new(build_link_validator(&all_guides));
    // Shared bag of broken-link errors. Built across all parallel
    // default-locale workers and drained at end-of-run.
    let link_errors: Arc<std::sync::Mutex<Vec<fcdocs_shared::link_validator::LinkError>>> =
        Arc::new(std::sync::Mutex::new(Vec::new()));
    let all_guides = Arc::new(all_guides);
    let selected_guides = Arc::new(selected_guides);

    // Shared browser pool for screenshot capture. Lazily launches the
    // chrome process on first use; reused across every page+locale that
    // needs a fresh screenshot. The host config carries the optional
    // proxy-select assets — markers that set `addProxySelect=true`
    // need them injected to make `<select>` dropdowns visible in
    // screenshots (mirrors Node addProxySelectToPage). Missing files
    // degrade gracefully: the marker still captures, just without the
    // styled dropdown overlay (capture() logs a warn if it runs).
    let browser_pool = Arc::new(BrowserPool::new(load_screenshot_host(&repo)));

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
    let default_locale_selected =
        locale_filter.as_ref().map_or(true, |f| f.iter().any(|l| l == &default_locale));
    {
        if !default_locale_selected {
            info!(
                locale = %default_locale,
                "--locale filter excludes default locale; skipping phase A"
            );
        }
        let mut tasks = FuturesUnordered::new();
        let mut iter: Box<dyn Iterator<Item = Guide> + Send> = if default_locale_selected {
            Box::new(selected_guides.iter().cloned())
        } else {
            Box::new(std::iter::empty())
        };
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
                link_validator.clone(),
                link_errors.clone(),
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
                    link_validator.clone(),
                    link_errors.clone(),
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
        let mut iter = selected_guides
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
                link_validator.clone(),
                link_errors.clone(),
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
                    link_validator.clone(),
                    link_errors.clone(),
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

    // Sitemap. Always emit the full guide set — an incremental --guide
    // build still needs the sitemap to reference every page on the
    // site, not just the rebuilt ones.
    write_sitemap(&static_generated_dir, &all_guides, &locales)?;

    browser_pool.shutdown().await;
    sidecar_proc.shutdown().await;
    info!(
        pages = total_pages,
        elapsed = ?started.elapsed(),
        "sitegen build complete"
    );

    // Drain accumulated link-validation errors. Node throws on the
    // first broken anchor (src/link-validator.js:73); we collect across
    // the build so the developer sees every broken link in one pass,
    // then bail non-zero. Matches the sdkgen openapi pattern.
    let errors = std::mem::take(&mut *link_errors.lock().expect("link_errors mutex"));
    if !errors.is_empty() {
        eprintln!();
        for e in &errors {
            eprintln!("{e}");
        }
        eprintln!(
            "=== BUILD FAILED: {n} broken link(s) in default-locale content ===",
            n = errors.len()
        );
        anyhow::bail!("sitegen: {n} broken link(s)", n = errors.len());
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
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
    link_validator: Arc<fcdocs_shared::link_validator::LinkValidator>,
    link_errors: Arc<std::sync::Mutex<Vec<fcdocs_shared::link_validator::LinkError>>>,
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
            &link_validator,
            &link_errors,
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
    link_validator: &fcdocs_shared::link_validator::LinkValidator,
    link_errors: &std::sync::Mutex<Vec<fcdocs_shared::link_validator::LinkError>>,
) -> Result<()> {
    // Load meta — prefer locale-translated meta.
    let meta = load_meta_for_locale(root, &guide.id, locale)?;

    // Link validation runs only on the default locale (matches
    // src/guides.js:137). Non-default locales would either re-flag the
    // same broken anchors against the inherited default-locale content
    // or false-positive against locale-specific translations of the
    // file basenames.
    let validate_links = locale == root.default_locale;

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
            validate_links.then_some((link_validator, link_errors)),
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
        // stableUrlId MUST be passed so the main comments widget keys its
        // thread on the default-locale URL, staying stable across locales
        // and matching the floating page-likes widget in page.html. Without
        // it the `{{#if stableUrlId}}` block falls back to
        // window.location.pathname, so localized (and even the -en_us
        // suffixed) pages load an empty thread while the floating widget
        // still shows the count from the stable urlId.
        "stableUrlId": format!(
            "/{}",
            guide_link(&guide.id, &locales.default_locale, &locales.default_locale)
        ),
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

#[allow(clippy::too_many_arguments)]
async fn build_one_item(
    guide_id: &str,
    meta_item: &MetaItem,
    locale: &str,
    root: &GuidesRoot,
    cfg: &FullPipelineConfig,
    sidecar: &SidecarClient,
    static_generated_dir: &Path,
    browser_pool: &BrowserPool,
    link_check: Option<(
        &fcdocs_shared::link_validator::LinkValidator,
        &std::sync::Mutex<Vec<fcdocs_shared::link_validator::LinkError>>,
    )>,
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

    // Validate links on raw markdown BEFORE marker / handlebars
    // substitution — same point Node's pipeline runs the check
    // (src/guides.js:138). Errors are collected, not thrown, so the
    // build surfaces every broken anchor in one pass at end-of-run.
    if let Some((validator, errors)) = link_check {
        let path_for_msg = path.to_string_lossy().into_owned();
        let new_errs = validator.validate(&raw, &path_for_msg, guide_id);
        if !new_errs.is_empty() {
            errors.lock().expect("link_errors mutex").extend(new_errs);
        }
    }

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
                    screenshot::capture(page, &args, &target_path, host).await
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
///
/// One non-obvious normalization: Node's
/// `src/app-screenshot-generator.js:330` wraps the singular
/// `args.clickSelector` into `[args.clickSelector]` BEFORE it reaches
/// the `clickSelectors` slot of the cache key. ~350 markers in the
/// corpus use the singular form; without this normalization Rust
/// omits the field entirely and every Node-built cache entry misses
/// on the first Rust build (chromium re-takes the screenshot, then
/// writes a different key shape).
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
        if key == "clickSelectors" {
            // Match Node's ternary at app-screenshot-generator.js:330:
            //   args.clickSelector ? [args.clickSelector] : args.clickSelectors
            // When singular is set it OVERRIDES plural (the plural
            // branch is never consulted).
            if let Some(single) = parsed.get("clickSelector") {
                map.insert(
                    "clickSelectors".to_string(),
                    Value::Array(vec![single.clone()]),
                );
                continue;
            }
            if let Some(v) = parsed.get("clickSelectors") {
                map.insert("clickSelectors".to_string(), v.clone());
            }
            continue;
        }
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

    // Per src/guides.js:337 a guide with no itemsOrdered uses meta.url
    // (verbatim) as its homepage card link. The intent is "this card
    // redirects users to a section inside another guide" — e.g. SSO's
    // meta.url is `/guide-customizations-and-configuration.html#sso`.
    // Without this, Rust always emitted `guide-sso.html` which still
    // exists as a near-empty page but skips the cross-reference target
    // that Node intentionally pointed at.
    //
    // Node does not interpolate the locale into meta.url (the field is
    // a literal path); we match that, even though it means non-default
    // locales also link at the default-locale URL. Changing that is a
    // separate design decision that would diverge from Node parity.
    let localize = |guides: &[&Guide]| -> Vec<Value> {
        guides
            .iter()
            .map(|g| {
                let url = if g.meta.items_ordered.is_empty() {
                    g.meta.url.clone().unwrap_or_else(|| "#".to_string())
                } else {
                    guide_link(&g.id, locale, &locales.default_locale)
                };
                guide_to_value(g, url)
            })
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

/// Compute the sitemap URL count using the same formula as Node
/// `src/app.js:176`:
///
/// > allLocaleKeys.length + guides.filter(g => !g.id.startsWith('code-')).length * allLocaleKeys.length
///
/// = (1 + non_code_guides) × locales. Lifted into a free function so
/// the URL cap can be tested without building the full sitemap string.
fn sitemap_url_count(guides: &[Guide], locales: &Locales) -> usize {
    let locale_count = locales.locales.len();
    let non_code_guides = guides.iter().filter(|g| !g.id.starts_with("code-")).count();
    locale_count + non_code_guides * locale_count
}

fn write_sitemap(
    static_generated_dir: &Path,
    guides: &[Guide],
    locales: &Locales,
) -> Result<()> {
    const BASE: &str = "https://docs.fastcomments.com/";
    // Sitemap protocol cap, mirrored from Node src/app.js:178-185. We
    // check URL count BEFORE building the string so a runaway config
    // (eg: someone adds a third dimension to the URL space) fails
    // fast rather than buffering 50+ MB of pointless XML first.
    const MAX_URLS: usize = 50_000;
    const MAX_BYTES: usize = 50 * 1024 * 1024;
    let url_count = sitemap_url_count(guides, locales);
    if url_count > MAX_URLS {
        anyhow::bail!(
            "sitemap exceeds max URL count: {url_count} > {MAX_URLS}"
        );
    }

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
    if bytes > MAX_BYTES {
        anyhow::bail!(
            "sitemap exceeds max file size: {:.1}MB > 50MB",
            bytes as f64 / 1024.0 / 1024.0
        );
    }
    std::fs::write(static_generated_dir.join("sitemap.xml"), sitemap)?;
    info!(
        urls = url_count,
        mb = format!("{:.1}", bytes as f64 / 1024.0 / 1024.0),
        "sitemap written"
    );
    Ok(())
}

/// Build a `LinkValidator` from the registered guides, mirroring Node's
/// `linkValidator.registerGuideItems(guide.id, meta.itemsOrdered)` loop
/// at src/guides.js:320-323.
///
/// Each item is keyed by its filename minus `.md` — link validation in
/// the validator compares those keys against `[text](#foo)` /
/// `[text](./foo.md)` / `[text](/guide-X.html#foo)` link components.
fn build_link_validator(guides: &[Guide]) -> fcdocs_shared::link_validator::LinkValidator {
    let mut v = fcdocs_shared::link_validator::LinkValidator::new();
    for g in guides {
        v.register_guide_items(&g.id, g.meta.items_ordered.iter().map(|it| it.file.as_str()));
    }
    v
}

fn parse_locale_filter(args: impl Iterator<Item = String>) -> Option<Vec<String>> {
    parse_csv_filter(args, &["--locale", "--locales"], "locale")
}

/// Watcher passes `--guide=foo,bar` so a content-file change only
/// rebuilds the affected guide(s) instead of the full 28-locale × 15-
/// guide matrix. Index + sitemap still iterate the full guide set so
/// the unfiltered guides' links stay valid.
fn parse_guide_filter(args: impl Iterator<Item = String>) -> Option<Vec<String>> {
    parse_csv_filter(args, &["--guide", "--guides"], "guide")
}

/// Common shape for `--<name>=a,b,c` / `--<name> a,b,c` filters.
/// Returns None when the flag isn't present (treat as "no filter"),
/// `Some(vec)` otherwise. The CSV is validated against
/// [`fcdocs_shared::guides::is_valid_id`] since the values end up in
/// `Path::join` — invalid entries are dropped with a warning rather
/// than widening the filter to "everything".
fn parse_csv_filter(
    args: impl Iterator<Item = String>,
    flag_names: &[&str],
    label: &'static str,
) -> Option<Vec<String>> {
    let raw: Option<Vec<String>> = {
        let mut iter = args.peekable();
        let mut out = None;
        'outer: while let Some(arg) = iter.next() {
            for name in flag_names {
                if arg == *name {
                    out = iter.next().map(split_csv);
                    break 'outer;
                }
                let prefix = format!("{name}=");
                if let Some(rest) = arg.strip_prefix(&prefix) {
                    out = Some(split_csv(rest.to_string()));
                    break 'outer;
                }
            }
        }
        out
    };
    raw.map(|values| {
        let (ok, bad): (Vec<String>, Vec<String>) = values
            .into_iter()
            .partition(|v| fcdocs_shared::guides::is_valid_id(v));
        for b in &bad {
            warn!(value = %b, kind = label, "ignoring CSV filter entry: invalid id");
        }
        ok
    })
}

fn split_csv(v: String) -> Vec<String> {
    v.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
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

/// Build a [`fcdocs_browser::ScreenshotHost`] populated with the
/// proxy-select script + style read from `src/static/`. Each missing
/// asset is logged at WARN and left as `None`; downstream
/// `screenshot::capture` calls then degrade gracefully (the marker
/// still captures, just without the styled-dropdown overlay).
///
/// Mirrors the behavior of Node `addProxySelectToPage`
/// (src/app-screenshot-generator.js:16-21). Lifted to a free function
/// so a unit test can pin "post-fix the assets are actually loaded"
/// without spinning up sitegen.
fn load_screenshot_host(repo: &Path) -> fcdocs_browser::ScreenshotHost {
    let mut host = fcdocs_browser::ScreenshotHost::default();
    let script_path = repo.join("src/static/js/proxy-select.js");
    match std::fs::read_to_string(&script_path) {
        Ok(s) => host.proxy_script = Some(Arc::new(s)),
        Err(e) => warn!(
            path = %script_path.display(),
            error = %e,
            "proxy-select.js missing; addProxySelect markers will degrade"
        ),
    }
    let style_path = repo.join("src/static/css/proxy-select.css");
    match std::fs::read_to_string(&style_path) {
        Ok(s) => host.proxy_style = Some(Arc::new(s)),
        Err(e) => warn!(
            path = %style_path.display(),
            error = %e,
            "proxy-select.css missing; addProxySelect markers will degrade"
        ),
    }
    host
}

pub use fcdocs_shared::repo::repo_root;

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

#[cfg(test)]
mod sitemap_cap_tests {
    use super::*;
    use fcdocs_shared::locales::Locale;
    use indexmap::IndexMap;

    fn loc(name: &str) -> Locale {
        Locale {
            name: name.to_string(),
            native_name: name.to_string(),
            hreflang: name.replace('_', "-"),
            flag: None,
        }
    }

    fn locales_with(n: usize) -> Locales {
        let mut map: IndexMap<String, Locale> = IndexMap::new();
        for i in 0..n {
            map.insert(format!("l{i}"), loc(&format!("l{i}")));
        }
        Locales {
            default_locale: "l0".to_string(),
            locales: map,
        }
    }

    fn guide(id: &str) -> Guide {
        Guide {
            id: id.to_string(),
            meta: GuideMeta::default(),
            items_dir: std::path::PathBuf::new(),
        }
    }

    /// Mirrors Node's formula at src/app.js:176 verbatim:
    ///   allLocaleKeys.length + non_code_guides * allLocaleKeys.length
    /// = (1 + non_code_guides) * locales
    #[test]
    fn url_count_matches_node_formula() {
        let l = locales_with(28);
        let g: Vec<Guide> = (0..15).map(|i| guide(&format!("g{i}"))).collect();
        // (1 + 15) * 28 = 448
        assert_eq!(sitemap_url_count(&g, &l), 448);
    }

    /// `code-*` guides are excluded from the sitemap (Node:
    /// `guides.filter(g => !g.id.startsWith('code-'))`). Pre-fix Rust
    /// didn't enforce a URL cap at all, so this property wasn't
    /// pinned anywhere.
    #[test]
    fn code_prefix_guides_excluded_from_count() {
        let l = locales_with(2);
        let g = vec![
            guide("real-1"),
            guide("code-snippet-x"),
            guide("real-2"),
            guide("code-snippet-y"),
        ];
        // 2 locales + 2 real guides * 2 locales = 6
        assert_eq!(sitemap_url_count(&g, &l), 6);
    }

    /// Pre-fix: only the 50 MB byte cap fired. The URL cap (50000) is
    /// the protocol limit and would have bailed Node's build first
    /// for any config that drifted high enough.
    #[test]
    fn cap_bails_on_overflow() {
        // (1 + 4999) * 11 = 55000 > 50000 — trips URL cap, NOT byte
        // cap (sitemap stays well under 50MB at this scale).
        let l = locales_with(11);
        let g: Vec<Guide> = (0..4999).map(|i| guide(&format!("g{i}"))).collect();
        let count = sitemap_url_count(&g, &l);
        assert!(count > 50_000, "test fixture should exceed cap, got {count}");

        let tmp = std::env::temp_dir().join(format!(
            "fcdocs-sitemap-cap-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        let err = write_sitemap(&tmp, &g, &l).unwrap_err().to_string();
        assert!(
            err.contains("max URL count") && err.contains("50000"),
            "expected URL-cap error, got: {err}"
        );
        // Sitemap file must NOT exist — we bail before writing.
        assert!(
            !tmp.join("sitemap.xml").exists(),
            "sitemap.xml should not be written when URL cap trips"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    /// Sanity: a typical-shape build (15 guides, 28 locales) writes
    /// the sitemap and is well under both caps.
    #[test]
    fn typical_build_passes_both_caps() {
        let l = locales_with(28);
        let g: Vec<Guide> = (0..15).map(|i| guide(&format!("g{i}"))).collect();
        let tmp = std::env::temp_dir().join(format!(
            "fcdocs-sitemap-ok-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        write_sitemap(&tmp, &g, &l).expect("typical config must pass caps");
        assert!(tmp.join("sitemap.xml").exists());
        let _ = std::fs::remove_dir_all(&tmp);
    }
}

#[cfg(test)]
mod cache_key_tests {
    use super::*;
    use serde_json::json;

    /// Pre-fix: Node's `args.clickSelector ? [args.clickSelector] : ...`
    /// wrapped the singular form into a one-element array before
    /// keying the image cache. Rust read `clickSelectors` raw and
    /// silently omitted the field when only the singular was present,
    /// so every Node-built cache entry missed on the first Rust build.
    /// The fix mirrors the wrap. Verified end-to-end against the live
    /// Node cache via the same JSON shape.
    #[test]
    fn singular_click_selector_normalizes_to_one_element_array() {
        let cfg = json!({
            "url": "/x",
            "width": 1920,
            "clickSelector": ".foo",
            "selector": "body",
            "title": "T",
        });
        let key = build_cache_key_json(&cfg);
        // Field present + shaped as Node's wrapped array.
        assert!(
            key.contains(r#""clickSelectors":[".foo"]"#),
            "expected wrapped clickSelectors in {key}"
        );
        // And the raw singular field is NOT in the cache key (Node
        // never put it there).
        assert!(
            !key.contains(r#""clickSelector""#) || key.contains(r#""clickSelectors""#),
            "raw singular key must not appear in the cache key: {key}"
        );
    }

    /// Two configs that differ only in singular-vs-plural form must
    /// produce identical cache keys. This is the actual contract the
    /// bug violated.
    #[test]
    fn singular_and_wrapped_plural_produce_identical_keys() {
        let with_singular = build_cache_key_json(&json!({
            "url": "/x",
            "width": 1920,
            "clickSelector": ".foo",
            "selector": "body",
            "title": "T",
        }));
        let with_plural = build_cache_key_json(&json!({
            "url": "/x",
            "width": 1920,
            "clickSelectors": [".foo"],
            "selector": "body",
            "title": "T",
        }));
        assert_eq!(
            with_singular, with_plural,
            "singular and pre-wrapped plural must key identically"
        );
    }

    /// Match Node's ternary: when both forms are set, the singular
    /// wins (the plural branch is never consulted). Edge case, but
    /// the contract the bug fix mirrors verbatim.
    #[test]
    fn singular_wins_when_both_are_set() {
        let key = build_cache_key_json(&json!({
            "url": "/x",
            "clickSelector": ".singular",
            "clickSelectors": [".one", ".two"],
            "selector": "body",
            "title": "T",
        }));
        assert!(
            key.contains(r#""clickSelectors":[".singular"]"#),
            "singular must override the plural array entirely: {key}"
        );
        assert!(
            !key.contains(".one") && !key.contains(".two"),
            "the unused plural array must not leak into the cache key: {key}"
        );
    }

    /// Sanity: nothing set means the field is omitted (matches Node
    /// JSON.stringify dropping undefined).
    #[test]
    fn neither_form_omits_field_from_key() {
        let key = build_cache_key_json(&json!({
            "url": "/x",
            "selector": "body",
            "title": "T",
        }));
        assert!(
            !key.contains("clickSelectors"),
            "missing both forms should produce no clickSelectors field: {key}"
        );
    }

    /// Sanity: pre-wrapped plural with multiple entries passes
    /// through unchanged.
    #[test]
    fn plural_with_multiple_entries_passes_through() {
        let key = build_cache_key_json(&json!({
            "url": "/x",
            "clickSelectors": [".a", ".b"],
            "selector": "body",
            "title": "T",
        }));
        assert!(
            key.contains(r#""clickSelectors":[".a",".b"]"#),
            "multi-element plural must pass through verbatim: {key}"
        );
    }
}

#[cfg(test)]
mod proxy_select_tests {
    use super::*;

    /// Pin the post-fix invariant: sitegen reads
    /// `src/static/{js/proxy-select.js, css/proxy-select.css}` and
    /// populates `ScreenshotHost`. Pre-fix this was None/None and
    /// `screenshot::capture` was called with None/None too, so
    /// `addProxySelect=true` markers degraded silently. The test
    /// looks up the actual repo (walking up from CARGO_MANIFEST_DIR)
    /// so a future maintainer deleting the asset files surfaces here
    /// instead of at screenshot-capture time.
    #[test]
    fn live_repo_assets_load_into_screenshot_host() {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        // sitegen/Cargo.toml is at rust/sitegen/; repo root is two up.
        let repo = manifest.parent().unwrap().parent().unwrap();
        let host = load_screenshot_host(repo);
        let script = host.proxy_script.expect("proxy-select.js must load");
        let style = host.proxy_style.expect("proxy-select.css must load");
        // Sanity-check the content actually carries the surface the
        // injection relies on. If someone replaces proxy-select.js
        // with an empty stub, this fires before screenshots silently
        // break.
        assert!(
            script.contains("proxifySelect"),
            "proxy-select.js missing core function — expected proxifySelect symbol"
        );
        assert!(
            style.contains("cb-pvt-dropdown"),
            "proxy-select.css missing core class — expected .cb-pvt-dropdown"
        );
    }

    /// Default host has no proxy assets — that's the gate that lets
    /// `screenshot::capture` log a warn when a marker requests
    /// injection on a setup that never loaded them (tests, eg).
    #[test]
    fn default_host_has_no_proxy_assets() {
        let h = fcdocs_browser::ScreenshotHost::default();
        assert!(h.proxy_script.is_none());
        assert!(h.proxy_style.is_none());
    }

    /// Missing asset files don't panic — they degrade. (Sitegen
    /// keeps building; the marker just gets a closed `<select>` in
    /// the screenshot.)
    #[test]
    fn missing_assets_degrade_to_none() {
        let tmp = std::env::temp_dir().join(format!(
            "fcdocs-proxy-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        // No src/static/ underneath -> both reads fail.
        let host = load_screenshot_host(&tmp);
        assert!(host.proxy_script.is_none());
        assert!(host.proxy_style.is_none());
        let _ = std::fs::remove_dir_all(&tmp);
    }
}

#[cfg(test)]
mod filter_tests {
    use super::*;

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn locale_filter_basic() {
        assert_eq!(
            parse_locale_filter(args(&["--locale=fr_fr,de_de"]).into_iter()),
            Some(vec!["fr_fr".to_string(), "de_de".to_string()])
        );
        assert_eq!(
            parse_locale_filter(args(&["--locale", "ja_jp"]).into_iter()),
            Some(vec!["ja_jp".to_string()])
        );
        assert_eq!(parse_locale_filter(args(&[]).into_iter()), None);
    }

    #[test]
    fn guide_filter_basic() {
        assert_eq!(
            parse_guide_filter(args(&["--guide=acme,beta"]).into_iter()),
            Some(vec!["acme".to_string(), "beta".to_string()])
        );
        assert_eq!(
            parse_guide_filter(args(&["--guides", "acme"]).into_iter()),
            Some(vec!["acme".to_string()])
        );
        assert_eq!(parse_guide_filter(args(&[]).into_iter()), None);
    }

    /// Mixing both filters in one args vec must parse independently.
    /// Pre-incremental-rebuild this didn't matter because only --locale
    /// existed; once the watcher started passing both, we needed each
    /// parser to walk past the other's flag without consuming it.
    #[test]
    fn guide_and_locale_filters_coexist() {
        let argv = args(&["--guide=acme", "--locale=fr_fr,de_de"]);
        assert_eq!(
            parse_guide_filter(argv.iter().cloned()),
            Some(vec!["acme".to_string()])
        );
        assert_eq!(
            parse_locale_filter(argv.iter().cloned()),
            Some(vec!["fr_fr".to_string(), "de_de".to_string()])
        );
    }

    /// Invalid ids (path-traversal attempt) drop with a warning rather
    /// than widening the filter to "everything" by returning None.
    #[test]
    fn invalid_ids_drop_quietly() {
        assert_eq!(
            parse_guide_filter(args(&["--guide=../etc/passwd,good"]).into_iter()),
            Some(vec!["good".to_string()])
        );
        assert_eq!(
            parse_locale_filter(args(&["--locale=../../../etc"]).into_iter()),
            Some(vec![])
        );
    }
}

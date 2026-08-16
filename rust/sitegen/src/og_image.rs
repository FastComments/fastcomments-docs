//! Per-guide, per-locale social card images.
//!
//! Before this existed every one of the ~2500 docs pages pointed `og:image`
//! at a single site-wide `https://fastcomments.com/images/og-card.png`, and
//! at 1200x923 that image isn't even the right shape for a
//! `summary_large_image` card, so X center-cropped it. This module renders a
//! real 1200x630 card per (guide, locale) carrying the localized title.
//!
//! **Rendering via Chrome, not a Rust rasterizer.** The 23 locales include
//! ja/ko/zh/he, so laying out a title of unknown width means font fallback,
//! bidi, and line wrapping. Chrome does all three for free and is already a
//! required build dependency for `[app-screenshot-*]` markers. We use the
//! plain `screenshot::launch` rather than `BrowserPool`/`launch_logged_in`,
//! both of which are wired to the logged-in app host and would navigate
//! somewhere irrelevant.
//!
//! **Caching.** Filenames are content-addressed on
//! `md5(title | kicker | icon | locale | CARD_VERSION)`, so the file
//! existing *is* the freshness check, with no sidecar metadata. This deliberately
//! does not reuse `fcdocs_browser::ImageCache`, whose one-week expiry exists
//! because the remote app UI drifts; here it would re-render every card once
//! a week for nothing. Bump `CARD_VERSION` to push a redesign through.
//!
//! **Never fatal.** Chrome missing, launch failing, or a capture erroring all
//! degrade to the old static card. A social image is not worth failing a build
//! over.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use base64::Engine as _;
use fcdocs_shared::guides::{Guide, GuidesRoot};
use fcdocs_shared::locales::Locales;
use fcdocs_shared::templates::TemplateRegistry;
use fcdocs_shared::translations::Translations;
use serde_json::json;
use tracing::{info, warn};

use crate::build::load_meta_for_locale;

/// Card canvas. 1200x630 is the 1.91:1 that both `summary_large_image` and
/// Open Graph want, so nothing gets cropped.
pub const CARD_WIDTH: u32 = 1200;
pub const CARD_HEIGHT: u32 = 630;

/// Bump to invalidate every cached card (design change, template edit).
const CARD_VERSION: u32 = 1;

/// Cards live in a subdirectory so `build.sh`'s
/// `rm -f src/static/generated/*.*` (top-level files only) leaves them alone
/// and the cache actually survives between builds.
const CARD_SUBDIR: &str = "images/og";

const SITE_BASE: &str = "https://docs.fastcomments.com";

/// Pre-existing site-wide card, kept as the degradation path. Dimensions are
/// the real ones for that file; claiming 1200x630 for a 1200x923 image would
/// make the crawler's layout wrong.
const FALLBACK_URL: &str = "https://fastcomments.com/images/og-card.png";
const FALLBACK_WIDTH: u32 = 1200;
const FALLBACK_HEIGHT: u32 = 923;

/// How many cards go into one `set_content` document. Keeps any single
/// document from becoming absurdly tall (25 * 630 = ~15,750px) while still
/// amortizing navigation cost across the batch.
const BATCH_SIZE: usize = 25;

/// What a template needs to point at an image.
pub struct CardImage {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

impl CardImage {
    fn fallback() -> Self {
        Self {
            url: FALLBACK_URL.to_string(),
            width: FALLBACK_WIDTH,
            height: FALLBACK_HEIGHT,
        }
    }

    fn generated(file_name: &str) -> Self {
        Self {
            url: format!("{SITE_BASE}/{CARD_SUBDIR}/{file_name}"),
            width: CARD_WIDTH,
            height: CARD_HEIGHT,
        }
    }
}

/// guide id -> locale -> card filename. The index page uses [`INDEX_KEY`] as
/// its guide id, which can't collide with a real guide because `is_valid_id`
/// rejects the `#` character.
///
/// Nested rather than keyed on `(String, String)` so lookups borrow: a tuple
/// key can't be probed with `(&str, &str)`, which would mean two `to_string()`
/// allocations on every one of the ~2500 page renders.
pub type CardMap = HashMap<String, HashMap<String, String>>;

const INDEX_KEY: &str = "#index";

/// Look up a guide's card, falling back to the static site-wide one.
pub fn card_for(cards: &CardMap, guide_id: &str, locale: &str) -> CardImage {
    lookup(cards, guide_id, locale)
}

/// Look up the docs-home card for a locale.
pub fn index_card_for(cards: &CardMap, locale: &str) -> CardImage {
    lookup(cards, INDEX_KEY, locale)
}

fn lookup(cards: &CardMap, guide_id: &str, locale: &str) -> CardImage {
    cards
        .get(guide_id)
        .and_then(|by_locale| by_locale.get(locale))
        .map(|f| CardImage::generated(f))
        .unwrap_or_else(CardImage::fallback)
}

/// `og:locale` wants `language_TERRITORY` (`fr_FR`), not the BCP-47 tag we
/// keep in locales.json for `hreflang` / `<html lang>`.
///
/// The conversion is not a plain `-` -> `_` swap. Two locales carry a bare
/// language with no region (`en`, `he`) and one carries a script subtag
/// (`sr-Latn-RS`), and consumers ignore anything that isn't the two-part
/// form. Returns `None` when we can't produce a valid pair, so the template
/// omits the tag rather than emitting something wrong.
pub fn og_locale(locales: &Locales, locale: &str) -> Option<String> {
    let hreflang = &locales.locales.get(locale)?.hreflang;
    let mut parts = hreflang.split('-');
    let lang = parts.next()?.to_ascii_lowercase();
    if lang.is_empty() {
        return None;
    }
    // Take the trailing region and drop any script subtag in between:
    // `sr-Latn-RS` -> `sr_RS`.
    let region = hreflang
        .rsplit('-')
        .next()
        .filter(|r| r.len() == 2 && r.chars().all(|c| c.is_ascii_alphabetic()) && *r != hreflang)
        .map(|r| r.to_ascii_uppercase())
        .or_else(|| default_region(&lang))?;
    Some(format!("{lang}_{region}"))
}

/// Regions for the locales.json entries that carry no region of their own.
/// Anything not listed yields no `og:locale` at all, because a wrong region is
/// worse than an absent tag.
fn default_region(lang: &str) -> Option<String> {
    match lang {
        "en" => Some("US".to_string()),
        "he" => Some("IL".to_string()),
        _ => None,
    }
}

/// One card to draw.
struct CardSpec {
    guide_id: String,
    locale: String,
    title: String,
    kicker: String,
    /// Basename under `src/static/images/guide-icons/`, if the guide has one.
    icon: Option<String>,
    lang: String,
    dir: &'static str,
    file_name: String,
}

impl CardSpec {
    fn new(
        guide_id: &str,
        locale: &str,
        title: String,
        kicker: String,
        icon: Option<String>,
        lang: String,
    ) -> Self {
        let dir = if is_rtl(locale) { "rtl" } else { "ltr" };
        let file_name = card_file_name(
            &title,
            &kicker,
            icon.as_deref().unwrap_or(""),
            locale,
        );
        Self {
            guide_id: guide_id.to_string(),
            locale: locale.to_string(),
            title,
            kicker,
            icon,
            lang,
            dir,
            file_name,
        }
    }
}

/// Hebrew is the only RTL locale in locales.json today, but matching on a
/// prefix keeps ar/fa working if they're ever added.
fn is_rtl(locale: &str) -> bool {
    matches!(locale, "he") || locale.starts_with("he_") || locale.starts_with("ar") || locale.starts_with("fa")
}

/// Content-addressed filename. Everything that changes a pixel is in the
/// hash, so an existing file is by definition current.
///
/// Fields are fed to the hasher separately and the hex is written into one
/// pre-sized buffer. The obvious `format!` version allocates a joined string
/// plus one `String` per digest byte, which is 17 allocations on a call made
/// once per (guide, locale).
fn card_file_name(title: &str, kicker: &str, icon: &str, locale: &str) -> String {
    use md5::{Digest, Md5};
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut hasher = Md5::new();
    hasher.update(CARD_VERSION.to_le_bytes());
    // The separator keeps field boundaries unambiguous, so a title ending in
    // the next field's opening text can't collide with a different split.
    for field in [locale, title, kicker, icon] {
        hasher.update(b"|");
        hasher.update(field.as_bytes());
    }
    let mut name = String::with_capacity(36);
    for b in hasher.finalize() {
        name.push(HEX[(b >> 4) as usize] as char);
        name.push(HEX[(b & 0x0f) as usize] as char);
    }
    name.push_str(".png");
    name
}

/// Render every card needed by this build, returning the map the page and
/// index templates read.
///
/// `prune` should be false for partial (`--guide` / `--locale`) builds, because the
/// spec list only covers what's being rebuilt, so pruning against it would
/// delete cards belonging to pages this run isn't touching.
#[allow(clippy::too_many_arguments)]
pub async fn render_all(
    repo: &Path,
    guides: &[Guide],
    root: &GuidesRoot,
    locales: &Locales,
    locale_keys: &[String],
    translations: &Translations,
    templates: &TemplateRegistry,
    static_generated_dir: &Path,
    prune: bool,
) -> CardMap {
    if std::env::var("SITEGEN_OG_CARDS").as_deref() == Ok("0") {
        info!("SITEGEN_OG_CARDS=0, using the static site-wide og:image");
        return CardMap::new();
    }

    let specs = collect_specs(guides, root, locales, locale_keys, translations);
    if specs.is_empty() {
        return CardMap::new();
    }

    let out_dir = static_generated_dir.join(CARD_SUBDIR);
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        warn!(error = %e, dir = ?out_dir, "cannot create og card dir; using static og:image");
        return CardMap::new();
    }

    // Everything we're about to claim in the map, whether or not it needs a
    // render this run.
    let mut map = CardMap::new();
    for s in &specs {
        map.entry(s.guide_id.clone())
            .or_default()
            .insert(s.locale.clone(), s.file_name.clone());
    }

    // One read_dir instead of a stat per spec. On a warm build every card is
    // present, so the naive version pays ~2500 syscalls to learn there is
    // nothing to do.
    let on_disk = existing_card_names(&out_dir);
    // The hash keys on title/kicker/icon/locale but not guide id, so two
    // guides sharing a title in the same locale legitimately share one card.
    // Draw it once.
    let mut queued = std::collections::HashSet::new();
    let stale: Vec<&CardSpec> = specs
        .iter()
        .filter(|s| !on_disk.contains(s.file_name.as_str()))
        .filter(|s| queued.insert(s.file_name.as_str()))
        .collect();

    if stale.is_empty() {
        info!(cards = specs.len(), "og cards up to date");
    } else {
        info!(
            total = specs.len(),
            render = stale.len(),
            "rendering og cards"
        );
        if let Err(e) = render_stale(repo, templates, &stale, &out_dir).await {
            // Partial output is fine: the map still points at whatever landed,
            // and anything missing 404s -> crawlers fall back to no image.
            // Rather than ship that, drop back to the static card wholesale.
            warn!(error = %format!("{e:#}"), "og card rendering failed; using static og:image");
            return CardMap::new();
        }
    }

    if prune {
        prune_orphans(&out_dir, &specs);
    }

    map
}

/// Build the spec list. Titles come from `load_meta_for_locale`, which
/// `GuidesRoot` already memoizes, so this is cheap even at 108 x 23.
fn collect_specs(
    guides: &[Guide],
    root: &GuidesRoot,
    locales: &Locales,
    locale_keys: &[String],
    translations: &Translations,
) -> Vec<CardSpec> {
    let mut specs = Vec::with_capacity(guides.len() * locale_keys.len() + locale_keys.len());
    for locale in locale_keys {
        let t = translations.for_locale(locale);
        // "Documentation", already translated for every locale.
        let kicker = t
            .map
            .get("DOCUMENTATION_TITLE")
            .cloned()
            .unwrap_or_else(|| "Documentation".to_string());
        let lang = locales
            .locales
            .get(locale)
            .map(|l| l.hreflang.clone())
            .unwrap_or_else(|| locale.to_string());

        for guide in guides {
            let meta = match load_meta_for_locale(root, &guide.id, locale) {
                Ok(m) => m,
                Err(e) => {
                    warn!(guide = %guide.id, locale, error = %e, "skip og card (meta)");
                    continue;
                }
            };
            let title = meta
                .page_header
                .clone()
                .unwrap_or_else(|| meta.name.clone().unwrap_or_default());
            if title.trim().is_empty() {
                continue;
            }
            specs.push(CardSpec::new(
                &guide.id,
                locale,
                title,
                kicker.clone(),
                meta.icon.clone(),
                lang.clone(),
            ));
        }

        // Docs home. Its <h1> is DOCUMENTATION_TITLE, so the card would read
        // "Documentation / Documentation" if it reused the kicker, so use the
        // product name as the title instead.
        specs.push(CardSpec::new(
            INDEX_KEY,
            locale,
            format!("FastComments {kicker}"),
            kicker.clone(),
            None,
            lang.clone(),
        ));
    }

    specs
}

async fn render_stale(
    repo: &Path,
    templates: &TemplateRegistry,
    stale: &[&CardSpec],
    out_dir: &Path,
) -> Result<()> {
    let font = font_data_uri(repo);
    let icons_dir = repo.join("src/static/images/guide-icons");

    // Chrome needs a viewport at least as wide as a card, otherwise the flex
    // layout reflows and the capture clip lands on a differently-wrapped
    // title than the one we designed.
    let (mut browser, handler_task) =
        fcdocs_browser::screenshot::launch(CARD_WIDTH + 40, CARD_HEIGHT + 40)
            .await
            .context("launch chrome for og cards")?;
    let page = browser
        .new_page("about:blank")
        .await
        .context("open og card page")?;

    let mut rendered = 0usize;
    let result: Result<()> = async {
        for batch in stale.chunks(BATCH_SIZE) {
            // Element ids are positional rather than derived from the
            // filename so that `#c<N>` is always a valid, unambiguous
            // selector regardless of what the hash looks like.
            let cards: Vec<serde_json::Value> = batch
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    json!({
                        "elementId": format!("c{i}"),
                        "title": s.title,
                        "kicker": s.kicker,
                        "lang": s.lang,
                        "dir": s.dir,
                        "iconDataUri": s.icon.as_deref().and_then(|i| icon_data_uri(&icons_dir, i)),
                    })
                })
                .collect();

            let html = templates.render(
                "og-card",
                &json!({ "fontDataUri": font, "cards": cards }),
            )?;
            page.set_content(&html).await.context("set card content")?;

            for (i, spec) in batch.iter().enumerate() {
                let png =
                    fcdocs_browser::screenshot::capture_element_png(&page, &format!("#c{i}"))
                        .await
                        .with_context(|| {
                            format!("capture card for {}/{}", spec.guide_id, spec.locale)
                        })?;
                let path = out_dir.join(&spec.file_name);
                std::fs::write(&path, &png).with_context(|| format!("write {path:?}"))?;
                rendered += 1;
            }
        }
        Ok(())
    }
    .await;

    let _ = browser.close().await;
    handler_task.abort();
    result?;
    info!(rendered, "og cards rendered");
    Ok(())
}

/// Read the font once per build and hand Chrome a data URI. `set_content`
/// leaves the document on `about:blank`, so `url('/css/...')` would resolve
/// to nothing and the card would silently render in a fallback face.
fn font_data_uri(repo: &Path) -> String {
    let path = repo.join("src/static/css/RedHatDisplay-VariableFont_wght.ttf");
    match std::fs::read(&path) {
        Ok(bytes) => format!(
            "data:font/ttf;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(bytes)
        ),
        Err(e) => {
            // Not fatal: the CSS font stack falls through to system faces.
            warn!(error = %e, path = ?path, "og card webfont missing; using system fonts");
            String::new()
        }
    }
}

fn icon_data_uri(icons_dir: &Path, icon: &str) -> Option<String> {
    // meta.json is content, so treat `icon` as untrusted: refuse anything
    // that could escape the icons directory.
    if icon.contains('/') || icon.contains('\\') || icon.contains("..") {
        warn!(icon, "ignoring guide icon with a path separator");
        return None;
    }
    let path = icons_dir.join(icon);
    let bytes = std::fs::read(&path).ok()?;
    let mime = match path.extension().and_then(|e| e.to_str()) {
        Some("svg") => "image/svg+xml",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        _ => "image/png",
    };
    Some(format!(
        "data:{mime};base64,{}",
        base64::engine::general_purpose::STANDARD.encode(bytes)
    ))
}

/// Card filenames already on disk. A missing or unreadable directory reads
/// as "nothing cached", which just means everything re-renders.
fn existing_card_names(out_dir: &Path) -> std::collections::HashSet<String> {
    let Ok(entries) = std::fs::read_dir(out_dir) else {
        return std::collections::HashSet::new();
    };
    entries
        .flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .collect()
}

/// Content-addressed names accumulate as titles change. Drop anything no
/// longer referenced so the directory doesn't grow without bound.
fn prune_orphans(out_dir: &Path, specs: &[CardSpec]) {
    let keep: std::collections::HashSet<&str> =
        specs.iter().map(|s| s.file_name.as_str()).collect();
    let Ok(entries) = std::fs::read_dir(out_dir) else {
        return;
    };
    let mut removed = 0usize;
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("png") {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !keep.contains(name) && std::fs::remove_file(&path).is_ok() {
            removed += 1;
        }
    }
    if removed > 0 {
        info!(removed, "pruned stale og cards");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_name_is_stable_and_title_sensitive() {
        let a = card_file_name("Badges", "Documentation", "badge.svg", "en");
        let b = card_file_name("Badges", "Documentation", "badge.svg", "en");
        let c = card_file_name("Badges!", "Documentation", "badge.svg", "en");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a.ends_with(".png"));
        assert_eq!(a.len(), 36);
    }

    #[test]
    fn file_name_varies_by_locale_and_icon() {
        let en = card_file_name("Badges", "Documentation", "badge.svg", "en");
        let fr = card_file_name("Badges", "Documentation", "badge.svg", "fr_fr");
        let no_icon = card_file_name("Badges", "Documentation", "", "en");
        assert_ne!(en, fr);
        assert_ne!(en, no_icon);
    }

    #[test]
    fn missing_card_falls_back_to_the_static_image() {
        let cards = CardMap::new();
        let img = card_for(&cards, "badges", "en");
        assert_eq!(img.url, FALLBACK_URL);
        assert_eq!(img.height, FALLBACK_HEIGHT);
    }

    #[test]
    fn present_card_resolves_to_an_absolute_docs_url() {
        let mut cards = CardMap::new();
        cards
            .entry("badges".to_string())
            .or_default()
            .insert("en".to_string(), "abc.png".to_string());
        let img = card_for(&cards, "badges", "en");
        assert_eq!(img.url, "https://docs.fastcomments.com/images/og/abc.png");
        assert_eq!((img.width, img.height), (CARD_WIDTH, CARD_HEIGHT));
        // A guide present in the map but not in this locale still falls back.
        assert_eq!(card_for(&cards, "badges", "fr_fr").url, FALLBACK_URL);
    }

    #[test]
    fn index_key_cannot_collide_with_a_guide_id() {
        // is_valid_id rejects '#', so no guide directory can ever be named
        // INDEX_KEY.
        assert!(!fcdocs_shared::guides::is_valid_id(INDEX_KEY));
    }

    fn locales_fixture() -> Locales {
        use fcdocs_shared::locales::Locale;
        use indexmap::IndexMap;
        let mut map = IndexMap::new();
        for (code, hreflang) in [
            ("en", "en"),
            ("he", "he"),
            ("fr_fr", "fr-FR"),
            ("pt_br", "pt-BR"),
            ("sr_latn_rs", "sr-Latn-RS"),
            ("xx_yy", "xx"),
        ] {
            map.insert(
                code.to_string(),
                Locale {
                    name: code.to_string(),
                    native_name: code.to_string(),
                    hreflang: hreflang.to_string(),
                    flag: None,
                },
            );
        }
        Locales {
            default_locale: "en".to_string(),
            locales: map,
        }
    }

    /// og:locale is `language_TERRITORY`, which is NOT a `-` -> `_` swap of
    /// the hreflang tag: two locales carry no region and one carries a
    /// script subtag.
    #[test]
    fn og_locale_normalizes_to_language_territory() {
        let l = locales_fixture();
        assert_eq!(og_locale(&l, "fr_fr").as_deref(), Some("fr_FR"));
        assert_eq!(og_locale(&l, "pt_br").as_deref(), Some("pt_BR"));
        // Script subtag dropped.
        assert_eq!(og_locale(&l, "sr_latn_rs").as_deref(), Some("sr_RS"));
        // Region-less locales get an explicit default.
        assert_eq!(og_locale(&l, "en").as_deref(), Some("en_US"));
        assert_eq!(og_locale(&l, "he").as_deref(), Some("he_IL"));
    }

    /// An unknown region-less language emits nothing rather than a guess:
    /// the template guards on `{{#if ogLocale}}`.
    #[test]
    fn og_locale_omits_rather_than_guesses() {
        let l = locales_fixture();
        assert_eq!(og_locale(&l, "xx_yy"), None);
        assert_eq!(og_locale(&l, "not_a_locale"), None);
    }

    #[test]
    fn hebrew_renders_rtl() {
        assert!(is_rtl("he"));
        assert!(!is_rtl("en"));
        assert!(!is_rtl("hr_hr"));
    }

    #[test]
    fn icon_path_traversal_is_refused() {
        let dir = Path::new("/nonexistent");
        assert!(icon_data_uri(dir, "../../../etc/passwd").is_none());
        assert!(icon_data_uri(dir, "sub/icon.png").is_none());
    }
}

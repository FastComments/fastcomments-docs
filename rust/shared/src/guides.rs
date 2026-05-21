use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::{Context, Result};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Locale + guide-id allowlist. Both are filesystem path components and
/// flow into `Path::join` in many places, so we reject anything outside
/// `[A-Za-z0-9_-]+`. This is more restrictive than the actual
/// filesystem accepts, intentionally — it stops `..`, slashes, NULs,
/// and weird unicode that could confuse downstream code.
static ID_OK: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Za-z0-9_-]+$").expect("regex"));

pub fn is_valid_id(s: &str) -> bool {
    !s.is_empty() && s.len() <= 128 && ID_OK.is_match(s)
}

fn check_id(field: &str, value: &str) -> Result<()> {
    if !is_valid_id(value) {
        anyhow::bail!(
            "invalid {field}: {value:?} (allowed: ^[A-Za-z0-9_-]+$, max 128 chars)"
        );
    }
    Ok(())
}

/// Mirrors the meta.json `itemsOrdered[]` entry shape used by `src/guides.js`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaItem {
    pub name: String,
    pub file: String,
    #[serde(default, rename = "subCat")]
    pub sub_cat: Option<String>,
    #[serde(default, rename = "sidebarItemClasses")]
    pub sidebar_item_classes: Option<String>,
    /// Catch-all for fields like `icon`, `type`, `isFunctional`, etc. we don't
    /// type-check from Rust.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Top-level guide meta.json shape.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GuideMeta {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "pageHeader")]
    pub page_header: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, rename = "itemsOrdered")]
    pub items_ordered: Vec<MetaItem>,
    /// Catch-all for fields like `description`, `faq`, etc. that we don't
    /// type-check from Rust.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// A discovered guide directory, plus the locale-resolved meta for it.
#[derive(Debug, Clone)]
pub struct Guide {
    pub id: String,
    pub meta: GuideMeta,
    pub items_dir: PathBuf,
}

/// Output of running the content pipeline against a single meta item.
#[derive(Debug, Clone)]
pub struct GuideItem {
    pub id: String,
    pub title: String,
    pub full_url: String,
    /// HTML, already passed through markers, markdown, snippet processor.
    pub html: String,
    pub is_fallback: bool,
}

pub struct GuidesRoot {
    pub guides_dir: PathBuf,
    pub default_locale: String,
    /// Per-(guide_id, meta_path) parsed-meta cache. Avoids re-reading
    /// + re-parsing the same meta.json once per locale (28+ times per
    /// full build).
    meta_cache: DashMap<PathBuf, GuideMeta>,
    /// One-shot warning so we don't spam logs when an upstream caller
    /// repeatedly hands us a bad id.
    warned_ids: Mutex<std::collections::HashSet<String>>,
}

impl GuidesRoot {
    pub fn new(guides_dir: impl Into<PathBuf>, default_locale: impl Into<String>) -> Self {
        Self {
            guides_dir: guides_dir.into(),
            default_locale: default_locale.into(),
            meta_cache: DashMap::new(),
            warned_ids: Mutex::new(std::collections::HashSet::new()),
        }
    }

    fn validate_id(&self, field: &str, value: &str) -> bool {
        if is_valid_id(value) {
            return true;
        }
        // Warn once per offending value so debug logs stay readable.
        let key = format!("{field}={value}");
        let mut seen = self.warned_ids.lock().unwrap();
        if seen.insert(key) {
            tracing::warn!(
                field,
                value,
                "rejecting invalid id (must match ^[A-Za-z0-9_-]+$)"
            );
        }
        false
    }

    fn load_meta(&self, meta_path: &Path) -> Result<GuideMeta> {
        if let Some(hit) = self.meta_cache.get(meta_path) {
            return Ok(hit.clone());
        }
        let bytes = std::fs::read(meta_path)
            .with_context(|| format!("read meta json {meta_path:?}"))?;
        let meta: GuideMeta = serde_json::from_slice(&bytes)
            .with_context(|| format!("parse meta json {meta_path:?}"))?;
        self.meta_cache.insert(meta_path.to_path_buf(), meta.clone());
        Ok(meta)
    }

    /// Walk every guide directory and load its locale-resolved meta.
    ///
    /// Mirrors `getGuides(locale)` in `src/guides.js:302-350`. Skips guides
    /// missing `meta.json` (warn + continue) and guides with no items/url.
    pub fn walk(&self, locale: &str) -> Result<Vec<Guide>> {
        let mut out = Vec::new();
        let entries = std::fs::read_dir(&self.guides_dir)
            .with_context(|| format!("read guides dir {:?}", self.guides_dir))?;

        for entry in entries {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if !file_type.is_dir() {
                continue;
            }
            let id = entry.file_name().to_string_lossy().into_owned();
            if id == "guide-order.json" {
                continue;
            }
            let guide_dir = entry.path();
            let default_meta_path = guide_dir.join("meta.json");
            if !default_meta_path.exists() {
                tracing::warn!(guide_id = %id, "skipping guide without meta.json");
                continue;
            }

            // Prefer locale-translated meta if present.
            let translated = guide_dir
                .join("meta_translated")
                .join(format!("meta_{locale}.json"));
            let meta_path = if translated.exists() {
                translated
            } else {
                default_meta_path
            };

            let meta = self.load_meta(&meta_path)?;

            let has_items = !meta.items_ordered.is_empty() || meta.url.is_some();
            if !has_items {
                continue;
            }

            let items_dir = guide_dir.join("items");
            out.push(Guide { id, meta, items_dir });
        }
        out.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(out)
    }

    /// Build the URL slug for a guide page. Mirrors `createGuideLink` in
    /// `src/guides.js:290-295`. `id` and `locale` are filename
    /// components; if either fails validation we return a sentinel
    /// "invalid-id" link so we never write `guide-../-foo.html`.
    pub fn guide_link(&self, id: &str, locale: &str) -> String {
        if !self.validate_id("guide_id", id) || !self.validate_id("locale", locale) {
            return "guide-invalid-id.html".to_string();
        }
        if locale == self.default_locale {
            format!("guide-{id}.html")
        } else {
            format!("guide-{id}-{locale}.html")
        }
    }

    /// Resolve the markdown file path for a meta item, applying the 3-tier
    /// fallback used by `buildGuideItemForMeta` in `src/guides.js:108-132`:
    /// locale-specific -> default locale -> non-localized root.
    ///
    /// `guide_id`, `locale`, and `file` flow into Path::join. We
    /// validate the first two against the id allowlist; `file` is left
    /// to per-pipeline checks (it comes from meta.json + may include
    /// `.md` suffix).
    pub fn resolve_item_path(&self, guide_id: &str, file: &str, locale: &str) -> (PathBuf, bool) {
        if !self.validate_id("guide_id", guide_id) || !self.validate_id("locale", locale) {
            return (self.guides_dir.join("_invalid"), false);
        }
        let base = self.guides_dir.join(guide_id).join("items");

        let localized = base.join(locale).join(file);
        if localized.exists() {
            return (localized, false);
        }

        let default = base.join(&self.default_locale).join(file);
        if default.exists() {
            let is_fallback = locale != self.default_locale;
            return (default, is_fallback);
        }

        let flat = base.join(file);
        (flat, false)
    }

    /// Check whether a locale has any translated content for a guide.
    /// Mirrors `hasLocaleContent` check in `src/build-search-index-worker.js:88-93`.
    pub fn has_locale_content(&self, guide_id: &str, locale: &str) -> bool {
        if !self.validate_id("guide_id", guide_id) || !self.validate_id("locale", locale) {
            return false;
        }
        let dir = self.guides_dir.join(guide_id).join("items").join(locale);
        let Ok(mut rd) = std::fs::read_dir(&dir) else {
            return false;
        };
        rd.next().is_some()
    }

    pub fn intro_path(&self, guide_id: &str, locale: &str) -> Option<PathBuf> {
        if !self.validate_id("guide_id", guide_id) || !self.validate_id("locale", locale) {
            return None;
        }
        let candidates = [
            self.guides_dir
                .join(guide_id)
                .join("items")
                .join(locale)
                .join("intro.md"),
            self.guides_dir
                .join(guide_id)
                .join("items")
                .join(&self.default_locale)
                .join("intro.md"),
            self.guides_dir.join(guide_id).join("intro.md"),
        ];
        candidates.into_iter().find(|p| p.exists())
    }
}

pub fn read_meta(path: &Path) -> Result<GuideMeta> {
    let bytes = std::fs::read(path).with_context(|| format!("read meta json {path:?}"))?;
    Ok(serde_json::from_slice(&bytes)?)
}

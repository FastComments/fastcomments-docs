use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

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
}

impl GuidesRoot {
    pub fn new(guides_dir: impl Into<PathBuf>, default_locale: impl Into<String>) -> Self {
        Self {
            guides_dir: guides_dir.into(),
            default_locale: default_locale.into(),
        }
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

            let meta_bytes = std::fs::read(&meta_path)
                .with_context(|| format!("read meta json {meta_path:?}"))?;
            let meta: GuideMeta = serde_json::from_slice(&meta_bytes)
                .with_context(|| format!("parse meta json {meta_path:?}"))?;

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
    /// `src/guides.js:290-295`.
    pub fn guide_link(&self, id: &str, locale: &str) -> String {
        if locale == self.default_locale {
            format!("guide-{id}.html")
        } else {
            format!("guide-{id}-{locale}.html")
        }
    }

    /// Resolve the markdown file path for a meta item, applying the 3-tier
    /// fallback used by `buildGuideItemForMeta` in `src/guides.js:108-132`:
    /// locale-specific -> default locale -> non-localized root.
    pub fn resolve_item_path(&self, guide_id: &str, file: &str, locale: &str) -> (PathBuf, bool) {
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
        let dir = self.guides_dir.join(guide_id).join("items").join(locale);
        let Ok(mut rd) = std::fs::read_dir(&dir) else {
            return false;
        };
        rd.next().is_some()
    }

    pub fn intro_path(&self, guide_id: &str, locale: &str) -> Option<PathBuf> {
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

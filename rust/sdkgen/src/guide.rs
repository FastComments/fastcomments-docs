//! End-to-end SDK guide generation: checkout, run generators, write
//! per-section markdown + meta.json. Mirrors
//! `src/sdk-guide-generator.js:60-244`.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde_json::json;
use tracing::{info, warn};

use crate::checkout::{Checkout, CheckoutManager};
use crate::config::{self, SdkConfig};
use crate::generators::ai::AiGenerator;
use crate::generators::base::{DocGenerator, DocSection, GeneratorCtx};
use crate::generators::openapi::OpenApiGenerator;
use crate::generators::readme::{self, ReadmeGenerator};

pub async fn generate_all(sdk_filter: Option<&str>) -> Result<()> {
    let repo_root = repo_root()?;
    let cfg_path = config::default_config_path(&repo_root);
    let cfg = config::load(&cfg_path)?;
    let checkout_dir = repo_root.join(
        cfg.checkout_directory
            .clone()
            .unwrap_or_else(|| "src/content/sdks-checkout".to_string()),
    );
    let guides_dir = repo_root.join(
        cfg.guides_directory
            .clone()
            .unwrap_or_else(|| "src/content/guides".to_string()),
    );

    let checkout_mgr = CheckoutManager::new(checkout_dir)?;

    let mut sdks: Vec<SdkConfig> = cfg.sdks;
    if let Some(filter) = sdk_filter {
        sdks.retain(|s| s.id == filter);
        if sdks.is_empty() {
            anyhow::bail!("no SDK matched --sdk {filter}");
        }
        info!(filter, "restricted to one SDK");
    }

    let checkouts = checkout_mgr.checkout_all(&sdks);

    for co in &checkouts {
        if let Err(e) = generate_one(co, &guides_dir).await {
            warn!(sdk = %co.sdk.id, error = %format!("{e:#}"), "skipping SDK");
        }
    }
    info!(count = checkouts.len(), "sdkgen done");
    Ok(())
}

async fn generate_one(checkout: &Checkout, guides_dir: &Path) -> Result<()> {
    let guide_dir = guides_dir.join(&checkout.sdk.id);
    let items_dir = guide_dir.join("items");
    let en_dir = items_dir.join("en");
    std::fs::create_dir_all(&en_dir).with_context(|| format!("create {en_dir:?}"))?;

    let ctx = GeneratorCtx {
        sdk: checkout.sdk.clone(),
        repo_path: checkout.path.clone(),
    };

    let mut merged_intro: Option<String> = None;
    let mut merged_conclusion: Option<String> = None;
    let mut all_sections: Vec<DocSection> = Vec::new();

    for kind in checkout.sdk.generators() {
        let generator: Box<dyn DocGenerator> = match kind.as_str() {
            "readme" => Box::new(ReadmeGenerator),
            "openapi" => Box::new(OpenApiGenerator),
            "typescript-ai" => Box::new(AiGenerator { language: "typescript" }),
            "rust-ai" => Box::new(AiGenerator { language: "rust" }),
            "cpp-ai" => Box::new(AiGenerator { language: "cpp" }),
            "nim-ai" => Box::new(AiGenerator { language: "nim" }),
            other => {
                warn!(generator = %other, "unknown generator type — skipping");
                continue;
            }
        };
        match generator.generate(&ctx).await {
            Ok(docs) => {
                if merged_intro.is_none() {
                    merged_intro = docs.intro;
                }
                if merged_conclusion.is_none() {
                    merged_conclusion = docs.conclusion;
                }
                all_sections.extend(docs.sections);
            }
            Err(e) => warn!(generator = %kind, error = %e, "generator failed"),
        }
    }

    // Write intro / conclusion.
    if let Some(intro) = &merged_intro {
        std::fs::write(en_dir.join("intro.md"), intro)?;
    }
    if let Some(conclusion) = &merged_conclusion {
        std::fs::write(en_dir.join("conclusion.md"), conclusion)?;
    }

    // Write each section.
    for section in &mut all_sections {
        let mut filename = section
            .file
            .clone()
            .unwrap_or_else(|| format!("{}.md", readme::sanitize_filename(&section.name)));
        // Strip legacy `generated/` prefix.
        if let Some(rest) = filename.strip_prefix("generated/") {
            filename = rest.to_string();
        }
        let file_path = en_dir.join(&filename);
        // Escape Handlebars-like `{{` so the templating step doesn't try
        // to interpret e.g. Blade syntax. Mirrors
        // src/sdk-guide-generator.js:229.
        let escaped = section.content.replace("{{", "\\{{");
        std::fs::write(&file_path, escaped)
            .with_context(|| format!("write {file_path:?}"))?;
        section.file = Some(filename);
    }

    // meta.json.
    let meta = build_meta(&checkout.sdk, &all_sections);
    let meta_path = guide_dir.join("meta.json");
    std::fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)
        .with_context(|| format!("write {meta_path:?}"))?;

    info!(sdk = %checkout.sdk.id, sections = all_sections.len(), "generated");
    Ok(())
}

fn build_meta(sdk: &SdkConfig, sections: &[DocSection]) -> serde_json::Value {
    // Match the priority + ordering logic in
    // src/sdk-guide-generator.js:91-138.
    let category_priority = |c: &str| match c {
        "Getting Started" => 1,
        "Documentation" => 2,
        "Usage" => 3,
        "API Reference" => 4,
        _ => 999,
    };

    let readme: Vec<&DocSection> = sections
        .iter()
        .filter(|s| s.type_.as_deref().unwrap_or("readme") == "readme")
        .collect();
    let mut api: Vec<&DocSection> = sections
        .iter()
        .filter(|s| s.type_.as_deref() == Some("api"))
        .collect();
    api.sort_by(|a, b| {
        let ca = a.sub_cat.as_deref().unwrap_or("Documentation");
        let cb = b.sub_cat.as_deref().unwrap_or("Documentation");
        let pa = category_priority(ca);
        let pb = category_priority(cb);
        pa.cmp(&pb).then(ca.cmp(cb)).then(a.name.cmp(&b.name))
    });

    let mut items: Vec<serde_json::Value> = Vec::with_capacity(sections.len());
    for s in readme.iter().chain(api.iter()) {
        let file = s
            .file
            .clone()
            .unwrap_or_else(|| format!("{}.md", readme::sanitize_filename(&s.name)));
        let mut obj = serde_json::Map::new();
        obj.insert("name".into(), json!(s.name));
        obj.insert("file".into(), json!(file));
        obj.insert(
            "subCat".into(),
            json!(s.sub_cat.clone().unwrap_or_else(|| "Documentation".to_string())),
        );
        obj.insert(
            "type".into(),
            json!(s.type_.clone().unwrap_or_else(|| "readme".to_string())),
        );
        if let Some(cls) = &s.sidebar_item_classes {
            obj.insert("sidebarItemClasses".into(), json!(cls));
        }
        items.push(serde_json::Value::Object(obj));
    }

    json!({
        "name": sdk.name,
        "pageHeader": sdk.page_header.clone().unwrap_or_else(|| sdk.name.clone()),
        "icon": sdk.icon,
        "itemsOrdered": items,
    })
}

fn repo_root() -> Result<PathBuf> {
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

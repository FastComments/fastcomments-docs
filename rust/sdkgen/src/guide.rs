//! End-to-end SDK guide generation: checkout, run generators, write
//! per-section markdown + meta.json. Mirrors
//! `src/sdk-guide-generator.js:60-244`.

use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use futures::stream::{FuturesUnordered, StreamExt};
use serde_json::json;
use tracing::{info, warn};

use crate::checkout::{Checkout, CheckoutManager};
use crate::config::{self, SdkConfig};
use crate::generators::ai::{CppAiGenerator, NimAiGenerator, RustAiGenerator, TypescriptAiGenerator};
use crate::generators::base::{DocGenerator, DocSection, GeneratorCtx};
use crate::generators::openapi::OpenApiGenerator;
use crate::generators::readme::{self, ReadmeGenerator};

/// Concurrency cap for SDK-level fan-out. SDK generation is mostly
/// network/IO-bound (git checkouts, OpenAI calls), so the default
/// runs all SDKs in parallel up to a cap. Override with
/// `SDKGEN_PARALLEL`.
const DEFAULT_SDK_PARALLELISM: usize = 8;

pub async fn generate_all(sdk_filter: Option<&str>) -> Result<()> {
    let repo_root = repo_root()?;
    let cfg_path = config::default_config_path(&repo_root);
    let cfg = config::load(&cfg_path)?;
    let checkout_dir = repo_root.join(
        cfg.checkout_directory
            .clone()
            .unwrap_or_else(|| "src/content/sdks-checkout".to_string()),
    );
    let guides_dir = Arc::new(repo_root.join(
        cfg.guides_directory
            .clone()
            .unwrap_or_else(|| "src/content/guides".to_string()),
    ));

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

    let parallelism: usize = std::env::var("SDKGEN_PARALLEL")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|n: &usize| *n > 0)
        .unwrap_or(DEFAULT_SDK_PARALLELISM);
    info!(parallelism, "sdkgen parallelism");

    let mut tasks = FuturesUnordered::new();
    let mut iter = checkouts.into_iter();
    while tasks.len() < parallelism {
        let Some(co) = iter.next() else { break };
        tasks.push(spawn_sdk(co, guides_dir.clone()));
    }
    let mut total = 0usize;
    // Aggregate outcomes across all SDKs. Per Node's
    // sdk-guide-generator.js:268-309 contract: run every SDK to
    // completion, collect every validation defect, print a
    // BUILD FAILED summary at the end, then exit non-zero.
    let mut all_outcomes: Vec<SdkOutcome> = Vec::new();
    while let Some(joined) = tasks.next().await {
        total += 1;
        match joined {
            Ok(Ok(outcome)) => all_outcomes.push(outcome),
            Ok(Err(e)) => {
                // generate_one itself failed (couldn't create en_dir,
                // couldn't write meta.json, etc.). Record under a
                // synthetic outcome so the end-of-run report shows it.
                all_outcomes.push(SdkOutcome {
                    sdk_id: "<unknown>".into(),
                    validation_errors: Vec::new(),
                    failures: vec![format!("generate_one bailed: {e:#}")],
                });
            }
            Err(e) => {
                all_outcomes.push(SdkOutcome {
                    sdk_id: "<panic>".into(),
                    validation_errors: Vec::new(),
                    failures: vec![format!("SDK task panicked: {e}")],
                });
            }
        }
        if let Some(co) = iter.next() {
            tasks.push(spawn_sdk(co, guides_dir.clone()));
        }
    }
    info!(count = total, "sdkgen done");

    // Build the aggregated failure report. Both classes of error gate
    // the build: validation_errors (missing methods, unextractable
    // return types) and failures (generator crashed or generate_one
    // bailed). Print SDK-grouped sections matching the Node format.
    let mut total_validation = 0usize;
    let mut total_failures = 0usize;
    for o in &all_outcomes {
        total_validation += o.validation_errors.len();
        total_failures += o.failures.len();
    }
    if total_validation == 0 && total_failures == 0 {
        return Ok(());
    }

    eprintln!();
    for o in &all_outcomes {
        if o.validation_errors.is_empty() && o.failures.is_empty() {
            continue;
        }
        eprintln!("{sdk}:", sdk = o.sdk_id);
        if !o.failures.is_empty() {
            eprintln!(
                "  {n} generator failure(s):",
                n = o.failures.len()
            );
            for f in &o.failures {
                eprintln!("    - {f}");
            }
        }
        if !o.validation_errors.is_empty() {
            eprintln!(
                "  {n} validation error(s):",
                n = o.validation_errors.len()
            );
            for v in &o.validation_errors {
                eprintln!("    - {v}");
            }
        }
        eprintln!();
    }
    eprintln!(
        "=== BUILD FAILED: {validation} validation error(s), {fail} generator failure(s) across {sdk_count} SDK(s) ===",
        validation = total_validation,
        fail = total_failures,
        sdk_count = all_outcomes.iter().filter(|o| !o.validation_errors.is_empty() || !o.failures.is_empty()).count()
    );
    anyhow::bail!(
        "sdkgen: {validation} validation error(s), {fail} generator failure(s)",
        validation = total_validation,
        fail = total_failures,
    )
}

fn spawn_sdk(
    co: Checkout,
    guides_dir: Arc<PathBuf>,
) -> tokio::task::JoinHandle<Result<SdkOutcome>> {
    tokio::spawn(async move { generate_one(&co, &guides_dir).await })
}

/// Result of running every generator for one SDK. Per-generator hard
/// failures (the generator didn't even return) become entries in
/// `failures`; per-operation defects (missing methods, missing return
/// types) accumulate as entries in `validation_errors`. `generate_all`
/// aggregates both across SDKs and exits non-zero if either is
/// non-empty, mirroring src/sdk-guide-generator.js:268-309.
#[derive(Debug, Default)]
struct SdkOutcome {
    sdk_id: String,
    validation_errors: Vec<String>,
    failures: Vec<String>,
}

async fn generate_one(checkout: &Checkout, guides_dir: &Path) -> Result<SdkOutcome> {
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
    let mut outcome = SdkOutcome {
        sdk_id: checkout.sdk.id.clone(),
        ..Default::default()
    };

    for kind in checkout.sdk.generators() {
        let generator: Box<dyn DocGenerator> = match kind.as_str() {
            "readme" => Box::new(ReadmeGenerator),
            "openapi" => Box::new(OpenApiGenerator),
            "typescript-ai" => Box::new(TypescriptAiGenerator),
            "rust-ai" => Box::new(RustAiGenerator),
            "cpp-ai" => Box::new(CppAiGenerator),
            "nim-ai" => Box::new(NimAiGenerator),
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
                outcome.validation_errors.extend(docs.validation_errors);
            }
            Err(e) => {
                // The generator itself crashed (config missing, parse
                // error, etc.). Node would have process.exit(1)'d on a
                // throw inside generateAll; mirror that by collecting
                // the failure and letting generate_all bail at the end.
                let msg = format!(
                    "sdk={} generator={kind} : {e:#}",
                    checkout.sdk.id
                );
                tracing::error!(error = %msg, "generator failed");
                outcome.failures.push(msg);
            }
        }
    }

    // Write intro / conclusion.
    if let Some(intro) = &merged_intro {
        std::fs::write(en_dir.join("intro.md"), intro)?;
    }
    if let Some(conclusion) = &merged_conclusion {
        std::fs::write(en_dir.join("conclusion.md"), conclusion)?;
    }

    // Write each section. Traversal guard: section.file comes from the
    // generator's output. A buggy or compromised upstream could produce
    // `../../etc/passwd.md`; reject anything that escapes en_dir or has
    // non-Normal components.
    for section in &mut all_sections {
        let mut filename = section
            .file
            .clone()
            .unwrap_or_else(|| format!("{}.md", readme::sanitize_filename(&section.name)));
        // Strip legacy `generated/` prefix.
        if let Some(rest) = filename.strip_prefix("generated/") {
            filename = rest.to_string();
        }
        let candidate = Path::new(&filename);
        if !is_safe_relative_path(candidate) {
            warn!(
                section = %section.name,
                file = %filename,
                "rejecting section.file with unsafe path components"
            );
            continue;
        }
        let file_path = en_dir.join(candidate);
        // After lexically joining, canonicalize the parent and confirm
        // it stays under en_dir's canonical path. en_dir exists already
        // (we created it above); the file parent may need creation
        // first if filename contains a subdirectory.
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
            let en_canon = en_dir.canonicalize().context("canonicalize en_dir")?;
            let parent_canon = parent
                .canonicalize()
                .with_context(|| format!("canonicalize {parent:?}"))?;
            if !parent_canon.starts_with(&en_canon) {
                warn!(
                    section = %section.name,
                    file = %filename,
                    "rejecting section.file: resolves outside en_dir"
                );
                continue;
            }
        }
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

    info!(
        sdk = %checkout.sdk.id,
        sections = all_sections.len(),
        validation_errors = outcome.validation_errors.len(),
        failures = outcome.failures.len(),
        "generated"
    );
    Ok(outcome)
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
    // Sort matches src/sdk-guide-generator.js:106-126 — uses
    // localeCompare which is case-insensitive for ASCII. We replicate
    // with `to_lowercase` keys so PascalCase methods order naturally.
    api.sort_by(|a, b| {
        let ca = a.sub_cat.as_deref().unwrap_or("Documentation");
        let cb = b.sub_cat.as_deref().unwrap_or("Documentation");
        let pa = category_priority(ca);
        let pb = category_priority(cb);
        pa.cmp(&pb)
            .then_with(|| ca.to_lowercase().cmp(&cb.to_lowercase()))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
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

    // Build the meta object with explicit insertion order so JSON.stringify
    // parity matches Node: name, pageHeader, [icon], itemsOrdered. Omit
    // `icon` entirely when the SDK has none (Node's JSON.stringify drops
    // undefined fields).
    let mut meta = serde_json::Map::new();
    meta.insert("name".into(), json!(sdk.name));
    meta.insert(
        "pageHeader".into(),
        json!(sdk.page_header.clone().unwrap_or_else(|| sdk.name.clone())),
    );
    if let Some(icon) = &sdk.icon {
        meta.insert("icon".into(), json!(icon));
    }
    meta.insert("itemsOrdered".into(), serde_json::Value::Array(items));
    serde_json::Value::Object(meta)
}

/// Accept only relative paths made of `Normal` components. Rejects
/// absolute paths, parent (`..`), Windows prefixes, and empty paths.
fn is_safe_relative_path(p: &Path) -> bool {
    let mut saw_any = false;
    for c in p.components() {
        match c {
            Component::Normal(_) => saw_any = true,
            _ => return false,
        }
    }
    saw_any
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

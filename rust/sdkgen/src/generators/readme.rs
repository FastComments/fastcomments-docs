//! README-based doc generator. Port of
//! `src/sdk-doc-generators/readme-generator.js`.

use anyhow::Result;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use regex::Regex;

use super::base::{DocGenerator, DocSection, GeneratedDocs, GeneratorCtx};

pub struct ReadmeGenerator;

#[async_trait]
impl DocGenerator for ReadmeGenerator {
    async fn generate(&self, ctx: &GeneratorCtx) -> Result<GeneratedDocs> {
        let mut sections: Vec<DocSection> = Vec::new();

        // README parsing.
        if let Some(readme_path) = find_readme(&ctx.repo_path) {
            let content = std::fs::read_to_string(&readme_path)?;
            sections.extend(parse_readme(
                &content,
                &ctx.sdk.repo,
                &ctx.sdk.branch,
                &ctx.sdk.id,
                &ctx.repo_path,
            ));
        } else {
            tracing::warn!(sdk = %ctx.sdk.id, "no README found");
        }

        // Additional docs/ markdown files.
        let docs_dir = ctx.repo_path.join("docs");
        if docs_dir.exists() {
            sections.extend(parse_docs_dir(
                &docs_dir,
                &ctx.sdk.repo,
                &ctx.sdk.branch,
                &ctx.sdk.id,
                &ctx.repo_path,
            ));
        }

        let intro = Some(generate_intro(&ctx.sdk));
        let conclusion = Some(generate_conclusion(&ctx.sdk));

        if sections.is_empty() {
            sections = generate_fallback(&ctx.sdk);
        }

        Ok(GeneratedDocs {
            intro,
            conclusion,
            sections,
            validation_errors: Vec::new(),
        })
    }
}

fn find_readme(repo: &std::path::Path) -> Option<std::path::PathBuf> {
    for name in ["README.md", "Readme.md", "readme.md", "README.MD", "README"] {
        let p = repo.join(name);
        if p.exists() {
            return Some(p);
        }
    }
    None
}

fn parse_readme(
    content: &str,
    repo_url: &str,
    branch: &str,
    sdk_id: &str,
    repo_path: &std::path::Path,
) -> Vec<DocSection> {
    let content = remove_front_matter(content);
    let mut out = Vec::new();
    // Split by `^## TITLE$`.
    static H2: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^##\s+(.+)$").expect("regex"));
    let matches: Vec<_> = H2.find_iter(&content).collect();
    if matches.is_empty() {
        let converted =
            convert_relative_links_for_sdk(&content, repo_url, branch, "", Some(sdk_id), Some(repo_path));
        out.push(DocSection {
            name: "Overview".to_string(),
            file: Some("overview-readme-generated.md".to_string()),
            content: converted,
            sub_cat: Some("Getting Started".to_string()),
            type_: Some("readme".to_string()),
            sidebar_item_classes: None,
        });
        return out;
    }

    for (i, m) in matches.iter().enumerate() {
        let start = m.start();
        let end = if i < matches.len() - 1 {
            matches[i + 1].start()
        } else {
            content.len()
        };
        let raw = &content[start..end];
        let title = H2
            .captures(raw)
            .and_then(|c| c.get(1))
            .map(|g| g.as_str().trim().to_string())
            .unwrap_or_default();
        // Strip the leading H2 (it'll be rendered by the front-end via the
        // section name). Matches `^##\s+.+\n` in readme-generator.js:104.
        let body = H2.replace(raw, "").trim_start_matches('\n').to_string();
        let body = body.trim().to_string();
        let body =
            convert_relative_links_for_sdk(&body, repo_url, branch, "", Some(sdk_id), Some(repo_path));
        let sub_cat = categorize(&title);
        if should_skip_section(&title) {
            continue;
        }
        out.push(DocSection {
            name: title.clone(),
            file: Some(format!("{}-readme-generated.md", sanitize_filename(&title))),
            content: body,
            sub_cat: Some(sub_cat),
            type_: Some("readme".to_string()),
            sidebar_item_classes: None,
        });
    }
    out
}

fn parse_docs_dir(
    dir: &std::path::Path,
    repo_url: &str,
    branch: &str,
    sdk_id: &str,
    repo_path: &std::path::Path,
) -> Vec<DocSection> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(&p) else { continue };
        let content = remove_front_matter(&content);
        let converted = convert_relative_links_for_sdk(
            &content,
            repo_url,
            branch,
            "docs/",
            Some(sdk_id),
            Some(repo_path),
        );
        let title = extract_title(&converted).unwrap_or_else(|| {
            p.file_stem()
                .map(|s| s.to_string_lossy().replace('-', " "))
                .unwrap_or_default()
        });
        // Strip leading H1. Mirror Node's `content.replace(/^#\s+.+\n/, '')`
        // — NO multiline flag, so leading whitespace (e.g. left over from
        // removed front matter) prevents the strip. Then trim.
        static H1: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A#\s+.+\n").expect("regex"));
        let body = H1.replace(&converted, "").trim().to_string();
        out.push(DocSection {
            name: title.clone(),
            file: Some(format!("{}-readme-generated.md", sanitize_filename(&title))),
            content: body,
            sub_cat: Some("Documentation".to_string()),
            type_: Some("readme".to_string()),
            sidebar_item_classes: None,
        });
    }
    out
}

fn remove_front_matter(content: &str) -> String {
    // Mirror Node's `/^---\n[\s\S]*?\n---\n/` at base-generator.js:74.
    // Use literal `\n` (not `\s*\n`) so we don't consume the blank line
    // after the closing `---` — that blank line is what prevents the
    // following H1 from being stripped by `\A#\s+.+\n`.
    static FM: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?s)\A---\n.*?\n---\n").expect("regex"));
    FM.replace(content, "").into_owned()
}

fn convert_relative_links(content: &str, repo_url: &str, branch: &str, prefix: &str) -> String {
    convert_relative_links_for_sdk(content, repo_url, branch, prefix, None, None)
}

/// Full port of `convertRelativeLinks` in
/// src/sdk-doc-generators/base-generator.js:155-206. When `sdk_id` and
/// `repo_path` are provided, image links get copied to
/// `src/static/generated/images/sdk-images/` (mirrors `copyImageToStatic`).
pub fn convert_relative_links_for_sdk(
    content: &str,
    repo_url: &str,
    branch: &str,
    base_path: &str,
    sdk_id: Option<&str>,
    repo_path: Option<&std::path::Path>,
) -> String {
    static LINK: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(!?)\[([^\]]+)\]\(([^)]+)\)").expect("regex"));
    LINK.replace_all(content, |caps: &regex::Captures| {
        let is_image = &caps[1];
        let text = &caps[2];
        let href = &caps[3];

        if href.starts_with("http://") || href.starts_with("https://") {
            return caps[0].to_string();
        }
        // Anchor links: sanitize + append `-readme-generated`.
        if let Some(anchor) = href.strip_prefix('#') {
            let sanitized = format!("{}-readme-generated", sanitize_filename(anchor));
            return format!("{is_image}[{text}](#{sanitized})");
        }

        // Resolve to a repo-root-relative path.
        let resolved = if let Some(rest) = href.strip_prefix('/') {
            rest.to_string()
        } else {
            posix_join(base_path, href)
        };
        let normalized = posix_normalize(&resolved);
        let repo_clean = repo_url.trim_end_matches(".git").trim_end_matches('/');

        if !is_image.is_empty() {
            if let (Some(id), Some(rp)) = (sdk_id, repo_path) {
                if let Some(local) = copy_image_to_static(id, rp, &normalized) {
                    return format!("![{text}]({local})");
                }
            }
            // Fallback to raw.githubusercontent.com.
            let raw_url = repo_clean.replace("https://github.com/", "https://raw.githubusercontent.com/");
            return format!("![{text}]({raw_url}/{branch}/{normalized})");
        }

        format!("[{text}]({repo_clean}/blob/{branch}/{normalized})")
    })
    .into_owned()
}

fn posix_join(a: &str, b: &str) -> String {
    if a.is_empty() {
        return b.to_string();
    }
    let trimmed_a = a.trim_end_matches('/');
    let trimmed_b = b.trim_start_matches("./");
    format!("{trimmed_a}/{trimmed_b}")
}

/// Best-effort posix path normalization: resolves `.` and `..` segments.
fn posix_normalize(path: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    for seg in path.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                if !out.is_empty() && out.last() != Some(&"..") {
                    out.pop();
                } else {
                    out.push("..");
                }
            }
            s => out.push(s),
        }
    }
    let joined = out.join("/");
    if path.ends_with('/') && !joined.ends_with('/') {
        format!("{joined}/")
    } else {
        joined
    }
}

/// Mirrors `copyImageToStatic` in src/sdk-doc-generators/base-generator.js:213-231.
fn copy_image_to_static(
    sdk_id: &str,
    repo_path: &std::path::Path,
    repo_relative_path: &str,
) -> Option<String> {
    let src = repo_path.join(repo_relative_path);
    if !src.exists() {
        tracing::warn!(path = %src.display(), "image not found in repo checkout");
        return None;
    }
    // Resolve the destination relative to the workspace static dir.
    // The Node code uses `__dirname/../static/generated/images/sdk-images`.
    // We resolve via repo_root() from the build module (the SDK repo
    // checkout is always under <repo_root>/src/content/sdks-checkout/).
    let repo_root = src
        .ancestors()
        .find(|p| p.join("src").join("locales.json").exists())?;
    let flat_name = format!("{sdk_id}--{}", repo_relative_path.replace('/', "-"));
    let dest_dir = repo_root.join("src/static/generated/images/sdk-images");
    if !dest_dir.exists() {
        std::fs::create_dir_all(&dest_dir).ok()?;
    }
    let dest = dest_dir.join(&flat_name);
    std::fs::copy(&src, &dest).ok()?;
    Some(format!("images/sdk-images/{flat_name}"))
}

fn extract_title(content: &str) -> Option<String> {
    static H1: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^#\s+(.+)$").expect("regex"));
    H1.captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
}

pub fn sanitize_filename(name: &str) -> String {
    static NON_ALNUM: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"[^a-z0-9]+").expect("regex"));
    let lower = name.to_lowercase();
    let collapsed = NON_ALNUM.replace_all(&lower, "-").to_string();
    collapsed.trim_matches('-').to_string()
}

pub fn categorize(title: &str) -> String {
    let lower = title.to_lowercase();
    if lower.contains("install") || lower.contains("setup") || lower.contains("getting started")
    {
        return "Getting Started".to_string();
    }
    if lower.contains("usage") || lower.contains("example") || lower.contains("quickstart") {
        return "Usage".to_string();
    }
    if lower.contains("api") || lower.contains("reference") || lower.contains("method") {
        return "API Reference".to_string();
    }
    if lower.contains("config") || lower.contains("option") {
        return "Configuration".to_string();
    }
    if lower.contains("auth") || lower.contains("security") {
        return "Authentication".to_string();
    }
    if lower.contains("contribut") {
        return "Contributing".to_string();
    }
    if lower.contains("license") {
        return "License".to_string();
    }
    "Documentation".to_string()
}

fn should_skip_section(title: &str) -> bool {
    let lower = title.to_lowercase();
    for kw in [
        "license",
        "contributing",
        "changelog",
        "contributors",
        "testing",
        "tests",
        "support",
        "development",
        "about this package",
    ] {
        if lower.contains(kw) {
            return true;
        }
    }
    false
}

fn generate_intro(sdk: &crate::config::SdkConfig) -> String {
    format!(
        "This is the official {} for FastComments.\n\n{}\n\n## Repository\n\n[View on GitHub]({})\n",
        sdk.name,
        sdk.description.clone().unwrap_or_default(),
        sdk.repo,
    )
}

fn generate_conclusion(sdk: &crate::config::SdkConfig) -> String {
    format!(
        "## Need Help?\n\nIf you encounter any issues or have questions about the {name}, please:\n\n- [Open an issue on GitHub]({repo}/issues)\n- [Contact FastComments Support](https://fastcomments.com/auth/my-account/help)\n\n## Contributing\n\nContributions are welcome! Please visit the [GitHub repository]({repo}) for contribution guidelines.\n",
        name = sdk.name,
        repo = sdk.repo,
    )
}

fn generate_fallback(sdk: &crate::config::SdkConfig) -> Vec<DocSection> {
    vec![DocSection {
        name: "Overview".to_string(),
        file: None,
        content: format!(
            "# {name}\n\n{desc}\n\nFor more information, please visit the [GitHub repository]({repo}).\n",
            name = sdk.name,
            desc = sdk.description.clone().unwrap_or_else(|| "Official SDK for the FastComments API.".to_string()),
            repo = sdk.repo,
        ),
        sub_cat: Some("Getting Started".to_string()),
        type_: Some("readme".to_string()),
        sidebar_item_classes: None,
    }]
}

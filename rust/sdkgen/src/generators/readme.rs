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
            convert_relative_links_for_sdk(&content, repo_url, branch, "", Some(sdk_id), repo_path);
        out.push(DocSection {
            name: "Overview".to_string(),
            file: Some("overview-readme-generated.md".to_string()),
            content: strip_leading_h1(&converted),
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
            convert_relative_links_for_sdk(&body, repo_url, branch, "", Some(sdk_id), repo_path);
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
            repo_path,
        );
        let title = extract_title(&converted).unwrap_or_else(|| {
            p.file_stem()
                .map(|s| s.to_string_lossy().replace('-', " "))
                .unwrap_or_default()
        });
        // Strip leading H1: the site renders the section `name` as the
        // item's heading, so keeping it would emit a second <h1> on the page.
        let body = strip_leading_h1(&converted);
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
    // The blank line it leaves behind is harmless: `strip_leading_h1`
    // skips leading whitespace.
    static FM: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?s)\A---\n.*?\n---\n").expect("regex"));
    FM.replace(content, "").into_owned()
}

/// Drop a document's own top-level heading, in either markdown form.
///
/// The generated item's `name` is rendered as the section heading by
/// `sitegen`, so a leading H1 in the body is always a duplicate. Node's
/// original `content.replace(/^#\s+.+\n/, '')` was anchored at offset 0
/// with no multiline flag, so the blank line left by front-matter removal
/// made it a no-op - that is how `# FastComments` reached every locale of
/// `guide-lib-vue-next`.
fn strip_leading_h1(content: &str) -> String {
    let trimmed = content.trim_start();
    let mut lines = trimmed.lines();
    let Some(first) = lines.next() else {
        return String::new();
    };
    // A document opening with a code fence has no heading to strip, and
    // `# ...` inside that fence is a comment, not markdown.
    if first.trim_start().starts_with("```") || first.trim_start().starts_with("~~~") {
        return trimmed.trim().to_string();
    }
    static ATX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A#\s+\S").expect("regex"));
    static SETEXT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A=+\s*\z").expect("regex"));
    let skip = if ATX.is_match(first) {
        1
    } else if lines.next().is_some_and(|second| SETEXT.is_match(second)) && !first.trim().is_empty()
    {
        2
    } else {
        return trimmed.trim().to_string();
    };
    trimmed
        .lines()
        .skip(skip)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

/// Full port of `convertRelativeLinks` in
/// src/sdk-doc-generators/base-generator.js:155-206, plus raw `<img src>`
/// rewriting. When `sdk_id` and `repo_path` are provided, image links get
/// copied to `src/static/generated/images/sdk-images/` (mirrors
/// `copyImageToStatic`).
///
/// READMEs reach for an HTML `<img>` whenever markdown can't express the
/// layout - fastcomments-react-native-sdk lays its screenshots out in a
/// `<table>` - and those relative `src`s resolved against docs.fastcomments.com
/// and 404'd until this handled them.
pub fn convert_relative_links_for_sdk(
    content: &str,
    repo_url: &str,
    branch: &str,
    base_path: &str,
    sdk_id: Option<&str>,
    repo_path: &std::path::Path,
) -> String {
    map_outside_code_fences(content, &|chunk: &str| {
        let chunk = convert_markdown_links(chunk, repo_url, branch, base_path, sdk_id, repo_path);
        convert_html_img_srcs(&chunk, repo_url, branch, base_path, sdk_id, repo_path)
    })
}

fn convert_markdown_links(
    content: &str,
    repo_url: &str,
    branch: &str,
    base_path: &str,
    sdk_id: Option<&str>,
    repo_path: &std::path::Path,
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

        if !is_image.is_empty() {
            return match resolve_asset_href(href, repo_url, branch, base_path, sdk_id, repo_path) {
                Some(url) => format!("![{text}]({url})"),
                None => caps[0].to_string(),
            };
        }

        // Resolve to a repo-root-relative path.
        let resolved = if let Some(rest) = href.strip_prefix('/') {
            rest.to_string()
        } else {
            posix_join(base_path, href)
        };
        let normalized = posix_normalize(&resolved);
        let repo_clean = repo_url.trim_end_matches(".git").trim_end_matches('/');
        // READMEs drift: they link directories (GitHub 301s /blob/ to /tree/)
        // and files that were renamed or deleted (404). The checkout is the
        // only place to tell those apart, so a missing target loses its link
        // rather than shipping a dead one.
        let target = repo_path.join(&normalized);
        if target.is_dir() {
            format!("[{text}]({repo_clean}/tree/{branch}/{normalized})")
        } else if target.exists() {
            format!("[{text}]({repo_clean}/blob/{branch}/{normalized})")
        } else {
            text.to_string()
        }
    })
    .into_owned()
}

fn convert_html_img_srcs(
    content: &str,
    repo_url: &str,
    branch: &str,
    base_path: &str,
    sdk_id: Option<&str>,
    repo_path: &std::path::Path,
) -> String {
    // `\s` before `src`, not `\b`: `\b` also matches inside `data-src`, and
    // the lazy `[^>]*?` would then rewrite the placeholder instead of the src.
    static IMG_SRC: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"(?i)(<img\b[^>]*?\ssrc\s*=\s*)(["'])([^"']*)(["'])"#).expect("regex")
    });
    IMG_SRC
        .replace_all(content, |caps: &regex::Captures| {
            let prefix = &caps[1];
            let open = &caps[2];
            let href = &caps[3];
            let close = &caps[4];
            match resolve_asset_href(href, repo_url, branch, base_path, sdk_id, repo_path) {
                Some(url) => format!("{prefix}{open}{url}{close}"),
                None => caps[0].to_string(),
            }
        })
        .into_owned()
}

/// Resolve one repo-relative image href to a URL the docs site can serve.
/// `None` means the href is already absolute and must be left untouched.
fn resolve_asset_href(
    href: &str,
    repo_url: &str,
    branch: &str,
    base_path: &str,
    sdk_id: Option<&str>,
    repo_path: &std::path::Path,
) -> Option<String> {
    let trimmed = href.trim();
    if trimmed.is_empty()
        || trimmed.starts_with('#')
        || trimmed.starts_with("//")
        || trimmed.starts_with("data:")
        || trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
    {
        return None;
    }
    let resolved = match trimmed.strip_prefix('/') {
        Some(rest) => rest.to_string(),
        None => posix_join(base_path, trimmed),
    };
    let normalized = posix_normalize(&resolved);
    if let Some(id) = sdk_id {
        if let Some(local) = copy_image_to_static(id, repo_path, &normalized) {
            return Some(local);
        }
    }
    // Fallback to raw.githubusercontent.com.
    let repo_clean = repo_url.trim_end_matches(".git").trim_end_matches('/');
    let raw_url = repo_clean.replace("https://github.com/", "https://raw.githubusercontent.com/");
    Some(format!("{raw_url}/{branch}/{normalized}"))
}

/// Apply `f` to every part of `content` that is not inside a fenced code
/// block. The lib-hugo / lib-11ty / lib-jekyll guides document
/// `<img src="/hero.jpg">` and markdown links as sample user markup, and
/// rewriting those turns working examples into nonsense.
fn map_outside_code_fences(content: &str, f: &dyn Fn(&str) -> String) -> String {
    let mut out = String::with_capacity(content.len());
    let mut buf = String::new();
    let mut fence: Option<(char, usize)> = None;
    for line in content.split_inclusive('\n') {
        let trimmed = line.trim_start();
        // A fence marker is indented at most 3 spaces; more makes it a code block.
        let marker = if line.len() - trimmed.len() <= 3 {
            fence_marker(trimmed)
        } else {
            None
        };
        match fence {
            Some((open_ch, open_len)) => {
                out.push_str(line);
                if matches!(marker, Some((ch, len)) if ch == open_ch && len >= open_len) {
                    fence = None;
                }
            }
            None => match marker {
                Some(m) => {
                    out.push_str(&f(&buf));
                    buf.clear();
                    out.push_str(line);
                    fence = Some(m);
                }
                None => buf.push_str(line),
            },
        }
    }
    out.push_str(&f(&buf));
    out
}

fn fence_marker(trimmed: &str) -> Option<(char, usize)> {
    let ch = trimmed.chars().next()?;
    if ch != '`' && ch != '~' {
        return None;
    }
    let len = trimmed.chars().take_while(|c| *c == ch).count();
    (len >= 3).then_some((ch, len))
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
        // No leading H1 - the section `name` is already rendered as the
        // item's heading, and a second <h1> fails the build's heading gate.
        content: format!(
            "{desc}\n\nFor more information, please visit the [GitHub repository]({repo}).\n",
            desc = sdk.description.clone().unwrap_or_else(|| format!("Official {} for the FastComments API.", sdk.name)),
            repo = sdk.repo,
        ),
        sub_cat: Some("Getting Started".to_string()),
        type_: Some("readme".to_string()),
        sidebar_item_classes: None,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_atx_h1_after_front_matter_blank_line() {
        // The exact shape of fastcomments-vue-next's docs/index.md.
        let doc = "---\ntitle: Introduction\n---\n\n# FastComments\n\nThis documentation contains a few examples.\n";
        let body = strip_leading_h1(&remove_front_matter(doc));
        assert_eq!(body, "This documentation contains a few examples.");
    }

    #[test]
    fn strips_atx_h1_with_no_front_matter() {
        assert_eq!(strip_leading_h1("# Title\n\nBody text.\n"), "Body text.");
    }

    #[test]
    fn strips_setext_h1() {
        assert_eq!(strip_leading_h1("Title\n=====\n\nBody text.\n"), "Body text.");
    }

    #[test]
    fn keeps_setext_h2_and_lower_headings() {
        // `---` is an H2 underline, not an H1.
        assert_eq!(strip_leading_h1("Title\n-----\n\nBody.\n"), "Title\n-----\n\nBody.");
        assert_eq!(strip_leading_h1("## Section\n\nBody.\n"), "## Section\n\nBody.");
    }

    #[test]
    fn keeps_body_that_has_no_leading_heading() {
        let doc = "Upload and resize an image\n\n## Parameters\n";
        assert_eq!(strip_leading_h1(doc), doc.trim());
    }

    #[test]
    fn leaves_comments_inside_an_opening_code_fence_alone() {
        let doc = "```bash\n# setup authorization\nexport KEY=1\n```\n";
        assert_eq!(strip_leading_h1(doc), doc.trim());
    }

    #[test]
    fn strips_only_the_first_heading() {
        let doc = "# Title\n\nBody.\n\n# Later\n";
        assert_eq!(strip_leading_h1(doc), "Body.\n\n# Later");
    }

    const REPO: &str = "https://github.com/FastComments/fastcomments-react-native-sdk";
    const RAW: &str =
        "https://raw.githubusercontent.com/FastComments/fastcomments-react-native-sdk/main";

    /// Link rewriting stats the checkout, so tests need a real tree.
    /// `sub/f.ts` is the file case; `sub/` is the directory case;
    /// anything else is the "target is gone" case.
    fn fixture_repo() -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("sub")).expect("mkdir");
        std::fs::write(dir.path().join("sub/f.ts"), "").expect("write");
        dir
    }

    fn convert(content: &str) -> String {
        let repo = fixture_repo();
        convert_relative_links_for_sdk(content, REPO, "main", "", None, repo.path())
    }

    #[test]
    fn rewrites_relative_html_img_src() {
        // The screenshot table in fastcomments-react-native-sdk's README.
        let doc = r#"<td><img src="./demo-screenshots/light.png" width="260" alt="Light"/></td>"#;
        assert_eq!(
            convert(doc),
            format!(r#"<td><img src="{RAW}/demo-screenshots/light.png" width="260" alt="Light"/></td>"#)
        );
    }

    #[test]
    fn rewrites_root_relative_and_single_quoted_html_img_src() {
        assert_eq!(
            convert("<img src='/docs/a.png'><IMG SRC = \"b.png\">"),
            format!("<img src='{RAW}/docs/a.png'><IMG SRC = \"{RAW}/b.png\">")
        );
    }

    #[test]
    fn rewrites_the_real_src_not_a_data_src_placeholder() {
        assert_eq!(
            convert(r#"<img data-src="./big.png" src="./thumb.png">"#),
            format!(r#"<img data-src="./big.png" src="{RAW}/thumb.png">"#)
        );
    }

    #[test]
    fn leaves_absolute_html_img_src_alone() {
        let doc = "<img src=\"https://img.shields.io/npm/v/x\"><img src=\"data:image/png;base64,AA\">";
        assert_eq!(convert(doc), doc);
    }

    #[test]
    fn leaves_html_img_inside_a_code_fence_alone() {
        // lib-hugo / lib-11ty / lib-jekyll document this as sample user markup.
        let doc = "Target an image:\n\n```text\n<img id=\"hero\" src=\"/hero.jpg\" alt=\"Hero\" />\n```\n\n<img src=\"./real.png\">\n";
        assert_eq!(
            convert(doc),
            format!("Target an image:\n\n```text\n<img id=\"hero\" src=\"/hero.jpg\" alt=\"Hero\" />\n```\n\n<img src=\"{RAW}/real.png\">\n")
        );
    }

    #[test]
    fn leaves_markdown_links_inside_a_code_fence_alone() {
        let doc = "~~~md\n![shot](./a.png)\n[docs](./b.md)\n~~~\n\n![shot](./a.png)\n";
        assert_eq!(
            convert(doc),
            format!("~~~md\n![shot](./a.png)\n[docs](./b.md)\n~~~\n\n![shot]({RAW}/a.png)\n")
        );
    }

    #[test]
    fn still_rewrites_markdown_images_links_and_anchors() {
        let doc = "![shot](./x/../y.png) [src](sub/f.ts) [top](#Getting Started) [ext](https://a.b)";
        assert_eq!(
            convert(doc),
            format!(
                "![shot]({RAW}/y.png) [src]({REPO}/blob/main/sub/f.ts) [top](#getting-started-readme-generated) [ext](https://a.b)"
            )
        );
    }

    #[test]
    fn links_a_directory_as_tree_not_blob() {
        // fastcomments-angular's README links `projects/fastcomments-angular`;
        // GitHub 301s /blob/ to /tree/ for a directory.
        assert_eq!(
            convert("[the lib](sub)"),
            format!("[the lib]({REPO}/tree/main/sub)")
        );
    }

    #[test]
    fn drops_the_link_when_the_target_is_gone() {
        // fastcomments-python's README links ./sso/README.md, which
        // doesn't exist — better unlinked text than a 404.
        assert_eq!(convert("[SSO docs](sso/README.md)"), "SSO docs");
    }
}

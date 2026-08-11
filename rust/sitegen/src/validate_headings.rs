//! Build gate: every generated page must have exactly one `<h1>`.
//!
//! Duplicate H1s reached production twice from two unrelated directions -
//! a README section whose leading H1 survived `sdkgen`'s strip, and
//! translators promoting a leading paragraph to a heading - so the check
//! runs against the rendered HTML rather than the markdown source. That
//! is the only place both causes are visible.

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::{info, warn};

/// Matches an opening `<h1>` tag. Markdown-rendered code samples escape
/// their angle brackets, so a literal `<h1` in the output is a heading.
static H1: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)<h1[\s>]").expect("regex"));

/// Text of each H1, for the failure message.
static H1_TEXT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<h1[^>]*>(.*?)</h1>").expect("regex"));

pub async fn run() -> Result<()> {
    let repo = super::build::repo_root()?;
    let generated = repo.join("src/static/generated");
    let mut offenders = Vec::new();
    let mut missing = 0usize;
    let mut checked = 0usize;

    let mut entries: Vec<_> = std::fs::read_dir(&generated)
        .with_context(|| format!("read {}", generated.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("html"))
        .collect();
    entries.sort();

    for path in entries {
        let html = std::fs::read_to_string(&path)
            .with_context(|| format!("read {}", path.display()))?;
        checked += 1;
        let count = H1.find_iter(&html).count();
        if count == 0 {
            missing += 1;
        } else if count > 1 {
            offenders.push((path, headings(&html)));
        }
    }

    if missing > 0 {
        // Not fatal: `code-*` sample pages and a handful of regional
        // locales legitimately render without a page header today.
        warn!(missing, "pages with no h1");
    }

    if !offenders.is_empty() {
        // A single bad item file fans out across every locale, so cap the
        // listing - the pattern is visible long before the 30th line.
        const MAX_LISTED: usize = 30;
        for (path, texts) in offenders.iter().take(MAX_LISTED) {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            eprintln!("  {name}: {} h1 tags -> {}", texts.len(), texts.join(" | "));
        }
        if offenders.len() > MAX_LISTED {
            eprintln!("  ... and {} more", offenders.len() - MAX_LISTED);
        }
        anyhow::bail!(
            "{} of {checked} generated pages have more than one <h1>",
            offenders.len()
        );
    }

    info!(checked, missing, "validate-headings passed");
    Ok(())
}

fn headings(html: &str) -> Vec<String> {
    H1_TEXT
        .captures_iter(html)
        .map(|c| {
            let inner = c.get(1).map(|m| m.as_str()).unwrap_or_default();
            let text = strip_tags(inner);
            text.split_whitespace().collect::<Vec<_>>().join(" ")
        })
        .collect()
}

fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut depth = 0usize;
    for ch in s.chars() {
        match ch {
            '<' => depth += 1,
            '>' => depth = depth.saturating_sub(1),
            _ if depth == 0 => out.push(ch),
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(html: &str) -> usize {
        H1.find_iter(html).count()
    }

    #[test]
    fn counts_only_real_h1_tags() {
        assert_eq!(count("<h1>One</h1>"), 1);
        assert_eq!(count("<h1 id=\"a\">One</h1><h1>Two</h1>"), 2);
        // Escaped sample code is not a heading.
        assert_eq!(count("<code>&lt;h1&gt;hi&lt;/h1&gt;</code>"), 0);
        // Don't match a longer tag name as a prefix.
        assert_eq!(count("<h11>x</h11>"), 0);
    }

    #[test]
    fn extracts_heading_text() {
        let html = "<h1 id=\"a\">Add Comments <a href=\"#a\">link</a></h1><h1>FastComments</h1>";
        assert_eq!(headings(html), vec!["Add Comments link", "FastComments"]);
    }
}

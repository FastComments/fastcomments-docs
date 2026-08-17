//! Backstop for generated links into an SDK's own repo.
//!
//! All three URL builders (`readme::convert_markdown_links`,
//! `openapi::generate_type_github_url`, `ai::common::type_github_url`) now
//! stat the checkout before emitting a link, so in principle nothing broken
//! can escape. In practice the AI generators paste model text straight into
//! markdown and can route around the builders, and that is exactly how the
//! doubled C++ model path and the `a_p_i_empty_response.rb` names shipped.
//!
//! This runs against the freshly-fetched checkout, in the same process that
//! wrote the markdown, so it is offline, deterministic, and cannot observe a
//! stale tree. It only judges links pointing at the SDK's *own* repo -
//! anything else has no local ground truth.

use std::path::Path;

use once_cell::sync::Lazy;
use regex::Regex;

static GITHUB_LINK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"https://github\.com/([\w.-]+/[\w.-]+)/(blob|tree)/([^/\s)\]]+)/([^\s)\]\}>\x22']+)")
        .expect("regex")
});

/// One bad link, formatted for `SdkOutcome::validation_errors`.
pub struct BadLink {
    pub url: String,
    pub reason: &'static str,
}

/// Every link in `content` that points into `repo_url` but doesn't resolve
/// against `repo_path`. `branch` filters out links deliberately pinned to
/// another ref, which we can't verify from a single checkout.
pub fn bad_links(content: &str, repo_url: &str, branch: &str, repo_path: &Path) -> Vec<BadLink> {
    let Some(own) = repo_slug(repo_url) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for caps in GITHUB_LINK.captures_iter(content) {
        let slug = &caps[1];
        if !slug.eq_ignore_ascii_case(&own) {
            continue;
        }
        if &caps[3] != branch {
            continue;
        }
        // Trailing markdown punctuation is not part of the path, and a
        // `#L39` line anchor isn't either.
        let path = caps[4]
            .split('#')
            .next()
            .unwrap_or_default()
            .trim_end_matches(['.', ',', ';', ':']);
        if path.is_empty() {
            continue;
        }
        let target = repo_path.join(path);
        let reason = match &caps[2] {
            "blob" if target.is_dir() => "links a directory with /blob/ (GitHub redirects to /tree/)",
            "blob" if !target.exists() => "no such file in the checkout",
            "tree" if target.is_file() => "links a file with /tree/",
            "tree" if !target.exists() => "no such directory in the checkout",
            _ => continue,
        };
        out.push(BadLink {
            url: caps[0].to_string(),
            reason,
        });
    }
    out
}

/// `https://github.com/Org/Repo(.git)` -> `Org/Repo`.
fn repo_slug(repo_url: &str) -> Option<String> {
    let rest = repo_url
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .strip_prefix("https://github.com/")?;
    if rest.contains('/') {
        Some(rest.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REPO: &str = "https://github.com/FastComments/fastcomments-cpp";

    fn fixture() -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("client/include")).expect("mkdir");
        std::fs::write(dir.path().join("client/include/Ok.h"), "").expect("write");
        dir
    }

    #[test]
    fn accepts_a_file_linked_with_blob_and_a_dir_linked_with_tree() {
        let dir = fixture();
        let doc = format!(
            "[a]({REPO}/blob/master/client/include/Ok.h) [b]({REPO}/tree/master/client/include)"
        );
        assert!(bad_links(&doc, REPO, "master", dir.path()).is_empty());
    }

    #[test]
    fn flags_the_doubled_model_path() {
        let dir = fixture();
        let doc = format!("[x]({REPO}/blob/master/client/include/client/include/Ok.h)");
        let bad = bad_links(&doc, REPO, "master", dir.path());
        assert_eq!(bad.len(), 1);
        assert_eq!(bad[0].reason, "no such file in the checkout");
    }

    #[test]
    fn flags_a_directory_linked_as_blob() {
        let dir = fixture();
        let doc = format!("[x]({REPO}/blob/master/client/include)");
        let bad = bad_links(&doc, REPO, "master", dir.path());
        assert_eq!(bad.len(), 1);
        assert!(bad[0].reason.contains("directory"));
    }

    #[test]
    fn ignores_line_anchors_and_trailing_punctuation() {
        let dir = fixture();
        let doc = format!("See {REPO}/blob/master/client/include/Ok.h#L39, and that's it.");
        assert!(bad_links(&doc, REPO, "master", dir.path()).is_empty());
    }

    #[test]
    fn ignores_other_repos_and_other_branches() {
        let dir = fixture();
        let doc = format!(
            "[a](https://github.com/FastComments/fastcomments-ruby/blob/master/nope.rb) \
             [b]({REPO}/blob/some-sha/nope.h)"
        );
        assert!(bad_links(&doc, REPO, "master", dir.path()).is_empty());
    }
}

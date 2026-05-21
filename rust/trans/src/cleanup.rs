//! `trans cleanup` — port of `src/cleanup-empty-translations.js` and
//! `src/cleanup-empty-generated.js`.
//!
//! Two failure modes the prior Rust port missed:
//!
//! * `src/cleanup-empty-translations.js` deletes
//!   `items/<locale>/intro.md` / `conclusion.md` whose body is `""`,
//!   `"---"`, or `"---\n\n---"` after trim. These are the empty
//!   front-matter sentinels the translator emits when the source file
//!   was itself empty/sentinel. The old Rust cleanup only removed
//!   zero-byte files, so the `---` cases stayed forever and the
//!   site shipped near-empty intros.
//!
//! * `src/cleanup-empty-generated.js` scans SDK API-generated
//!   markdown (`*-api-generated.md` under each SDK guide) for
//!   `[inline-code-start]...[inline-code-end]` blocks whose body is
//!   empty or near-empty (Node uses `length < 10` after trim).
//!   Those are AI-generation duds — shipping them produces a code
//!   sample with nothing in the snippet. The old Rust cleanup
//!   ignored this entirely.
//!
//! Behavior contract here:
//!
//! * `trans cleanup` (no flags) is a DRY RUN. It prints what would
//!   be deleted; it never writes. Differs from Node's
//!   cleanup-empty-translations.js which always deleted — the safer
//!   default lets a `trans cleanup` in CI flag drift without
//!   silently mutating the tree.
//! * `trans cleanup --delete` actually removes the files for both
//!   classes.
//! * `--only translations` / `--only generated` restricts scope when
//!   investigating one category.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tracing::{info, warn};

/// What to clean up. `--only` defaults to `Both`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Both,
    Translations,
    Generated,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::Both
    }
}

/// CLI options for `trans cleanup`.
#[derive(Debug, Default, Clone)]
pub struct Options {
    /// When false (the default), report what would be deleted without
    /// writing. Matches `cleanup-empty-generated.js` default; safer
    /// than `cleanup-empty-translations.js` which always deleted.
    pub delete: bool,
    pub scope: Scope,
}

pub fn parse_options<I: IntoIterator<Item = String>>(args: I) -> Result<Options> {
    let mut opts = Options::default();
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--delete" => opts.delete = true,
            "--only" => {
                let v = iter
                    .next()
                    .context("--only requires a value (translations|generated)")?;
                opts.scope = match v.as_str() {
                    "translations" => Scope::Translations,
                    "generated" => Scope::Generated,
                    "both" => Scope::Both,
                    other => anyhow::bail!(
                        "--only must be one of translations|generated|both, got {other:?}"
                    ),
                };
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => anyhow::bail!("unknown arg: {other:?} (try --help)"),
        }
    }
    Ok(opts)
}

fn print_help() {
    let lines: &[&str] = &[
        "Usage: trans cleanup [options]",
        "",
        "Reports — or with --delete, removes — two classes of stale files:",
        "  * items/<locale>/intro.md|conclusion.md whose body is empty,",
        "    \"---\", or \"---\\n\\n---\" (port of cleanup-empty-translations.js).",
        "  * SDK *-api-generated.md whose [inline-code-start]...[inline-code-end]",
        "    block is empty / < 10 chars after trim (port of",
        "    cleanup-empty-generated.js).",
        "",
        "Options:",
        "  --delete             Actually remove the files. Without this flag, runs",
        "                       as a dry-run and prints the would-delete list.",
        "  --only <scope>       translations | generated | both (default both)",
        "  --help, -h           Show this message.",
    ];
    for l in lines {
        println!("{l}");
    }
}

pub async fn run_with(opts: Options) -> Result<()> {
    let repo = repo_root()?;
    let guides = repo.join("src/content/guides");

    let mut translation_candidates: Vec<PathBuf> = Vec::new();
    let mut generated_candidates: Vec<(PathBuf, String)> = Vec::new();

    if matches!(opts.scope, Scope::Both | Scope::Translations) {
        translation_candidates = find_empty_translations(&guides)?;
    }
    if matches!(opts.scope, Scope::Both | Scope::Generated) {
        generated_candidates = find_empty_generated(&guides)?;
    }

    let mode = if opts.delete { "DELETE" } else { "DRY-RUN" };
    info!(
        mode,
        translations = translation_candidates.len(),
        generated = generated_candidates.len(),
        "trans cleanup"
    );

    let mut removed = 0usize;
    if !translation_candidates.is_empty() {
        info!("--- empty translation files ---");
        for p in &translation_candidates {
            info!("  {}", p.display());
            removed += try_delete(p, opts.delete);
        }
    }
    if !generated_candidates.is_empty() {
        info!("--- generated SDK files with empty/minimal code blocks ---");
        for (p, reason) in &generated_candidates {
            info!("  {}  [{reason}]", p.display());
            removed += try_delete(p, opts.delete);
        }
    }

    let total_candidates = translation_candidates.len() + generated_candidates.len();
    if !opts.delete && total_candidates > 0 {
        info!(
            "(dry-run) {total_candidates} file(s) would be removed. Re-run with --delete to apply."
        );
    } else if opts.delete {
        info!("removed {removed} file(s)");
    }
    Ok(())
}

/// Iterate every immediate-child directory of `dir`, calling `cb` on
/// each. Silently skips IO failures (matches Node's behavior of
/// quietly walking past unreadable entries during cleanup scans).
fn for_each_subdir(dir: &Path, mut cb: impl FnMut(&Path)) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let Ok(ft) = entry.file_type() else { continue };
        if ft.is_dir() {
            cb(&entry.path());
        }
    }
}

/// Helper for the cleanup-action loop. Returns 1 if a file was
/// removed, 0 otherwise (including dry-run and IO failure). Exists
/// because both candidate categories want exactly the same
/// "log → remove iff --delete → count" dance.
fn try_delete(path: &Path, delete: bool) -> usize {
    if !delete {
        return 0;
    }
    match std::fs::remove_file(path) {
        Ok(()) => 1,
        Err(e) => {
            warn!(path = %path.display(), error = %e, "failed to remove");
            0
        }
    }
}

/// Mirrors `cleanup-empty-translations.js`: finds every
/// `guides/<id>/items/<locale>/{intro,conclusion}.md` whose content
/// (after trim) is `""`, `"---"`, or `"---\n\n---"`.
fn find_empty_translations(guides_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for_each_subdir(guides_dir, |guide_path| {
        let items_path = guide_path.join("items");
        for_each_subdir(&items_path, |locale_path| {
            for special in ["intro.md", "conclusion.md"] {
                let p = locale_path.join(special);
                if !p.exists() {
                    continue;
                }
                if let Ok(body) = std::fs::read_to_string(&p) {
                    if is_empty_translation_body(&body) {
                        out.push(p);
                    }
                }
            }
        });
    });
    Ok(out)
}

/// Mirrors the Node check at cleanup-empty-translations.js:40-44/53-57:
///   if (content === '---\n\n---' || content === '---' || content === '')
///
/// We compare against the trimmed body — Node also `.trim()`s first.
pub fn is_empty_translation_body(content: &str) -> bool {
    let t = content.trim();
    t.is_empty() || t == "---" || t == "---\n\n---"
}

/// Mirrors `cleanup-empty-generated.js`: scans
/// `guides/<sdk>/items/{generated,<default-locale>}/*-api-generated.md`
/// for empty / minimal `[inline-code-start]...[inline-code-end]`
/// blocks.
///
/// The Node script only looked at `items/generated/` because that was
/// the old layout. Our Rust sdkgen writes to `items/en/` instead, so
/// we scan both — that's the only difference from a strict
/// translation; everything else (file-name filter, block extraction,
/// length threshold) matches Node line 21-46.
fn find_empty_generated(guides_dir: &Path) -> Result<Vec<(PathBuf, String)>> {
    let mut out = Vec::new();
    for_each_subdir(guides_dir, |guide_path| {
        let name = guide_path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        if !name.starts_with("sdk-") {
            return;
        }
        let items = guide_path.join("items");
        if !items.exists() {
            return;
        }
        // Walk every items/<subdir>/*-api-generated.md regardless of
        // whether <subdir> is `generated/` (legacy Node layout) or
        // `<default_locale>/` (Rust sdkgen layout). Skips other locale
        // subdirs because translated copies of empty files would
        // already match the translation-cleanup path above.
        for_each_subdir(&items, |sub_path| {
            let sub_name = sub_path
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default();
            // Node only inspected `generated/`; we also include `en`
            // (default locale) since that's where Rust sdkgen writes
            // today. Other locale dirs are translations and handled
            // by the translation-cleanup path above.
            if sub_name != "generated" && sub_name != "en" {
                return;
            }
            let Ok(files) = std::fs::read_dir(sub_path) else {
                return;
            };
            for f in files.flatten() {
                let fp = f.path();
                let Some(fname) = fp.file_name().and_then(|s| s.to_str()) else {
                    continue;
                };
                if !fname.ends_with("-api-generated.md") {
                    continue;
                }
                let Ok(content) = std::fs::read_to_string(&fp) else {
                    continue;
                };
                if let Some(reason) = empty_code_block_reason(&content) {
                    out.push((fp, reason));
                }
            }
        });
    });
    Ok(out)
}

/// Inspect a `*-api-generated.md` body for empty/minimal code blocks.
/// Returns the reason string Node's cleanup-empty-generated.js
/// would log, or `None` if the file is fine.
///
/// Node behavior (line 28-46):
///   * No `[inline-code-start]` OR no `[inline-code-end]` →
///     "missing code block markers".
///   * Extract content between markers, trim, length < 10 →
///     "empty or minimal code block".
pub fn empty_code_block_reason(content: &str) -> Option<String> {
    let start = content.find("[inline-code-start]");
    let end = content.find("[inline-code-end]");
    if start.is_none() || end.is_none() {
        return Some("missing code block markers".into());
    }
    let start = start.unwrap();
    let end = end.unwrap();
    if end < start {
        return Some("missing code block markers".into());
    }
    // Extract between the END of `[inline-code-start]` and the START
    // of `[inline-code-end]`.
    let inner_start = start + "[inline-code-start]".len();
    if inner_start > end {
        return Some("missing code block markers".into());
    }
    let body = content[inner_start..end].trim();
    if body.len() < 10 {
        return Some(format!("empty or minimal code block (len={})", body.len()));
    }
    None
}

use fcdocs_shared::repo::repo_root;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_translation_body_matches_node_set() {
        // Exact set Node deletes (cleanup-empty-translations.js:40):
        //   '', '---', '---\n\n---'  (all after .trim()).
        assert!(is_empty_translation_body(""));
        assert!(is_empty_translation_body("   "));
        assert!(is_empty_translation_body("---"));
        assert!(is_empty_translation_body("---\n"));
        assert!(is_empty_translation_body("---\n\n---"));
        assert!(is_empty_translation_body("\n---\n\n---\n"));
        // NON-matches: any actual content survives.
        assert!(!is_empty_translation_body("# Hello"));
        assert!(!is_empty_translation_body("---\nactual body\n---"));
        assert!(!is_empty_translation_body("- single dash"));
        assert!(!is_empty_translation_body("--"));
        assert!(!is_empty_translation_body("----"));
    }

    #[test]
    fn empty_code_block_reasons() {
        // Both markers missing.
        assert!(empty_code_block_reason("# title").is_some());
        // Only one marker present.
        assert!(empty_code_block_reason("[inline-code-start]\nfoo").is_some());
        assert!(empty_code_block_reason("[inline-code-end]\nfoo").is_some());
        // Empty block.
        let r = empty_code_block_reason("[inline-code-start][inline-code-end]").unwrap();
        assert!(r.contains("empty or minimal"));
        // Whitespace-only block.
        let r = empty_code_block_reason("[inline-code-start]\n   \n[inline-code-end]").unwrap();
        assert!(r.contains("empty or minimal"));
        // Just under threshold (Node: < 10 chars after trim).
        let r = empty_code_block_reason("[inline-code-start]\nfoo();\n[inline-code-end]").unwrap();
        assert!(r.contains("empty or minimal"), "expected minimal flag, got {r:?}");
        // At threshold (>= 10 chars).
        let body = "let x = 1;";
        assert_eq!(body.len(), 10);
        let ok = format!("[inline-code-start]\n{body}\n[inline-code-end]");
        assert!(empty_code_block_reason(&ok).is_none(), "10 chars should pass");
        // Real content well over threshold.
        let ok = "[inline-code-start]\nconst client = new FastComments();\n[inline-code-end]";
        assert!(empty_code_block_reason(ok).is_none());
    }

    #[test]
    fn end_before_start_treated_as_missing_markers() {
        let bad = "[inline-code-end] body [inline-code-start]";
        assert!(empty_code_block_reason(bad).is_some());
    }

    #[test]
    fn find_empty_translations_walks_locale_dirs() {
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        // Guide A: empty intro, real conclusion.
        std::fs::create_dir_all(g.join("guide-a/items/fr_fr")).unwrap();
        std::fs::write(g.join("guide-a/items/fr_fr/intro.md"), "---\n\n---").unwrap();
        std::fs::write(
            g.join("guide-a/items/fr_fr/conclusion.md"),
            "# Real Conclusion\n",
        )
        .unwrap();
        // Guide A in another locale: empty conclusion.
        std::fs::create_dir_all(g.join("guide-a/items/de_de")).unwrap();
        std::fs::write(g.join("guide-a/items/de_de/conclusion.md"), "").unwrap();
        // Should not touch non-intro/conclusion files.
        std::fs::write(g.join("guide-a/items/fr_fr/other.md"), "").unwrap();

        let found = find_empty_translations(g).unwrap();
        assert_eq!(found.len(), 2);
        assert!(found.iter().any(|p| p.ends_with("fr_fr/intro.md")));
        assert!(found.iter().any(|p| p.ends_with("de_de/conclusion.md")));
        assert!(!found.iter().any(|p| p.ends_with("other.md")));
    }

    #[test]
    fn find_empty_generated_scans_both_layouts() {
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        // SDK with legacy generated/ layout.
        std::fs::create_dir_all(g.join("sdk-cpp/items/generated")).unwrap();
        std::fs::write(
            g.join("sdk-cpp/items/generated/foo-api-generated.md"),
            "[inline-code-start]\n\n[inline-code-end]",
        )
        .unwrap();
        // SDK with new en/ layout.
        std::fs::create_dir_all(g.join("sdk-rust/items/en")).unwrap();
        std::fs::write(
            g.join("sdk-rust/items/en/bar-api-generated.md"),
            "no markers here",
        )
        .unwrap();
        // SDK with real content — must NOT be flagged.
        std::fs::write(
            g.join("sdk-rust/items/en/ok-api-generated.md"),
            "[inline-code-start]\nlet client = FastComments::new();\n[inline-code-end]",
        )
        .unwrap();
        // Non-SDK guide — must be ignored entirely.
        std::fs::create_dir_all(g.join("api/items/generated")).unwrap();
        std::fs::write(
            g.join("api/items/generated/something-api-generated.md"),
            "[inline-code-start]\n\n[inline-code-end]",
        )
        .unwrap();
        // SDK file but wrong suffix — must be ignored.
        std::fs::write(
            g.join("sdk-rust/items/en/readme.md"),
            "[inline-code-start]\n\n[inline-code-end]",
        )
        .unwrap();

        let found = find_empty_generated(g).unwrap();
        assert_eq!(
            found.len(),
            2,
            "expected exactly 2 hits, got: {found:#?}"
        );
        assert!(found.iter().any(|(p, _)| p.ends_with("foo-api-generated.md")));
        assert!(found.iter().any(|(p, _)| p.ends_with("bar-api-generated.md")));
    }

    // --- CLI parser ---

    fn args(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn cleanup_defaults_to_dry_run_both() {
        let o = parse_options(args(&[])).unwrap();
        assert!(!o.delete);
        assert_eq!(o.scope, Scope::Both);
    }

    #[test]
    fn cleanup_parses_delete_and_only() {
        let o = parse_options(args(&["--delete", "--only", "translations"])).unwrap();
        assert!(o.delete);
        assert_eq!(o.scope, Scope::Translations);
    }

    #[test]
    fn cleanup_only_must_be_valid_value() {
        assert!(parse_options(args(&["--only", "nonsense"])).is_err());
        assert!(parse_options(args(&["--only"])).is_err()); // missing value
    }

    #[test]
    fn cleanup_unknown_arg_rejected() {
        let e = parse_options(args(&["--wat"])).unwrap_err();
        assert!(e.to_string().contains("unknown arg"));
    }
}

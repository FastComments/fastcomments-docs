//! Shared file-discovery helpers for the translation pipeline.
//!
//! Mirrors src/check-translations.js::getDefaultLocaleFiles (line
//! 141-161) which has a special-case: even when `intro.md` /
//! `conclusion.md` don't appear in `items/<defaultLocale>/`, they're
//! treated as default-locale files if they exist at the guide root
//! (`guides/<id>/intro.md` / `guides/<id>/conclusion.md`). Translations
//! for those still land under `items/<locale>/intro.md` /
//! `items/<locale>/conclusion.md` — the placement is normalized on
//! translation, not preserved.
//!
//! Without this special case, both `trans check` and `trans run`
//! silently skip ~157 root-level files across ~80 guides.

use std::path::{Path, PathBuf};

/// One discovered default-locale source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    /// File basename (e.g. `intro.md`). This is the key used in cache
    /// + the basename under `items/<locale>/` where the translation
    /// goes — never includes a leading directory.
    pub filename: String,
    /// Absolute path the source content is read from. May be
    /// `<guide>/items/<default_locale>/<filename>` or
    /// `<guide>/<filename>` (root-level intro/conclusion fallback).
    pub source_path: PathBuf,
}

/// Enumerate the default-locale source files for a guide.
///
/// Order:
///   1. Every `.md` under `<guide>/items/<default_locale>/`.
///   2. `intro.md` at the guide root, if it exists AND wasn't already
///      listed under (1).
///   3. `conclusion.md` likewise.
///
/// `guide_dir` is the absolute path to `src/content/guides/<id>`.
/// `default_locale` is e.g. `"en"`.
pub fn default_locale_files(guide_dir: &Path, default_locale: &str) -> Vec<SourceFile> {
    let mut out = Vec::new();
    let items_default = guide_dir.join("items").join(default_locale);
    if let Ok(entries) = std::fs::read_dir(&items_default) {
        for e in entries.flatten() {
            let p = e.path();
            if p.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            let Some(name) = p.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            out.push(SourceFile {
                filename: name.to_string(),
                source_path: p,
            });
        }
    }
    // Root-level intro.md / conclusion.md fallback. Only added if not
    // already present under items/<default_locale>/. Matches Node's
    // "if (... && !files.includes('intro.md'))" guard at line 153/156.
    for special in ["intro.md", "conclusion.md"] {
        if out.iter().any(|s| s.filename == special) {
            continue;
        }
        let root = guide_dir.join(special);
        let items_path = items_default.join(special);
        if items_path.exists() {
            // Already would have been picked up above; defensive
            // — read_dir might have failed earlier.
            out.push(SourceFile {
                filename: special.to_string(),
                source_path: items_path,
            });
        } else if root.exists() {
            out.push(SourceFile {
                filename: special.to_string(),
                source_path: root,
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(root: &Path, rel: &str, body: &str) {
        let p = root.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(&p, body).unwrap();
    }

    #[test]
    fn picks_up_items_default_locale() {
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write(g, "items/en/a.md", "A");
        write(g, "items/en/b.md", "B");
        let files = default_locale_files(g, "en");
        let names: Vec<&str> = files.iter().map(|s| s.filename.as_str()).collect();
        // Sort for stable comparison — readdir order is unspecified.
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(sorted, vec!["a.md", "b.md"]);
    }

    #[test]
    fn root_intro_is_appended_when_items_dir_lacks_it() {
        // The regression case: guide uses guide-root intro.md, no
        // intro.md under items/en/. Node picks it up; the prior Rust
        // walks didn't.
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write(g, "items/en/howto.md", "HT");
        write(g, "intro.md", "Welcome");
        let files = default_locale_files(g, "en");
        let mut names: Vec<&str> = files.iter().map(|s| s.filename.as_str()).collect();
        names.sort();
        assert_eq!(names, vec!["howto.md", "intro.md"]);
        // intro.md's source_path must point at the GUIDE ROOT, not
        // items/en/. Subsequent reads of source content depend on this.
        let intro = files.iter().find(|s| s.filename == "intro.md").unwrap();
        assert_eq!(intro.source_path, g.join("intro.md"));
    }

    #[test]
    fn items_dir_intro_wins_over_root_intro() {
        // If both exist, the items/<default>/ copy is canonical
        // (matches Node's `if (!files.includes('intro.md'))` guard).
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write(g, "items/en/intro.md", "items-version");
        write(g, "intro.md", "root-version (should be ignored)");
        let files = default_locale_files(g, "en");
        let intro = files.iter().find(|s| s.filename == "intro.md").unwrap();
        assert_eq!(intro.source_path, g.join("items/en/intro.md"));
    }

    #[test]
    fn root_conclusion_is_also_appended() {
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write(g, "items/en/howto.md", "HT");
        write(g, "conclusion.md", "Thanks for reading.");
        let files = default_locale_files(g, "en");
        let conclusion = files
            .iter()
            .find(|s| s.filename == "conclusion.md")
            .expect("conclusion.md should be present");
        assert_eq!(conclusion.source_path, g.join("conclusion.md"));
    }

    #[test]
    fn no_items_dir_falls_back_to_just_root_intro_conclusion() {
        // Defensive: a guide with NO items/en/ dir at all but root-
        // level intro.md and conclusion.md should still surface them.
        // (This shape is rare today but matches Node's behavior of
        // checking root regardless of whether items/<default>/ exists.)
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write(g, "intro.md", "Welcome");
        write(g, "conclusion.md", "Bye");
        let files = default_locale_files(g, "en");
        let mut names: Vec<&str> = files.iter().map(|s| s.filename.as_str()).collect();
        names.sort();
        assert_eq!(names, vec!["conclusion.md", "intro.md"]);
    }

    #[test]
    fn non_md_files_are_ignored() {
        let tmp = tempfile::tempdir().unwrap();
        let g = tmp.path();
        write(g, "items/en/a.md", "A");
        write(g, "items/en/img.png", "binary");
        write(g, "items/en/README", "plain");
        let files = default_locale_files(g, "en");
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].filename, "a.md");
    }
}

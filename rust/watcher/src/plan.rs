//! Map changed paths → minimal set of sitegen invocations.
//!
//! Before this module, watcher rebuilt every guide × every locale on
//! any non-static change — a 420-page build for a single fr_fr file
//! edit. Mirrors `src/watch.js:74-156` which only rebuilds the affected
//! guide; we go one step further and also restrict to the affected
//! locale when the change is under `items/<locale>/`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

use fcdocs_shared::guides::is_valid_id;

/// One concrete sitegen invocation. The watcher emits at most one of
/// each kind per batch.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    /// `sitegen build-static` — file copies from `src/static/{css,images,js,csv}`.
    BuildStatic,
    /// `sitegen build` with optional --guide / --locale filters. `None`
    /// for either field means "no filter" (build everything).
    Build {
        guides: Option<Vec<String>>,
        locales: Option<Vec<String>>,
    },
}

impl Action {
    pub fn to_args(&self) -> Vec<String> {
        match self {
            Action::BuildStatic => vec!["build-static".to_string()],
            Action::Build { guides, locales } => {
                let mut args = vec!["build".to_string()];
                if let Some(g) = guides {
                    if !g.is_empty() {
                        args.push(format!("--guide={}", g.join(",")));
                    }
                }
                if let Some(l) = locales {
                    if !l.is_empty() {
                        args.push(format!("--locale={}", l.join(",")));
                    }
                }
                args
            }
        }
    }
}

/// Result of classifying one debounced batch. Empty `actions` means
/// nothing to do (the batch was filtered noise).
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Plan {
    pub actions: Vec<Action>,
}

/// Classify a batch of changed paths against `repo_root`. Paths
/// outside the repo are ignored. The classification is pure — it
/// reads no filesystem state — which keeps the watcher snappy and the
/// behavior trivially testable.
pub fn classify(repo_root: &Path, paths: &[PathBuf]) -> Plan {
    let mut needs_static = false;
    let mut needs_full = false;
    // Per-guide locale set; `None` means "all locales".
    let mut guides: BTreeMap<String, Option<BTreeSet<String>>> = BTreeMap::new();

    for path in paths {
        let rel = match path.strip_prefix(repo_root) {
            Ok(r) => r,
            Err(_) => continue,
        };
        let segs: Vec<&str> = rel
            .components()
            .filter_map(|c| match c {
                Component::Normal(os) => os.to_str(),
                _ => None,
            })
            .collect();
        if segs.is_empty() {
            continue;
        }
        match segs.as_slice() {
            // src/static/{css,images,js,csv}/** → just copy files.
            ["src", "static", sub, ..] if matches!(*sub, "css" | "images" | "js" | "csv") => {
                needs_static = true;
            }
            // src/static/generated/** is the OUTPUT of sitegen — never
            // trigger a rebuild from our own writes.
            ["src", "static", "generated", ..] => {}
            // Templates, translations, locales, snippets, sdk config —
            // anything that cuts across every guide/locale → full.
            ["src", "templates", ..] => needs_full = true,
            ["src", "translations.json"] => needs_full = true,
            ["src", "locales.json"] => needs_full = true,
            ["src", "snippets", ..] => needs_full = true,
            ["src", "content", "sdk-repos.json"] => needs_full = true,
            // src/content/guides/<id>/items/<locale>/<file>
            ["src", "content", "guides", guide, "items", locale, ..]
                if is_valid_id(guide) && is_valid_id(locale) =>
            {
                add_guide(&mut guides, guide, Some(locale.to_string()));
            }
            // src/content/guides/<id>/items/<file>            (root-level, default locale)
            // src/content/guides/<id>/meta.json, intro.md, conclusion.md, index.md.html
            // src/content/guides/<id>/anything-else
            ["src", "content", "guides", guide, ..] if is_valid_id(guide) => {
                add_guide(&mut guides, guide, None);
            }
            // Anything else under src/ we don't recognize → fall back
            // to a full rebuild rather than silently swallowing the
            // event. Worst-case: a no-op file like a CHANGELOG edit
            // triggers a full build, which is the existing behavior.
            ["src", ..] => needs_full = true,
            // Top-level files (package.json, build.sh, README.md, etc.)
            // are not content the site embeds. Ignore.
            _ => {}
        }
    }

    let mut actions = Vec::new();
    if needs_static {
        actions.push(Action::BuildStatic);
    }
    if needs_full {
        // Full rebuild supersedes any per-guide collected entries.
        actions.push(Action::Build {
            guides: None,
            locales: None,
        });
    } else if !guides.is_empty() {
        // If any guide demanded "all locales", we can't restrict --locale
        // for the combined invocation (sitegen takes a single locale
        // set across all selected guides). Otherwise union the per-guide
        // sets into one --locale list.
        let any_all_locales = guides.values().any(|v| v.is_none());
        let locales: Option<Vec<String>> = if any_all_locales {
            None
        } else {
            let mut union: BTreeSet<String> = BTreeSet::new();
            for v in guides.values() {
                if let Some(s) = v {
                    union.extend(s.iter().cloned());
                }
            }
            Some(union.into_iter().collect())
        };
        let guide_ids: Vec<String> = guides.keys().cloned().collect();
        actions.push(Action::Build {
            guides: Some(guide_ids),
            locales,
        });
    }
    Plan { actions }
}

fn add_guide(
    guides: &mut BTreeMap<String, Option<BTreeSet<String>>>,
    guide: &str,
    locale: Option<String>,
) {
    let entry = guides.entry(guide.to_string()).or_insert(Some(BTreeSet::new()));
    match (entry, locale) {
        // Already promoted to "all locales" — anything new is subsumed.
        (None, _) => {}
        (slot @ Some(_), None) => {
            // A guide-level change (meta.json, intro, etc.) trumps
            // any per-locale entries we'd already accumulated. We
            // have to rebuild every locale to keep them consistent.
            *slot = None;
        }
        (Some(set), Some(loc)) => {
            set.insert(loc);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(repo: &Path, rel: &str) -> PathBuf {
        repo.join(rel)
    }

    fn build_action(guides: Option<&[&str]>, locales: Option<&[&str]>) -> Action {
        Action::Build {
            guides: guides.map(|g| g.iter().map(|s| s.to_string()).collect()),
            locales: locales.map(|l| l.iter().map(|s| s.to_string()).collect()),
        }
    }

    #[test]
    fn locale_specific_item_builds_only_that_guide_and_locale() {
        let repo = Path::new("/tmp/repo");
        let plan = classify(repo, &[p(repo, "src/content/guides/acme/items/fr_fr/intro.md")]);
        assert_eq!(
            plan.actions,
            vec![build_action(Some(&["acme"]), Some(&["fr_fr"]))]
        );
    }

    #[test]
    fn default_locale_item_builds_all_locales_of_that_guide() {
        // The default-locale source is the fallback for every other
        // locale, so editing it affects every locale's render.
        let repo = Path::new("/tmp/repo");
        let plan = classify(repo, &[p(repo, "src/content/guides/acme/items/intro.md")]);
        assert_eq!(plan.actions, vec![build_action(Some(&["acme"]), None)]);
    }

    #[test]
    fn meta_json_builds_all_locales_of_that_guide() {
        let repo = Path::new("/tmp/repo");
        let plan = classify(repo, &[p(repo, "src/content/guides/acme/meta.json")]);
        assert_eq!(plan.actions, vec![build_action(Some(&["acme"]), None)]);
    }

    #[test]
    fn template_change_is_full_rebuild() {
        let repo = Path::new("/tmp/repo");
        let plan = classify(repo, &[p(repo, "src/templates/page.html")]);
        assert_eq!(plan.actions, vec![build_action(None, None)]);
    }

    #[test]
    fn translations_or_locales_or_snippets_or_sdk_repos_force_full() {
        let repo = Path::new("/tmp/repo");
        for trigger in [
            "src/translations.json",
            "src/locales.json",
            "src/snippets/code-foo.html",
            "src/content/sdk-repos.json",
        ] {
            let plan = classify(repo, &[p(repo, trigger)]);
            assert_eq!(
                plan.actions,
                vec![build_action(None, None)],
                "trigger {trigger} should force full"
            );
        }
    }

    #[test]
    fn static_only_change_runs_build_static() {
        let repo = Path::new("/tmp/repo");
        let plan = classify(repo, &[p(repo, "src/static/css/main.css")]);
        assert_eq!(plan.actions, vec![Action::BuildStatic]);
    }

    #[test]
    fn static_and_content_batch_emits_both() {
        let repo = Path::new("/tmp/repo");
        let plan = classify(
            repo,
            &[
                p(repo, "src/static/css/main.css"),
                p(repo, "src/content/guides/acme/items/fr_fr/intro.md"),
            ],
        );
        assert_eq!(
            plan.actions,
            vec![
                Action::BuildStatic,
                build_action(Some(&["acme"]), Some(&["fr_fr"])),
            ]
        );
    }

    #[test]
    fn multiple_locale_items_same_guide_unions_locales() {
        let repo = Path::new("/tmp/repo");
        let plan = classify(
            repo,
            &[
                p(repo, "src/content/guides/acme/items/fr_fr/intro.md"),
                p(repo, "src/content/guides/acme/items/de_de/intro.md"),
            ],
        );
        assert_eq!(
            plan.actions,
            vec![build_action(Some(&["acme"]), Some(&["de_de", "fr_fr"]))]
        );
    }

    #[test]
    fn locale_item_then_root_item_promotes_to_all_locales() {
        let repo = Path::new("/tmp/repo");
        let plan = classify(
            repo,
            &[
                p(repo, "src/content/guides/acme/items/fr_fr/intro.md"),
                // Default-locale root → bumps acme to "all locales".
                p(repo, "src/content/guides/acme/items/intro.md"),
            ],
        );
        assert_eq!(plan.actions, vec![build_action(Some(&["acme"]), None)]);
    }

    #[test]
    fn mixed_guides_one_demanding_all_drops_locale_filter() {
        // acme: one fr_fr item. beta: meta.json (all locales).
        // We can only emit one --locale flag across the spawn, so we
        // have to fall back to "all locales" when ANY guide demands it.
        let repo = Path::new("/tmp/repo");
        let plan = classify(
            repo,
            &[
                p(repo, "src/content/guides/acme/items/fr_fr/intro.md"),
                p(repo, "src/content/guides/beta/meta.json"),
            ],
        );
        assert_eq!(
            plan.actions,
            vec![build_action(Some(&["acme", "beta"]), None)]
        );
    }

    #[test]
    fn generated_dir_writes_are_ignored() {
        // sitegen writes to src/static/generated/**. Watcher must
        // never react to its own output or we'd loop forever.
        let repo = Path::new("/tmp/repo");
        let plan = classify(
            repo,
            &[p(repo, "src/static/generated/guide-acme.html")],
        );
        assert_eq!(plan, Plan::default());
    }

    #[test]
    fn paths_outside_repo_are_ignored() {
        let repo = Path::new("/tmp/repo");
        let plan = classify(repo, &[PathBuf::from("/etc/passwd")]);
        assert_eq!(plan, Plan::default());
    }

    #[test]
    fn invalid_guide_id_falls_back_to_full() {
        // Defense-in-depth: any segment that wouldn't pass is_valid_id
        // becomes a full rebuild rather than being passed through to
        // sitegen as a CLI arg. The unknown-src case picks this up.
        let repo = Path::new("/tmp/repo");
        let plan = classify(
            repo,
            &[p(repo, "src/content/guides/..%2Fevil/meta.json")],
        );
        assert_eq!(plan.actions, vec![build_action(None, None)]);
    }

    #[test]
    fn to_args_for_each_variant() {
        assert_eq!(
            Action::BuildStatic.to_args(),
            vec!["build-static".to_string()]
        );
        assert_eq!(
            build_action(None, None).to_args(),
            vec!["build".to_string()]
        );
        assert_eq!(
            build_action(Some(&["acme"]), Some(&["fr_fr", "de_de"])).to_args(),
            vec![
                "build".to_string(),
                "--guide=acme".to_string(),
                "--locale=fr_fr,de_de".to_string(),
            ]
        );
    }
}

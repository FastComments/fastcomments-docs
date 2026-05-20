//! Port of `src/sdk-checkout-manager.js`. Uses `git` via `Command` rather
//! than libgit2 because the existing behavior is simple (shallow clone,
//! fetch + reset) and matches the Node implementation's commands exactly.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};
use tracing::{info, warn};

use crate::config::SdkConfig;

pub struct CheckoutManager {
    pub checkout_dir: PathBuf,
}

pub struct Checkout {
    pub sdk: SdkConfig,
    pub path: PathBuf,
}

impl CheckoutManager {
    pub fn new(checkout_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&checkout_dir)
            .with_context(|| format!("create checkout dir {checkout_dir:?}"))?;
        Ok(Self { checkout_dir })
    }

    pub fn checkout_all(&self, sdks: &[SdkConfig]) -> Vec<Checkout> {
        let mut out = Vec::with_capacity(sdks.len());
        for sdk in sdks {
            let target = self.checkout_dir.join(&sdk.id);
            match self.ensure_checkout(sdk, &target) {
                Ok(()) => out.push(Checkout {
                    sdk: sdk.clone(),
                    path: target,
                }),
                Err(e) => {
                    warn!(sdk = %sdk.id, error = %e, "skip SDK (checkout failed)");
                }
            }
        }
        out
    }

    fn ensure_checkout(&self, sdk: &SdkConfig, target: &Path) -> Result<()> {
        if target.join(".git").exists() {
            // `SDKGEN_NO_FETCH=1` lets dev iterate offline / test the
            // build with locally-mutated SDK trees (useful for
            // simulating "method missing in SDK" scenarios end-to-end).
            // It does NOT skip the checkout existence check.
            if std::env::var("SDKGEN_NO_FETCH").map(|v| v == "1").unwrap_or(false) {
                info!(sdk = %sdk.id, "SDKGEN_NO_FETCH=1; skipping fetch+reset");
                return Ok(());
            }

            // Detect URL drift before fetching. Node's
            // sdk-checkout-manager.js:212-220 trips `needsUpdate`
            // when the remote hash differs and then `fs.rmSync`s the
            // dir before cloning fresh. If sdk-repos.json changes a
            // URL but we keep fetching the baked-in `origin`, we
            // silently track the wrong repo.
            match current_remote_url(target) {
                Ok(current) if !urls_equivalent(&current, &sdk.repo) => {
                    info!(
                        sdk = %sdk.id,
                        old = %current,
                        new = %sdk.repo,
                        "remote URL changed; removing checkout and re-cloning"
                    );
                    std::fs::remove_dir_all(target)
                        .with_context(|| format!("remove stale checkout {target:?}"))?;
                    // fall through to clone branch below
                }
                Ok(_) => {
                    info!(sdk = %sdk.id, "updating existing checkout");
                    run_git(target, &["fetch", "origin", &sdk.branch])
                        .with_context(|| format!("git fetch {} in {target:?}", sdk.branch))?;
                    run_git(
                        target,
                        &["reset", "--hard", &format!("origin/{}", sdk.branch)],
                    )
                    .with_context(|| {
                        format!("git reset --hard origin/{} in {target:?}", sdk.branch)
                    })?;
                    return Ok(());
                }
                Err(e) => {
                    warn!(
                        sdk = %sdk.id,
                        error = %e,
                        "could not read remote.origin.url; treating checkout as stale and re-cloning"
                    );
                    std::fs::remove_dir_all(target)
                        .with_context(|| format!("remove broken checkout {target:?}"))?;
                    // fall through to clone branch below
                }
            }
        }

        info!(sdk = %sdk.id, repo = %sdk.repo, "cloning");
        // Mirrors src/sdk-checkout-manager.js:129 (`git clone --depth 1 --branch BRANCH REPO TARGET`).
        run_git(
            &self.checkout_dir,
            &[
                "clone",
                "--depth",
                "1",
                "--branch",
                &sdk.branch,
                &sdk.repo,
                &target.to_string_lossy(),
            ],
        )
        .with_context(|| format!("git clone {} -> {target:?}", sdk.repo))?;
        Ok(())
    }
}

/// Read `git config --get remote.origin.url` from an existing checkout.
fn current_remote_url(target: &Path) -> Result<String> {
    let out = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .current_dir(target)
        .output()
        .with_context(|| format!("spawn git config in {target:?}"))?;
    if !out.status.success() {
        anyhow::bail!(
            "git config --get remote.origin.url exited with {}",
            out.status
        );
    }
    Ok(String::from_utf8(out.stdout)
        .context("remote.origin.url not utf-8")?
        .trim()
        .to_string())
}

/// Compare two git remote URLs for equivalence. Tolerates the only
/// shape git itself normalizes silently: an optional trailing `.git`
/// suffix. Everything else (scheme, host, path case) is treated as a
/// meaningful change and triggers a reclone.
fn urls_equivalent(a: &str, b: &str) -> bool {
    fn norm(s: &str) -> &str {
        let s = s.trim();
        s.strip_suffix(".git").unwrap_or(s)
    }
    norm(a) == norm(b)
}

fn run_git(cwd: &Path, args: &[&str]) -> Result<()> {
    let status = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .status()
        .with_context(|| format!("spawn git {args:?}"))?;
    if !status.success() {
        anyhow::bail!("git {args:?} exited with status {status}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_silent(cwd: &Path, args: &[&str]) {
        let out = Command::new("git")
            .args(args)
            .current_dir(cwd)
            .output()
            .expect("spawn git");
        assert!(
            out.status.success(),
            "git {args:?} in {cwd:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    /// Build a local bare git repo containing a single file with the
    /// given contents on `main`. Returns the bare-repo path (suitable
    /// as a `repo` URL for `git clone`).
    fn make_bare_repo(root: &Path, name: &str, file_name: &str, file_body: &str) -> PathBuf {
        let work = root.join(format!("{name}-work"));
        let bare = root.join(format!("{name}.git"));
        std::fs::create_dir_all(&work).unwrap();
        run_silent(&work, &["init", "--initial-branch=main", "-q"]);
        run_silent(&work, &["config", "user.email", "t@example.com"]);
        run_silent(&work, &["config", "user.name", "t"]);
        run_silent(&work, &["config", "commit.gpgsign", "false"]);
        std::fs::write(work.join(file_name), file_body).unwrap();
        run_silent(&work, &["add", "."]);
        run_silent(&work, &["commit", "-m", "init", "-q"]);
        run_silent(
            root,
            &[
                "clone",
                "--bare",
                "-q",
                work.to_str().unwrap(),
                bare.to_str().unwrap(),
            ],
        );
        bare
    }

    fn config(id: &str, repo: PathBuf) -> SdkConfig {
        SdkConfig {
            id: id.to_string(),
            name: id.to_string(),
            page_header: None,
            repo: repo.to_string_lossy().into_owned(),
            branch: "main".to_string(),
            language: None,
            icon: None,
            doc_generator: None,
            doc_generators: None,
            description: None,
            extra: None,
        }
    }

    /// Regression test: pre-fix, ensure_checkout ran `git fetch
    /// origin` against the baked-in remote URL even after the
    /// configured URL had changed. Node's sdk-checkout-manager.js
    /// `fs.rmSync`s and re-clones whenever needsUpdate trips, so the
    /// new URL takes effect on the next build. Rust must do the
    /// same.
    #[test]
    fn reclones_when_sdk_repo_url_changes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repos = tmp.path().join("repos");
        std::fs::create_dir_all(&repos).unwrap();

        let bare_old = make_bare_repo(&repos, "old", "marker.txt", "OLD-REPO");
        let bare_new = make_bare_repo(&repos, "new", "marker.txt", "NEW-REPO");

        let checkout_root = tmp.path().join("checkouts");
        let manager = CheckoutManager::new(checkout_root.clone()).expect("manager");

        // First pass: clone the OLD repo.
        let sdk = config("acme", bare_old.clone());
        let target = checkout_root.join("acme");
        manager.ensure_checkout(&sdk, &target).expect("first clone");
        assert_eq!(
            std::fs::read_to_string(target.join("marker.txt")).unwrap(),
            "OLD-REPO"
        );
        assert!(
            urls_equivalent(&current_remote_url(&target).unwrap(), &sdk.repo),
            "remote url after first clone"
        );

        // Now sdk-repos.json swaps the URL to the NEW repo. Pre-fix
        // this just ran `git fetch origin` against the OLD remote
        // and left `marker.txt` reading OLD-REPO. The fix detects
        // the URL mismatch, wipes the dir, and re-clones.
        let sdk = config("acme", bare_new.clone());
        manager.ensure_checkout(&sdk, &target).expect("second pass after URL change");
        assert_eq!(
            std::fs::read_to_string(target.join("marker.txt")).unwrap(),
            "NEW-REPO",
            "ensure_checkout must reclone when sdk.repo URL changes"
        );
        assert!(
            urls_equivalent(&current_remote_url(&target).unwrap(), &sdk.repo),
            "remote url must point at the NEW repo after reclone"
        );
    }

    #[test]
    fn matching_url_takes_fetch_path_and_keeps_dir() {
        // Sanity check: same URL across calls keeps the existing
        // .git intact (no reclone, just fetch+reset). Asserted via
        // inode-stable file path on .git itself.
        let tmp = tempfile::tempdir().expect("tempdir");
        let repos = tmp.path().join("repos");
        std::fs::create_dir_all(&repos).unwrap();
        let bare = make_bare_repo(&repos, "only", "marker.txt", "ONLY");

        let checkout_root = tmp.path().join("checkouts");
        let manager = CheckoutManager::new(checkout_root.clone()).expect("manager");
        let sdk = config("acme", bare);
        let target = checkout_root.join("acme");

        manager.ensure_checkout(&sdk, &target).expect("first");
        let head_path = target.join(".git").join("HEAD");
        let head_meta_first = std::fs::metadata(&head_path).unwrap();
        let ctime_first = head_meta_first
            .created()
            .or_else(|_| head_meta_first.modified())
            .unwrap();

        manager.ensure_checkout(&sdk, &target).expect("second");
        let head_meta_second = std::fs::metadata(&head_path).unwrap();
        let ctime_second = head_meta_second
            .created()
            .or_else(|_| head_meta_second.modified())
            .unwrap();
        assert_eq!(
            ctime_first, ctime_second,
            "matching URL must not reclone (HEAD created/modified should be unchanged)"
        );
    }

    #[test]
    fn urls_equivalent_tolerates_trailing_dot_git() {
        assert!(urls_equivalent(
            "https://github.com/x/y",
            "https://github.com/x/y.git"
        ));
        assert!(urls_equivalent(
            "https://github.com/x/y.git",
            "https://github.com/x/y"
        ));
        assert!(!urls_equivalent(
            "https://github.com/x/y",
            "https://github.com/x/z"
        ));
        assert!(!urls_equivalent(
            "https://github.com/x/y",
            "git@github.com:x/y"
        ));
    }
}

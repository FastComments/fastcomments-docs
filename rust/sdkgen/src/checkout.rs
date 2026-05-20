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
            info!(sdk = %sdk.id, "updating existing checkout");
            run_git(target, &["fetch", "origin", &sdk.branch])
                .with_context(|| format!("git fetch {} in {target:?}", sdk.branch))?;
            run_git(target, &["reset", "--hard", &format!("origin/{}", sdk.branch)])
                .with_context(|| format!("git reset --hard origin/{} in {target:?}", sdk.branch))?;
        } else {
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
        }
        Ok(())
    }
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

//! Replaces `src/app.js` + `src/custom-styling-guide-generator.js` +
//! `build-static.sh`. Subcommands: `build`, `custom-styling`,
//! `build-static`.
//!
//! The actual work lives in submodules; this file is just the CLI router.

mod build;
mod custom_styling;
mod sidecar_supervisor;
mod static_build;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // chromiumoxide's CDP connection logs a lot of harmless
                // serde errors when paired with older Chromium builds
                // (the puppeteer-bundled 848005 build is from late 2020
                // and lacks newer CDP message fields). Suppress unless
                // RUST_LOG explicitly raises it.
                tracing_subscriber::EnvFilter::new(
                    "info,chromiumoxide::conn=off,chromiumoxide::handler=off",
                )
            }),
        )
        .init();

    let cmd = std::env::args().nth(1).unwrap_or_else(|| "build".to_string());
    match cmd.as_str() {
        "build" => build::run(std::env::args().skip(2).collect()).await,
        "custom-styling" => custom_styling::run().await,
        "build-static" => static_build::run().await,
        other => anyhow::bail!(
            "unknown subcommand: {other}\nusage: sitegen [build|custom-styling|build-static]"
        ),
    }
}

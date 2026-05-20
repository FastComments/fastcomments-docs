//! Replaces `src/app.js` + `src/custom-styling-guide-generator.js` +
//! `build-static.sh`. Subcommands: `build`, `custom-styling`, `build-static`.
//!
//! Stub. Real impl lands as part of task #17.

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cmd = std::env::args().nth(1).unwrap_or_else(|| "build".to_string());
    match cmd.as_str() {
        "build" => tracing::info!("sitegen build — not yet implemented"),
        "custom-styling" => tracing::info!("sitegen custom-styling — not yet implemented"),
        "build-static" => tracing::info!("sitegen build-static — not yet implemented"),
        other => {
            anyhow::bail!("unknown subcommand: {other}");
        }
    }
    Ok(())
}

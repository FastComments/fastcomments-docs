//! Replaces `src/sdk-guide-generator.js` + `src/sdk-doc-generators/*`.
//! Stub. Real impl lands as part of task #20.

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
    tracing::info!("sdkgen — not yet implemented");
    Ok(())
}

//! Replaces `src/check-translations.js` + `src/translate-with-gpt.js`.
//! Subcommands: `check`, `run`, `cleanup`. Stub. Real impl: task #21.

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
    let cmd = std::env::args().nth(1).unwrap_or_else(|| "check".to_string());
    match cmd.as_str() {
        "check" => tracing::info!("trans check — not yet implemented"),
        "run" => tracing::info!("trans run — not yet implemented"),
        "cleanup" => tracing::info!("trans cleanup — not yet implemented"),
        other => anyhow::bail!("unknown subcommand: {other}"),
    }
    Ok(())
}

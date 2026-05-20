//! Replaces `src/check-translations.js` + `src/translate-with-gpt.js` +
//! `src/translation-snapshot.js` + cleanup utilities.
//!
//! Current scope: `trans check` and `trans cleanup` fully ported.
//! `trans run` (the LLM-driven actual-translation pipeline, 1259 LOC of
//! Node code with custom prompts + model fallback + git commit) is a
//! framework stub that calls into the shared `fcdocs-llm` crate (already
//! verified for OpenAI cache key parity) but the prompt set + per-batch
//! orchestration is a TODO follow-up. The Node script remains the
//! authority for actually generating new translations until that lands.

mod check;
mod cleanup;
mod run;
mod snapshot;

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
        "check" => check::run().await,
        "cleanup" => cleanup::run().await,
        "run" => run::run().await,
        other => anyhow::bail!("unknown subcommand: {other}"),
    }
}

//! Replaces `src/check-translations.js` + `src/translate-with-gpt.js` +
//! `src/translation-snapshot.js` + cleanup utilities.
//!
//! Subcommands:
//!   `check`   - audits markdown items, UI strings, meta.json, and
//!               inline-code parity. Exits non-zero on any gap so
//!               build.sh branches into `run`.
//!   `run`     - translates everything `check` would flag. Three
//!               phases in order: markdown items, UI strings,
//!               meta.json. All three share the same on-disk caches
//!               and prompt shapes as Node's translate-with-gpt.js.
//!   `cleanup` - empties stale translation files matching Node's
//!               cleanup-empty-translations.js + cleanup-empty-generated.js.

mod check;
mod cleanup;
mod discover;
mod meta_json;
mod openai;
mod run;
mod snapshot;
mod ui;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    fcdocs_shared::repo::init_tracing();

    let mut args = std::env::args().skip(1);
    let cmd = args.next().unwrap_or_else(|| "check".to_string());
    match cmd.as_str() {
        "check" => check::run().await,
        "cleanup" => {
            let opts = cleanup::parse_options(args)?;
            cleanup::run_with(opts).await
        }
        "run" => {
            let opts = run::parse_options(args)?;
            run::run_with(opts).await
        }
        other => anyhow::bail!("unknown subcommand: {other}"),
    }
}

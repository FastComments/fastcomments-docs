//! Replaces `src/check-translations.js` + `src/the legacy Node translator` +
//! `src/translation-snapshot.js` + cleanup utilities.
//!
//! Subcommands:
//!   `check`   - audits markdown items, UI strings, meta.json, and
//!               inline-code parity. Exits non-zero on any gap so
//!               build.sh branches into `run`.
//!   `run`     - translates everything `check` would flag. Three
//!               phases in order: markdown items, UI strings,
//!               meta.json. All three share the same on-disk caches
//!               and prompt shapes as Node's the legacy Node translator.
//!   `cleanup` - empties stale translation files matching Node's
//!               cleanup-empty-translations.js + cleanup-empty-generated.js.
//!   `validate-images`
//!             - build gate: every translated item must reference the
//!               exact same images as its default-locale source.
//!               Exits non-zero on any mismatch.
//!   `validate-links`
//!             - build gate: every translated item must link exactly
//!               where its default-locale source links. URLs are
//!               technical identifiers, and a translator that alters
//!               one ships a 404. Exits non-zero on any mismatch.

mod check;
mod cleanup;
mod discover;
mod images;
mod meta_json;
mod json_translator;
mod links;
mod llm_client;
mod run;
mod snapshot;
mod ui;
mod validate;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    fcdocs_shared::repo::init_tracing();

    let mut args = std::env::args().skip(1);
    let cmd = args.next().unwrap_or_else(|| "check".to_string());
    match cmd.as_str() {
        "check" => check::run().await,
        "validate-images" => validate::run_with(validate::Gate::Images, args).await,
        "validate-links" => validate::run_with(validate::Gate::Links, args).await,
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

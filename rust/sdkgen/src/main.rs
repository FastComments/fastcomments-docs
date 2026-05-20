//! Replaces `src/sdk-guide-generator.js` + `src/sdk-checkout-manager.js`
//! + `src/sdk-doc-generators/*`.
//!
//! Current scope: full README generator, full checkout manager, full
//! meta.json generation. OpenAPI and the four AI generators (typescript,
//! rust, cpp, nim) compile-link through the framework but their content
//! emission is stubbed — they write a single placeholder section and log
//! a warning. The framework calls them in the correct chain so once they
//! land they just slot in.

mod checkout;
mod config;
mod generators;
mod guide;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Optional `--sdk <id>` filter, mirroring src/sdk-guide-generator.js:319-320.
    let sdk_filter = parse_sdk_filter(std::env::args().skip(1));
    guide::generate_all(sdk_filter.as_deref()).await
}

fn parse_sdk_filter(args: impl Iterator<Item = String>) -> Option<String> {
    let mut iter = args.peekable();
    while let Some(arg) = iter.next() {
        if arg == "--sdk" {
            return iter.next();
        }
        if let Some(rest) = arg.strip_prefix("--sdk=") {
            return Some(rest.to_string());
        }
    }
    None
}

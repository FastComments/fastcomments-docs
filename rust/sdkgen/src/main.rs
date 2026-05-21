//! Replaces `src/sdk-guide-generator.js` + `src/sdk-checkout-manager.js`
//! + `src/sdk-doc-generators/*`. Production build pipeline
//! (`build.sh`) calls this binary directly; the Node script is kept
//! in the tree only as a parity reference during the transition.
//!
//! Current scope:
//!   - Full checkout manager (git clone/fetch/reset, with
//!     SDKGEN_NO_FETCH=1 escape hatch for offline iteration).
//!   - Full README generator.
//!   - Full OpenAPI generator with fail-fast validation (missing
//!     methods and unextractable return types collect per-SDK and
//!     bail the build at end-of-run; matches Node's
//!     src/sdk-guide-generator.js:268-309 contract).
//!   - Four AI generators (typescript, rust, cpp, nim) parallelized
//!     across methods, sharing the `fcdocs-llm` cache (byte-identical
//!     SHA256 keys vs Node so existing src/sdk-ai-cache/ entries hit).
//!   - meta.json emission with Node-shaped category ordering.
//!
//! SDK-level fan-out is parallel up to SDKGEN_PARALLEL (default 8).
//! `--sdk <id>` restricts to one SDK for dev iteration.

mod checkout;
mod config;
mod generators;
mod guide;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    fcdocs_shared::repo::init_tracing();

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

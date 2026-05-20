//! Content pipeline. Two flavors:
//!
//! - `indexer` (existing, hot path for `fcdocs-indexer`): optimized for
//!   extracting indexable text — skips screenshot generation, omits real
//!   syntax highlighting since `html-to-text` would strip it anyway.
//! - `full` (new, for `fcdocs-sitegen`): mirrors the full Node content
//!   build pipeline — invokes browser pool for screenshots, sidecar
//!   `/highlight` for code blocks, creates code-snippet pages on disk.
//!
//! Both share the same upstream stages (handlebars substitution, marker
//! tokenization, snippet processing) — diverging at where side effects
//! are emitted.

pub mod indexer;
pub mod full;

// Re-export the indexer API so existing callers (rust/indexer) work
// unchanged after the directory move.
pub use indexer::{process_markdown, ProcessedItem, EXAMPLE_TENANT_ID};

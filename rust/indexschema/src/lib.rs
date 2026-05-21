//! Tantivy schema shared by the indexer (writer) and the server
//! (reader). They MUST agree byte-for-byte on field names, options,
//! and field order — drift would mean the server can't read what
//! the indexer just wrote.
//!
//! Lifted out of each binary so the schema lives in exactly one
//! place. Previously both `rust/indexer/src/main.rs` and the
//! server's test fixture carried near-identical copies that drifted
//! independently.

use tantivy::schema::{
    IndexRecordOption, Schema, SchemaBuilder, TextFieldIndexing, TextOptions, STORED, STRING,
};
use tantivy::tokenizer::{
    AsciiFoldingFilter, Language, LowerCaser, RemoveLongFilter, SimpleTokenizer, Stemmer,
    TextAnalyzer,
};
use tantivy::Index;

/// Name the schema's `title` / `search_text` / etc. text fields use
/// for their tokenizer. The indexer registers an analyzer under
/// THIS name; the server's QueryParser resolves the same name out
/// of the index's tokenizer manager. Drift between the two
/// (analyzer/schema naming) reintroduces the analyzer-mismatch
/// class of bugs that brought us here.
pub const DOCS_TEXT_TOKENIZER: &str = "docs_text";

/// Build the search-index Schema. Field order matches what
/// `src/build-search-index-worker.js:62-72` registers via SQLite FTS5
/// (`doc_id, title, parent_title, url, parent_url, icon,
/// search_text`).
pub fn build_schema() -> Schema {
    let mut b: SchemaBuilder = Schema::builder();
    b.add_text_field("doc_id", STRING | STORED);
    let text_idx = TextFieldIndexing::default()
        .set_tokenizer(DOCS_TEXT_TOKENIZER)
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let text_opts = TextOptions::default()
        .set_indexing_options(text_idx)
        .set_stored();
    b.add_text_field("title", text_opts.clone());
    b.add_text_field("parent_title", text_opts.clone());
    b.add_text_field("url", text_opts.clone());
    b.add_text_field("parent_url", text_opts.clone());
    b.add_text_field("icon", STRING | STORED);
    b.add_text_field("search_text", text_opts);
    b.build()
}

/// Build the `docs_text` TextAnalyzer chain. Identical analyzer
/// must run on the indexer side (over every term being stored)
/// AND on the server side (over every query term) or queries
/// tokenize one way and indexed terms another — the M10/M11/M13
/// drift surface the indexschema crate exists to eliminate.
///
/// Chain (in order):
///   - `SimpleTokenizer`            — whitespace + ASCII punctuation
///                                    split; matches SQLite FTS5
///                                    `unicode61` behavior on the
///                                    ASCII fast path.
///   - `RemoveLongFilter(40)`       — bound term length so a
///                                    base64 blob can't blow up
///                                    the index.
///   - `LowerCaser`                 — case-insensitive matching;
///                                    Node's `LOWER(...)` analog.
///   - `AsciiFoldingFilter`         — strip diacritics (café -> cafe)
///                                    so French/German users
///                                    searching ASCII still hit
///                                    accented terms. Mirrors
///                                    `unicode61`'s default
///                                    `remove_diacritics=1`.
///   - `Stemmer(English)`           — Porter stemming, applied to
///                                    EVERY locale (matches Node
///                                    `tokenize='porter unicode61'`
///                                    at build-search-index-worker.js:70).
///                                    `_locale` is reserved for the
///                                    day we switch per-locale
///                                    stemmers; today every locale
///                                    routes here.
///
/// Folding MUST go before stemming because Porter is ASCII-only
/// and would silently no-op on already-diacritic'd input.
pub fn build_docs_text_analyzer(_locale: &str) -> TextAnalyzer {
    TextAnalyzer::builder(SimpleTokenizer::default())
        .filter(RemoveLongFilter::limit(40))
        .filter(LowerCaser)
        .filter(AsciiFoldingFilter)
        .filter(Stemmer::new(Language::English))
        .build()
}

/// Register the [`DOCS_TEXT_TOKENIZER`] analyzer on `index`.
/// Convenience wrapper so the indexer's per-locale registration
/// loop stays a one-liner.
pub fn register_docs_text_analyzer(index: &Index, locale: &str) {
    index
        .tokenizers()
        .register(DOCS_TEXT_TOKENIZER, build_docs_text_analyzer(locale));
}

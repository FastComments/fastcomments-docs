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

/// Build the search-index Schema. Field order matches what
/// `src/build-search-index-worker.js:62-72` registers via SQLite FTS5
/// (`doc_id, title, parent_title, url, parent_url, icon,
/// search_text`).
pub fn build_schema() -> Schema {
    let mut b: SchemaBuilder = Schema::builder();
    b.add_text_field("doc_id", STRING | STORED);
    let text_idx = TextFieldIndexing::default()
        .set_tokenizer("docs_text")
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

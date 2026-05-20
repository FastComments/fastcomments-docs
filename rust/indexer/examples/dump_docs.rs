//! Dump every doc in a Tantivy index as JSONL on stdout.
//!
//! Used by scripts/compare-indexes.py to diff content against the legacy
//! SQLite FTS5 indexes during the migration verification phase.
//!
//! Usage: dump_docs <index_dir>

use std::env;
use std::io::{self, BufWriter, Write};

use anyhow::Result;
use tantivy::schema::Value;
use tantivy::{collector::DocSetCollector, query::AllQuery, Index, TantivyDocument};

fn main() -> Result<()> {
    let dir = env::args()
        .nth(1)
        .expect("usage: dump_docs <index_dir>");
    let index = Index::open_in_dir(dir)?;
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let schema = index.schema();
    let f_doc_id = schema.get_field("doc_id")?;
    let f_title = schema.get_field("title")?;
    let f_parent_title = schema.get_field("parent_title")?;
    let f_url = schema.get_field("url")?;
    let f_parent_url = schema.get_field("parent_url")?;
    let f_icon = schema.get_field("icon")?;
    let f_search_text = schema.get_field("search_text")?;

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let docs = searcher.search(&AllQuery, &DocSetCollector)?;
    for addr in docs {
        let doc: TantivyDocument = searcher.doc(addr)?;
        let row = serde_json::json!({
            "doc_id": doc.get_first(f_doc_id).and_then(|v| v.as_str()).unwrap_or(""),
            "title": doc.get_first(f_title).and_then(|v| v.as_str()).unwrap_or(""),
            "parent_title": doc.get_first(f_parent_title).and_then(|v| v.as_str()),
            "url": doc.get_first(f_url).and_then(|v| v.as_str()).unwrap_or(""),
            "parent_url": doc.get_first(f_parent_url).and_then(|v| v.as_str()),
            "icon": doc.get_first(f_icon).and_then(|v| v.as_str()),
            "search_text": doc.get_first(f_search_text).and_then(|v| v.as_str()).unwrap_or(""),
        });
        writeln!(out, "{}", serde_json::to_string(&row)?)?;
    }
    out.flush()?;
    Ok(())
}

//! Dump every doc_id stored in a Tantivy index, one per line.
//!
//! Usage: cargo run -p fcdocs-indexer --example dump_ids -- <index_dir>

use std::env;

use anyhow::Result;
use tantivy::schema::Value;
use tantivy::{collector::DocSetCollector, query::AllQuery, Index, TantivyDocument};

fn main() -> Result<()> {
    let dir = env::args()
        .nth(1)
        .expect("usage: dump_ids <index_dir>");
    let index = Index::open_in_dir(dir)?;
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let schema = index.schema();
    let f_doc_id = schema.get_field("doc_id")?;

    let docs = searcher.search(&AllQuery, &DocSetCollector)?;
    let mut ids = Vec::with_capacity(docs.len());
    for addr in docs {
        let doc: TantivyDocument = searcher.doc(addr)?;
        if let Some(v) = doc.get_first(f_doc_id).and_then(|v| v.as_str()) {
            ids.push(v.to_string());
        }
    }
    ids.sort();
    for id in ids {
        println!("{id}");
    }
    Ok(())
}

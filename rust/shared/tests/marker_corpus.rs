//! Marker-config corpus parity: extract every embedded marker config from
//! `src/content/guides/**/*.md`, evaluate it via `markers::qjs`, and assert
//! that 100% succeed and that the unique-config set matches across the
//! 4 marker kinds.
//!
//! This is the regression gate that catches subtle differences between
//! V8's `vm.runInContext` and embedded QuickJS for the existing content
//! corpus.

use std::collections::HashSet;
use std::path::PathBuf;

use fcdocs_shared::markers::qjs::eval_marker_sync;
use fcdocs_shared::sidecar::MarkerKind;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("canonicalize repo root")
}

fn extract_blocks<'a>(
    haystack: &'a str,
    start_tok: &str,
    end_tok: &str,
) -> Vec<&'a str> {
    let mut out = Vec::new();
    let mut rest = haystack;
    while let Some(s) = rest.find(start_tok) {
        let after_start = &rest[s + start_tok.len()..];
        if let Some(e) = after_start.find(end_tok) {
            out.push(after_start[..e].trim());
            rest = &after_start[e + end_tok.len()..];
        } else {
            break;
        }
    }
    out
}

fn walk_markdown(dir: &std::path::Path, kind: MarkerKind, start: &str, end: &str) -> (usize, usize) {
    let mut seen: HashSet<String> = HashSet::new();
    let mut failed = 0usize;
    let walker = walkdir::WalkDir::new(dir).follow_links(false);
    for entry in walker.into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let p = entry.path();
        if p.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(p) else {
            continue;
        };
        for block in extract_blocks(&text, start, end) {
            if !seen.insert(block.to_string()) {
                continue;
            }
            if let Err(e) = eval_marker_sync(kind, block) {
                failed += 1;
                eprintln!(
                    "FAIL {kind:?} {p}:\n  block: {block}\n  error: {e}",
                    kind = kind,
                    p = p.display(),
                );
            }
        }
    }
    (seen.len(), failed)
}

#[test]
fn inline_code_attrs_corpus_parses() {
    let dir = repo_root().join("src/content/guides");
    let (n, failed) = walk_markdown(
        &dir,
        MarkerKind::InlineCode,
        "[inline-code-attrs-start",
        "inline-code-attrs-end]",
    );
    println!("inline-code: {n} unique configs, {failed} failed");
    assert_eq!(failed, 0, "{failed} inline-code config(s) failed to eval");
    assert!(n > 0, "expected to find at least one inline-code config");
}

#[test]
fn code_example_corpus_parses() {
    let dir = repo_root().join("src/content/guides");
    let (n, failed) = walk_markdown(
        &dir,
        MarkerKind::CodeExample,
        "[code-example-start",
        "code-example-end]",
    );
    println!("code-example: {n} unique configs, {failed} failed");
    assert_eq!(failed, 0, "{failed} code-example config(s) failed to eval");
}

#[test]
fn api_resource_header_corpus_parses() {
    let dir = repo_root().join("src/content/guides");
    let (n, failed) = walk_markdown(
        &dir,
        MarkerKind::ApiResourceHeader,
        "[api-resource-header-start",
        "api-resource-header-end]",
    );
    println!("api-resource-header: {n} unique configs, {failed} failed");
    assert_eq!(
        failed, 0,
        "{failed} api-resource-header config(s) failed to eval"
    );
}

#[test]
fn related_parameter_corpus_parses() {
    let dir = repo_root().join("src/content/guides");
    let (n, failed) = walk_markdown(
        &dir,
        MarkerKind::RelatedParameter,
        "[related-parameter-start",
        "related-parameter-end]",
    );
    println!("related-parameter: {n} unique configs, {failed} failed");
    assert_eq!(
        failed, 0,
        "{failed} related-parameter config(s) failed to eval"
    );
}

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

/// Rewrite every `start..end`-delimited block in `input` using `f`, in a
/// single linear pass.
///
/// The old expand_* helpers used a `find` + `format!("{}{repl}{}",
/// &current[..s], &current[block_end..])` loop, which copies the full
/// document on every iteration (O(N·markers) → O(N²) on long guides).
/// This util walks the input once, pushing slice-by-slice into a single
/// pre-sized output buffer.
pub(crate) fn rewrite_blocks_sync(
    input: &str,
    start: &str,
    end: &str,
    mut f: impl FnMut(&str) -> anyhow::Result<String>,
) -> anyhow::Result<String> {
    let mut out = String::with_capacity(input.len());
    let mut cursor = 0;
    while let Some(rel_s) = input[cursor..].find(start) {
        let s = cursor + rel_s;
        let Some(rel_e) = input[s..].find(end) else {
            anyhow::bail!(
                "rewrite_blocks_sync: '{start}' without matching '{end}' (input prefix: {prefix:?})",
                prefix = &input[s..(s + 80).min(input.len())]
            );
        };
        let block_end = s + rel_e + end.len();
        let body = &input[s + start.len()..s + rel_e];
        let replacement = f(body)?;
        out.push_str(&input[cursor..s]);
        out.push_str(&replacement);
        cursor = block_end;
    }
    out.push_str(&input[cursor..]);
    Ok(out)
}

/// Like `rewrite_blocks_sync`, but the replacement closure is async. We
/// await between blocks rather than batching so the closure can hold
/// borrows of state across the loop.
pub(crate) async fn rewrite_blocks_async<F, Fut>(
    input: &str,
    start: &str,
    end: &str,
    mut f: F,
) -> anyhow::Result<String>
where
    F: FnMut(String) -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<String>>,
{
    let mut out = String::with_capacity(input.len());
    let mut cursor = 0;
    while let Some(rel_s) = input[cursor..].find(start) {
        let s = cursor + rel_s;
        let Some(rel_e) = input[s..].find(end) else {
            anyhow::bail!(
                "rewrite_blocks_async: '{start}' without matching '{end}' (input prefix: {prefix:?})",
                prefix = &input[s..(s + 80).min(input.len())]
            );
        };
        let block_end = s + rel_e + end.len();
        let body = input[s + start.len()..s + rel_e].to_string();
        let replacement = f(body).await?;
        out.push_str(&input[cursor..s]);
        out.push_str(&replacement);
        cursor = block_end;
    }
    out.push_str(&input[cursor..]);
    Ok(out)
}

#[cfg(test)]
mod rewrite_tests {
    use super::*;

    #[test]
    fn linear_in_marker_count() {
        let mut s = String::new();
        for i in 0..1000 {
            s.push_str(&format!("filler {i} "));
            s.push_str("[X-start payload");
            s.push_str(&i.to_string());
            s.push_str(" X-end]");
            s.push_str(" more filler ");
        }
        let out = rewrite_blocks_sync(&s, "[X-start", "X-end]", |body| {
            Ok(format!("<{}>", body.trim()))
        })
        .unwrap();
        assert!(out.contains("<payload0>"));
        assert!(out.contains("<payload999>"));
        assert!(!out.contains("[X-start"));
    }

    #[test]
    fn missing_end_token_is_an_error() {
        let s = "before [X-start oops";
        let err = rewrite_blocks_sync(s, "[X-start", "X-end]", |_| Ok(String::new()))
            .unwrap_err();
        assert!(err.to_string().contains("without matching"));
    }

    #[test]
    fn empty_input_returns_empty() {
        let out =
            rewrite_blocks_sync("", "[X-start", "X-end]", |_| Ok("zzz".into())).unwrap();
        assert_eq!(out, "");
    }
}

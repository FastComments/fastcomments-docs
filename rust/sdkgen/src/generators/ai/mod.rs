//! AI-powered doc generators for typescript, rust, cpp, nim. Each
//! parses per-language source files, builds a deterministic prompt,
//! looks up the SHA256 cache key in `src/sdk-ai-cache/<sdk-id>/`, and
//! emits a markdown section.
//!
//! These are the leaf generators behind sdk-javascript, sdk-rust,
//! sdk-cpp, sdk-nim. Validated for byte-parity by reusing the
//! committed cache files: no LLM access required when source
//! signatures haven't changed.
//!
//! References:
//! - `src/sdk-doc-generators/typescript-ai-generator.js`
//! - `src/sdk-doc-generators/rust-ai-generator.js`
//! - `src/sdk-doc-generators/cpp-ai-generator.js`
//! - `src/sdk-doc-generators/nim-ai-generator.js`
//! - `src/sdk-doc-generators/openai-client.js`

pub mod common;
pub mod cpp;
pub mod cpp_parser;
pub mod nim;
pub mod nim_parser;
pub mod prompts;
pub mod rust;
pub mod rust_parser;
pub mod typescript;
pub mod typescript_parser;

pub use cpp::CppAiGenerator;
pub use nim::NimAiGenerator;
pub use rust::RustAiGenerator;
pub use typescript::TypescriptAiGenerator;

//! Shared OpenAI chat-completions client.
//!
//! Used by both `fcdocs-sdkgen`'s four AI doc generators and `fcdocs-trans`.
//! Mirrors `src/sdk-doc-generators/openai-client.js` so the existing
//! `sdk-ai-cache/` cache hits remain valid after the migration.

pub mod cache_key;
pub mod client;

pub use client::{LlmClient, LlmError, LlmResponse};

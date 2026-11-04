//! Shared LLM chat-completions client.
//!
//! Used by both `fcdocs-sdkgen`'s four AI doc generators and `fcdocs-trans`.
//! `sdk-ai-cache/` entries are keyed on the method and prompt, not the model,
//! so changing models does not invalidate them.

pub mod cache_key;
pub mod client;

pub use client::{LlmClient, LlmError, LlmResponse};

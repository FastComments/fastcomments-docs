//! Doc generators. Each implements `DocGenerator` and emits a set of
//! sections + optional intro/conclusion.

pub mod base;
pub mod readme;
pub mod openapi;
pub mod ai;

pub use base::{DocGenerator, DocSection, GeneratedDocs};

//! Shared types, content walker, and sidecar client for the FastComments docs
//! search system.
//!
//! Mirrors behavior in `src/guides.js`, `src/build-search-index-worker.js`,
//! and the marker processors under `src/`.

pub mod locales;
pub mod guides;
pub mod sidecar;
pub mod pipeline;

pub use locales::{Locale, Locales};
pub use guides::{Guide, MetaItem, GuideItem};
pub use sidecar::SidecarClient;

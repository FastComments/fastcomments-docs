//! Marker config evaluation. Replaces the build-time Node sidecar's
//! `/eval-marker` endpoint.
//!
//! Marker tags in source markdown embed real JS object-literal config blocks
//! between start/end tokens (e.g. `[inline-code-attrs-start title = 'x';
//! type = 'js'; inline-code-attrs-end]`). The original Node implementation
//! used `vm.createContext` + `vm.runInContext`. We now embed QuickJS via
//! `rquickjs` so we don't need a Node round-trip.

pub mod qjs;

pub use qjs::eval_marker;

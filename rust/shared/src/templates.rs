//! Loads and renders the 4 Handlebars templates under `src/templates/`
//! using the `handlebars` crate. Templates use only basic features:
//! `{{var}}`, `{{{var}}}`, `{{#if}}`, `{{#each}}`, `{{@key}}` — no custom
//! helpers needed (`t.KEY` is property access on the `t` translations
//! object, not a Handlebars helper).
//!
//! Stub — real implementation lands as part of task #13.

use std::path::Path;

use anyhow::{Context, Result};
use handlebars::Handlebars;

#[derive(Clone)]
pub struct TemplateRegistry {
    pub hb: Handlebars<'static>,
}

impl TemplateRegistry {
    pub fn load_from(template_dir: impl AsRef<Path>) -> Result<Self> {
        let mut hb = Handlebars::new();
        // Don't HTML-escape inside templates — markdown HTML already escapes
        // user content where needed and matches Node `handlebars`'s default
        // behavior of treating `{{var}}` as escaped and `{{{var}}}` as raw.
        for name in ["page", "guide-layout", "index", "code"] {
            let path = template_dir.as_ref().join(format!("{name}.html"));
            let body = std::fs::read_to_string(&path)
                .with_context(|| format!("read template {path:?}"))?;
            hb.register_template_string(name, &body)
                .with_context(|| format!("register template {name}"))?;
        }
        Ok(Self { hb })
    }

    pub fn render(&self, name: &str, ctx: &serde_json::Value) -> Result<String> {
        Ok(self.hb.render(name, ctx)?)
    }
}

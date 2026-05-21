//! Single source of truth for the marker token strings every
//! pipeline + every marker processor in the Node original looks for.
//!
//! Both [`super::indexer`] and [`super::full`] used to declare their
//! own copies. A typo or drift in one stayed silent until the other
//! pipeline was touched — fixing the wrong typo in one file would
//! leave the other still broken. Defining them here once eliminates
//! that risk.

pub const INLINE_CODE_START: &str = "[inline-code-start]";
pub const INLINE_CODE_END: &str = "[inline-code-end]";
pub const INLINE_CODE_ATTRS_START: &str = "[inline-code-attrs-start";
pub const INLINE_CODE_ATTRS_END: &str = "inline-code-attrs-end]";

pub const CODE_EXAMPLE_START: &str = "[code-example-start";
pub const CODE_EXAMPLE_END: &str = "code-example-end]";

pub const API_RES_START: &str = "[api-resource-header-start";
pub const API_RES_END: &str = "api-resource-header-end]";

pub const RELATED_PARAM_START: &str = "[related-parameter-start";
pub const RELATED_PARAM_END: &str = "related-parameter-end]";

pub const APP_SCREENSHOT_START: &str = "[app-screenshot-start";
pub const APP_SCREENSHOT_END: &str = "app-screenshot-end]";

pub const FLOW_DIAGRAM_START: &str = "[flow-diagram-start";
pub const FLOW_DIAGRAM_END: &str = "flow-diagram-end]";

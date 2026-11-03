//! # spec2artifact
//!
//! **The LLM writes a small, task-variant SPEC; the algebra renders the whole artifact.**
//! Fewer tokens, lower latency, exact output — deterministic, offline, zero LLM tokens
//! for the body.
//!
//! ```no_run
//! use spec2artifact::{spec, render};
//!
//! let mut s = spec::spec_template("records").unwrap();
//! s["n"] = serde_json::json!(1000);
//! let artifact: String = render::render(&s).unwrap();
//! assert_eq!(artifact.lines().count(), 1000);
//! ```
//!
//! Module map:
//! - [`spec`] — variant spec skeletons per task kind + structural validation
//! - [`render`] — deterministic renderers (pure functions: spec -> artifact)
//! - [`bundle`] — multi-part artifacts with cross-part `{ref:...}` references
//! - [`expr`] — the safe expression algebra behind `{expr:...}` (arithmetic over `ref(...)`)
//! - [`chart`] — deterministic Unicode charts from bundle data (bars/histogram/sparkline)
//! - [`rollup`] — group-by aggregations powering charts and rollup tables
//! - [`rng`] — the deterministic PRNG behind seeded rendering
//! - [`mcp`] — MCP stdio JSON-RPC server (`spec2artifact mcp`)

pub mod bundle;
pub mod chart;
pub mod expr;
pub mod mcp;
pub mod render;
pub mod rng;
pub mod rollup;
pub mod spec;

pub use render::render;
pub use spec::{kinds, spec_template, validate};

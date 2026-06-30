//! Tools the agent may use, gated by both the role (tool boundary) and the scope
//! (read-only default). v0.1 ships a filesystem tool.

pub mod fs;

pub use fs::FsTool;

/// Marker trait for tools (kept minimal in v0.1).
pub trait Tool {
    fn name(&self) -> &str;
}

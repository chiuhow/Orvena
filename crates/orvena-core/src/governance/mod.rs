//! Governance: the three disciplines v0.1 enforces — scope lock, read-only
//! default, and gates. (Bounded Change + Verifiable Gates pillars.)

pub mod gate;
pub mod scope;

pub use gate::{GateOutcome, GateRunner};
pub use scope::{Scope, ScopeDecision};

/// A bounded-change blocker: the agent could not proceed *within scope*, with a
/// recovery hint instead of silently expanding scope. (Evidence & Done pillar.)
#[derive(Debug, Clone)]
pub struct Blocker {
    pub reason: String,
    pub recovery: String,
}

impl Blocker {
    pub fn new(reason: impl Into<String>, recovery: impl Into<String>) -> Self {
        Self { reason: reason.into(), recovery: recovery.into() }
    }
}

impl std::fmt::Display for Blocker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (recovery: {})", self.reason, self.recovery)
    }
}

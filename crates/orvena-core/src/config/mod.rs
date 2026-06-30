//! Config-first surface. Every behavioral knob is YAML the user can edit without
//! forking code: provider selection, roles & tool boundaries, context budgets,
//! and gates. v0.1's minimal set is `orvena.yaml` / `roles.yaml` / `gates.yaml`
//! / `context-budgets.yaml`.

pub mod agent;
pub mod context_budget;
pub mod gates;
pub mod roles;

pub use agent::{AgentConfig, ProviderSelection, Tier};
pub use context_budget::{ContextBudget, ContextBudgets};
pub use gates::{Gate, Gatekeeper, Gates};
pub use roles::{Role, Roles};

use crate::error::{Error, Result};
use serde::de::DeserializeOwned;
use std::path::Path;

/// The fully-loaded config-first surface for a project.
#[derive(Debug, Clone)]
pub struct Config {
    pub agent: AgentConfig,
    pub roles: Roles,
    pub gates: Gates,
    pub budgets: ContextBudgets,
}

impl Config {
    /// Load the four config files from a directory (typically `.orvena/`).
    pub fn load_dir(dir: impl AsRef<Path>) -> Result<Self> {
        let dir = dir.as_ref();
        let cfg = Self {
            agent: read_yaml(dir.join("orvena.yaml"))?,
            roles: read_yaml(dir.join("roles.yaml"))?,
            gates: read_yaml(dir.join("gates.yaml"))?,
            budgets: read_yaml(dir.join("context-budgets.yaml"))?,
        };
        cfg.validate()?;
        Ok(cfg)
    }

    /// Cheap structural checks surfaced as human-readable errors (used by `doctor`).
    pub fn validate(&self) -> Result<()> {
        if self.roles.get(&self.agent.default_role).is_none() {
            return Err(Error::Config(format!(
                "default_role '{}' is not defined in roles.yaml",
                self.agent.default_role
            )));
        }
        if self.agent.max_steps == 0 {
            return Err(Error::Config("max_steps must be >= 1".into()));
        }
        Ok(())
    }
}

/// Read + deserialize a YAML file, mapping the path into the error for clarity.
pub(crate) fn read_yaml<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let path = path.as_ref();
    let text = std::fs::read_to_string(path)
        .map_err(|e| Error::Config(format!("cannot read {}: {e}", path.display())))?;
    serde_yaml::from_str(&text)
        .map_err(|e| Error::Config(format!("invalid YAML in {}: {e}", path.display())))
}

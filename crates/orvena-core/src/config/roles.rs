//! Roles & tool boundaries (`roles.yaml`). Each role declares which tools it may
//! use; forbidden tools are hard-denied. (Specialized Roles pillar.)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Roles {
    #[serde(default)]
    pub roles: Vec<Role>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    /// Tools this role may use. Empty = "all except forbidden".
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    /// Tools this role may never use (takes precedence over `allowed_tools`).
    #[serde(default)]
    pub forbidden_tools: Vec<String>,
    /// Directory hints this role is meant to reason about (advisory in v0.1).
    #[serde(default)]
    pub knowledge_scope: Vec<String>,
}

impl Roles {
    pub fn get(&self, name: &str) -> Option<&Role> {
        self.roles.iter().find(|r| r.name == name)
    }
}

impl Role {
    /// Hard-enforced tool boundary: forbidden wins; empty allow-list means
    /// "everything not forbidden".
    pub fn tool_allowed(&self, tool: &str) -> bool {
        if self.forbidden_tools.iter().any(|t| t == tool) {
            return false;
        }
        if self.allowed_tools.is_empty() {
            return true;
        }
        self.allowed_tools.iter().any(|t| t == tool)
    }
}

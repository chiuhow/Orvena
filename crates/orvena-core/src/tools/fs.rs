//! Filesystem tool. Every write passes two checks: the role must allow `fs.write`
//! (Specialized Roles) and the scope must mark the path writable (Bounded Change).
//! Reads are likewise role-gated. Tool names: `fs.read`, `fs.write`, `fs.list`.

use super::Tool;
use crate::config::roles::Role;
use crate::error::{Error, Result};
use crate::governance::scope::{Scope, ScopeDecision};
use std::path::PathBuf;

pub struct FsTool<'a> {
    pub root: PathBuf,
    pub scope: &'a Scope,
    pub role: &'a Role,
}

impl<'a> FsTool<'a> {
    pub fn new(root: impl Into<PathBuf>, scope: &'a Scope, role: &'a Role) -> Self {
        Self { root: root.into(), scope, role }
    }

    pub fn read(&self, rel: &str) -> Result<String> {
        self.require_tool("fs.read")?;
        Ok(std::fs::read_to_string(self.root.join(rel))?)
    }

    /// Read a file, returning `None` if it does not exist yet (for new files).
    pub fn read_opt(&self, rel: &str) -> Result<Option<String>> {
        self.require_tool("fs.read")?;
        let p = self.root.join(rel);
        if !p.exists() {
            return Ok(None);
        }
        Ok(Some(std::fs::read_to_string(p)?))
    }

    pub fn write(&self, rel: &str, content: &str) -> Result<()> {
        self.require_tool("fs.write")?;
        match self.scope.decision(rel) {
            ScopeDecision::Allow => {
                let p = self.root.join(rel);
                if let Some(parent) = p.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(p, content)?;
                Ok(())
            }
            ScopeDecision::ReadOnly => Err(Error::Scope(format!(
                "'{rel}' is read-only (not in allowed_modifications) — report a blocker, \
                 do not expand scope"
            ))),
            ScopeDecision::Excluded => {
                Err(Error::Scope(format!("'{rel}' is excluded from this task's scope")))
            }
        }
    }

    fn require_tool(&self, tool: &str) -> Result<()> {
        if self.role.tool_allowed(tool) {
            Ok(())
        } else {
            Err(Error::Scope(format!("role '{}' is not allowed to use '{tool}'", self.role.name)))
        }
    }
}

impl<'a> Tool for FsTool<'a> {
    fn name(&self) -> &str {
        "fs"
    }
}

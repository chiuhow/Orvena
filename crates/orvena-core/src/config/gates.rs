//! Gates (`gates.yaml`). A change is "done" only when it passes these gates.
//! Each gate has a human-readable condition, an optional `verify` command that
//! produces **observable evidence** (exit 0 = pass — the local analogue of
//! "re-run CI until green"), and a gatekeeper that is either `automated`
//! (evidence decides) or `human` (escalates and stops the loop).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Gates {
    #[serde(default)]
    pub gates: Vec<Gate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gate {
    pub name: String,
    pub condition: String,
    /// Shell command run to produce evidence. Exit 0 = pass.
    #[serde(default)]
    pub verify: Option<String>,
    #[serde(default)]
    pub gatekeeper: Gatekeeper,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Gatekeeper {
    /// Mechanical: the `verify` command's evidence decides pass/fail.
    #[default]
    Automated,
    /// Judgment: requires a human; the loop stops and reports a blocker.
    Human,
}

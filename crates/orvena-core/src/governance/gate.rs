//! Gate evaluation. An automated gate runs its `verify` command in the project
//! root and treats exit 0 as a pass, capturing the command output as observable
//! evidence (the local analogue of "re-run CI until green"). A human gate cannot
//! be auto-confirmed: it escalates and stops the loop.

use crate::config::gates::{Gate, Gatekeeper};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct GateOutcome {
    pub gate: String,
    pub passed: bool,
    /// Observable evidence (command output, or why it could not be auto-decided).
    pub evidence: String,
    /// True when the gate needs a human (judgment, not mechanics).
    pub needs_human: bool,
}

pub struct GateRunner;

impl GateRunner {
    pub fn run(gate: &Gate, cwd: &Path) -> GateOutcome {
        match gate.gatekeeper {
            Gatekeeper::Human => GateOutcome {
                gate: gate.name.clone(),
                passed: false,
                evidence: format!("'{}' requires human judgment", gate.condition),
                needs_human: true,
            },
            Gatekeeper::Automated => match &gate.verify {
                None => GateOutcome {
                    gate: gate.name.clone(),
                    passed: false,
                    evidence:
                        "automated gate has no `verify` command — cannot produce evidence".into(),
                    needs_human: false,
                },
                Some(cmd) => Self::run_verify(&gate.name, cmd, cwd),
            },
        }
    }

    fn run_verify(name: &str, cmd: &str, cwd: &Path) -> GateOutcome {
        let output = Command::new("sh").arg("-c").arg(cmd).current_dir(cwd).output();
        match output {
            Ok(out) => {
                let mut evidence = String::new();
                evidence.push_str(&String::from_utf8_lossy(&out.stdout));
                evidence.push_str(&String::from_utf8_lossy(&out.stderr));
                GateOutcome {
                    gate: name.to_string(),
                    passed: out.status.success(),
                    evidence: evidence.trim().to_string(),
                    needs_human: false,
                }
            }
            Err(e) => GateOutcome {
                gate: name.to_string(),
                passed: false,
                evidence: format!("could not run verify command: {e}"),
                needs_human: false,
            },
        }
    }
}

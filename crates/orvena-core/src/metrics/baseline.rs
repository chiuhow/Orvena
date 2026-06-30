//! Golden-task baseline: run a fixed task, freeze its frozen fields, and diff a
//! later run against the frozen record to catch "did this change break or get
//! more expensive". Design reference: the lab's `scripts/baseline/` (rewritten).

use super::RunReport;
use serde::{Deserialize, Serialize};

/// A fixed task used as a regression fixture. Run against the deterministic
/// `offline` provider so the baseline is reproducible.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenTask {
    pub id: String,
    pub instruction: String,
    pub allowed_modifications: Vec<String>,
}

/// The frozen subset of a run we regression-check.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BaselineRecord {
    pub task_id: String,
    pub completed: bool,
    pub steps: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub tool_calls: u32,
}

impl BaselineRecord {
    pub fn from_report(task_id: impl Into<String>, report: &RunReport) -> Self {
        Self {
            task_id: task_id.into(),
            completed: report.completed,
            steps: report.steps,
            input_tokens: report.input_tokens,
            output_tokens: report.output_tokens,
            tool_calls: report.tool_calls,
        }
    }

    /// Human-readable differences vs a frozen baseline (empty = no regression).
    pub fn diff(&self, frozen: &BaselineRecord) -> Vec<String> {
        let mut out = Vec::new();
        if self.completed != frozen.completed {
            out.push(format!("completed: {} -> {}", frozen.completed, self.completed));
        }
        if self.steps != frozen.steps {
            out.push(format!("steps: {} -> {}", frozen.steps, self.steps));
        }
        if self.input_tokens != frozen.input_tokens {
            out.push(format!("input_tokens: {} -> {}", frozen.input_tokens, self.input_tokens));
        }
        if self.output_tokens != frozen.output_tokens {
            out.push(format!("output_tokens: {} -> {}", frozen.output_tokens, self.output_tokens));
        }
        if self.tool_calls != frozen.tool_calls {
            out.push(format!("tool_calls: {} -> {}", frozen.tool_calls, self.tool_calls));
        }
        out
    }
}

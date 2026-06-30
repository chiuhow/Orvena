//! `orvena run "<task>"` — load config, resolve any matching skill, run one
//! bounded loop, and print the run report (the L1 metric fields).

use super::{config_dir, load_dotenv, project_root};
use anyhow::{bail, Result};
use orvena_core::config::Config;
use orvena_core::skills::{self, SkillRegistry};
use orvena_core::{Agent, Task};

pub async fn run(task_text: String, write: Vec<String>) -> Result<()> {
    load_dotenv();

    let dir = config_dir();
    if !super::is_initialized(&dir) {
        bail!("no config found — run `orvena init` first");
    }
    let config = Config::load_dir(&dir)?;

    // Resolve a skill from the task text (engine ships in v0.1; content grows
    // one reviewed skill at a time).
    let registry = SkillRegistry::discover(dir.join("skills"))?;
    let active_role = config.agent.default_role.clone();
    let instruction = match skills::resolve(&registry, &task_text, &active_role) {
        Some(skill) => {
            println!("(applying skill '{}')", skill.name);
            skills::apply(skill, &task_text)
        }
        None => task_text.clone(),
    };

    let agent = Agent::new(config, project_root())?;
    let report = agent.run(Task::new(instruction, write)).await?;

    print_report(&report);
    if !report.completed {
        bail!("run did not complete (see blockers above)");
    }
    Ok(())
}

fn print_report(report: &orvena_core::RunReport) {
    println!("\n── run report ──");
    println!("completed:     {}", report.completed);
    println!("steps:         {}", report.steps);
    println!("tool calls:    {}", report.tool_calls);
    println!(
        "tokens:        {} in / {} out ({} total)",
        report.input_tokens,
        report.output_tokens,
        report.total_tokens()
    );
    if !report.gate_outcomes.is_empty() {
        println!("gates:");
        for g in &report.gate_outcomes {
            let mark = if g.passed { "pass" } else if g.needs_human { "human" } else { "fail" };
            println!("  - {:<20} {}", g.gate, mark);
        }
    }
    if !report.blockers.is_empty() {
        println!("blockers:");
        for b in &report.blockers {
            println!("  - {b}");
        }
    }
}

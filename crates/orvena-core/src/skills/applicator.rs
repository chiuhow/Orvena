//! Skill application: fold a resolved skill's procedure into the task
//! instruction so the loop runs with that guidance in context.

use super::registry::Skill;

/// Produce an augmented instruction with the skill's procedure prepended.
pub fn apply(skill: &Skill, instruction: &str) -> String {
    format!(
        "Apply the '{}' skill:\n{}\n\n---\nUser request: {}",
        skill.name, skill.body, instruction
    )
}

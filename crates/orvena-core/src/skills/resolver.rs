//! Skill resolution: match user input to a skill by an explicit `/slash` command
//! or a trigger phrase, respecting the active role. First match wins.

use super::registry::{Skill, SkillRegistry};

/// Resolve a skill for the given input and active role. A skill scoped to a role
/// only matches when that role is active; role-less skills match any role.
pub fn resolve<'a>(
    registry: &'a SkillRegistry,
    input: &str,
    active_role: &str,
) -> Option<&'a Skill> {
    let lower = input.to_lowercase();
    registry.skills.iter().find(|s| {
        let role_ok = s.role.as_deref().map(|r| r == active_role).unwrap_or(true);
        role_ok && s.triggers.iter().any(|t| matches_trigger(&lower, t))
    })
}

fn matches_trigger(input_lower: &str, trigger: &str) -> bool {
    let t = trigger.to_lowercase();
    if let Some(cmd) = t.strip_prefix('/') {
        // Slash command: match as a leading token.
        input_lower == t || input_lower.starts_with(&format!("/{cmd} "))
    } else {
        input_lower.contains(&t)
    }
}

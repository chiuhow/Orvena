//! Skill discovery. Scans `<dir>/<name>/SKILL.md` files, each with YAML
//! front-matter (`name`, `description`, `triggers`, optional `role`) followed by
//! a Markdown body (the procedure).

use crate::error::Result;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub triggers: Vec<String>,
    pub role: Option<String>,
    /// The Markdown body (the procedure injected into context when applied).
    pub body: String,
    pub path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct FrontMatter {
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    triggers: Vec<String>,
    #[serde(default)]
    role: Option<String>,
}

#[derive(Debug, Default)]
pub struct SkillRegistry {
    pub skills: Vec<Skill>,
}

impl SkillRegistry {
    /// Discover all skills under a directory (typically `.orvena/skills/`).
    /// A missing directory yields an empty registry (skills are optional).
    pub fn discover(dir: impl AsRef<Path>) -> Result<Self> {
        let dir = dir.as_ref();
        let mut skills = Vec::new();
        if !dir.exists() {
            return Ok(Self { skills });
        }
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let skill_md = entry.path().join("SKILL.md");
            if skill_md.is_file() {
                if let Some(skill) = parse_skill(&skill_md)? {
                    skills.push(skill);
                }
            }
        }
        skills.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(Self { skills })
    }
}

/// Parse a `SKILL.md`. Returns `None` if it has no valid front-matter.
fn parse_skill(path: &Path) -> Result<Option<Skill>> {
    let text = std::fs::read_to_string(path)?;
    let Some((front, body)) = split_front_matter(&text) else {
        return Ok(None);
    };
    let fm: FrontMatter = match serde_yaml::from_str(front) {
        Ok(fm) => fm,
        Err(_) => return Ok(None),
    };
    Ok(Some(Skill {
        name: fm.name,
        description: fm.description,
        triggers: fm.triggers,
        role: fm.role,
        body: body.trim().to_string(),
        path: path.to_path_buf(),
    }))
}

/// Split `---\n<yaml>\n---\n<body>` into (yaml, body).
fn split_front_matter(text: &str) -> Option<(&str, &str)> {
    let rest = text.strip_prefix("---")?;
    let rest = rest.trim_start_matches(['\r', '\n']);
    let end = rest.find("\n---")?;
    let front = &rest[..end];
    let body = rest[end..].trim_start_matches('\n');
    let body = body.strip_prefix("---").unwrap_or(body);
    Some((front, body))
}

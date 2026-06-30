//! Minimal skill engine: reusable, triggerable procedures. The *engine*
//! (discover → resolve → apply) ships in v0.1; skill *content* is added one
//! reviewed skill at a time. A skill is a `SKILL.md` with YAML front-matter and a
//! Markdown body describing the procedure.

pub mod applicator;
pub mod registry;
pub mod resolver;

pub use applicator::apply;
pub use registry::{Skill, SkillRegistry};
pub use resolver::resolve;

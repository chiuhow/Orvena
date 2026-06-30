//! Error types. Hard failures surface as [`Error`]; recoverable "can't proceed
//! within scope" situations surface as a [`crate::governance::Blocker`] recorded
//! on the run report (with a recovery hint), per the Evidence & Done pillar.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("config error: {0}")]
    Config(String),

    #[error("provider error: {0}")]
    Provider(String),

    /// A write or tool use that the scope lock / role boundary forbids.
    #[error("scope violation: {0}")]
    Scope(String),

    /// A gate could not be evaluated (not the same as a gate failing).
    #[error("gate error: {0}")]
    Gate(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("yaml error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

use std::{io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NibbError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("TOML serializing error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("Unsupported language extension: {0}")]
    UnsupportedLanguage(String),

    #[error("Missing field: {0}")]
    MissingField(&'static str),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid snippet slug: {0}")]
    InvalidSlug(String),

    #[error("Other: {0}")]
    Other(String),
}

pub type NibbResult<T> = Result<T, NibbError>;
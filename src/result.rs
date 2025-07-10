use std::{io};
use serde::{Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NibbError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("TOML serializing error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),

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

impl NibbError {

    pub fn to_json(&self) -> String {
        serde_json::to_string(&NibbFFIError::from(self))
            .unwrap_or_else(|_| "{\"type\":\"Other\",\"message\":\"Serialization failed\"}".to_string())
    }
}


#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum NibbFFIError {
    Io(String),
    Toml(String),
    TomlSer(String),
    UnsupportedLanguage(String),
    MissingField(&'static str),
    NotFound(String),
    InvalidSlug(String),
    FFIError(String),
    Other(String),
}

impl NibbFFIError {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| "{\"type\":\"Other\",\"message\":\"Serialization failed\"}".to_string())
    }
}

impl From<&NibbError> for NibbFFIError {
    fn from(err: &NibbError) -> Self {
        match err {
            NibbError::Io(e) => Self::Io(e.to_string()),
            NibbError::Toml(e) => Self::Toml(e.to_string()),
            NibbError::TomlSer(e) => Self::TomlSer(e.to_string()),
            NibbError::SerdeJson(e) => Self::FFIError(e.to_string()),
            NibbError::UnsupportedLanguage(s) => Self::UnsupportedLanguage(s.to_string()),
            NibbError::MissingField(f) => Self::MissingField(f),
            NibbError::NotFound(s) => Self::NotFound(s.to_string()),
            NibbError::InvalidSlug(s) => Self::InvalidSlug(s.to_string()),
            NibbError::Other(s) => Self::Other(s.to_string()),
        }
    }
}
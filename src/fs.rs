use std::path::PathBuf;
use dirs::home_dir;
use crate::result::{NibbError, NibbResult};

/// Get the path to the main Nibb directory
pub fn get_nibb_dir() -> NibbResult<PathBuf> {
    Ok(home_dir().ok_or(NibbError::NotFound("home directory".to_string()))?.join(".nibb"))
}
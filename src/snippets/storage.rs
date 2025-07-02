#![allow(dead_code)]
use crate::errors::NibbError;
use crate::snippets::snippet::Snippet;
use crate::utils::fs::get_storage_path;

/// Stores snippets on the disk
pub fn save_snippets(snippets: &[Snippet]) -> Result<(), NibbError>{
    let path = get_storage_path()?;
    let data = serde_json::to_string_pretty(&snippets).expect("Error: Unable to serialize snippets");
    std::fs::write(path, data).map_err(|e| NibbError::FSError(e.to_string()))
}
/// Loads saved snippets from disk
pub fn load_snippets() -> Result<Vec<Snippet>, NibbError>{
    let path = get_storage_path()?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let data = std::fs::read_to_string(path)?;
    serde_json::from_str(&data).map_err(|e| NibbError::FSError(e.to_string()))
}


use crate::snippets::snippet::Snippet;
use crate::utils::fs::get_storage_path;
use anyhow::{Context, Result};

pub fn save_snippets(snippets: &[Snippet]) -> Result<()>{
    let path = get_storage_path()?;
    let data = serde_json::to_string_pretty(&snippets).expect("Error: Unable to serialize snippets");
    std::fs::write(path, data).context("Error: Unable to write snippets to file")
}

pub fn load_snippets() -> Result<Vec<Snippet>>{
    let path = get_storage_path()?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let data = std::fs::read_to_string(path)
        .context("Error: Unable to read snippets from file")?;
    serde_json::from_str(&data).context("Error: Unable to deserialize snippets")
}


use std::io::Write;
use crate::snippets::storage::load_snippets;
use anyhow::{anyhow, Result};
use crate::snippets::snippet::Snippet;
use crate::utils::clipboard::copy_to_clipboard;

pub fn insert_to_clipboard(name: &str) -> Result<()> {
    let snippets = load_snippets()?;
    let snippet = get_snippet(name, &snippets)?;
    let content = snippet.content.clone();
    copy_to_clipboard(&content).map_err(|e| anyhow!("Failed to copy to clipboard: {}", e))?;
    Ok(())
}

use std::fs::{self, OpenOptions};

pub fn insert_to_file_end(name: &str, file: &str) -> Result<()> {
    let original = fs::read_to_string(file)?;
    let content = get_snippet(name, &load_snippets()?)?.content.clone();
    let new_content = format!("{}\n{}", original, content);
    fs::write(file, new_content)?;
    Ok(())
}
pub fn insert_to_file_start(name: &str, file: &str) -> Result<()> {
    let original = fs::read_to_string(file)?;
    let content = get_snippet(name, &load_snippets()?)?.content.clone();
    let new_content = format!("{}\n{}", content, original);
    fs::write(file, new_content)?;
    Ok(())
}

pub fn get_snippet<'a>(name: &str, snippets: &'a [Snippet]) -> Result<&'a Snippet> {
    for snippet in snippets {
        if snippet.name == name {
            return Ok(snippet);
        }
    };
    Err(anyhow!("Snippet not found"))
}
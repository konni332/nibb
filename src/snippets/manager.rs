#![allow(dead_code)]

use crate::snippets::storage::{load_snippets, save_snippets};
use crate::snippets::snippet::Snippet;
use crate::utils::clipboard::copy_to_clipboard;
use std::fs::{self};
use std::process::Command;
use crossterm::style::Stylize;
use tempfile::NamedTempFile;
use crate::cli::command::Position;
use crate::errors::NibbError;
use std::io::Write;

// === Insertions ===
/// Inserts the content of the given snippet, if found, into the systems clipboard
pub fn insert_to_clipboard(name: &str) -> Result<(), NibbError> {
    let snippets = load_snippets()?;
    let snippet = get_snippet(name, &snippets)?;
    let content = snippet.content.clone();
    copy_to_clipboard(&content)?;
    Ok(())
}
/// Appends the content of the given snippet, if found, to the given file if it exists
pub fn insert_to_file_end(name: &str, file: &str) -> Result<(), NibbError> {
    let original = fs::read_to_string(file)?;
    let content = get_snippet(name, &load_snippets()?)?.content.clone();
    let new_content = format!("{}\n{}", original, content);
    fs::write(file, new_content)?;
    Ok(())
}
/// Prepends the content of the given snippet, if found, to the given file if it exists
pub fn insert_to_file_start(name: &str, file: &str) -> Result<(), NibbError> {
    let original = fs::read_to_string(file)?;
    let content = get_snippet(name, &load_snippets()?)?.content.clone();
    let new_content = format!("{}\n{}", content, original);
    fs::write(file, new_content)?;
    Ok(())
}

/// Genric insert function that matches the given Position and calls the appropriate insert function
pub fn insert_snippet(name: String, file: Option<String>, at: Position) -> Result<(), NibbError> {
    let file = if at != Position::Clipboard {
        if let Some(file) = file {
            file
        }
        else { 
            return Err(NibbError::FSError("No file specified for insertion".to_string()))
        }
    }
    else {
        "".to_string()
    };
    match at {
        Position::Clipboard => {
            insert_to_clipboard(&name)?;
            println!("Snippet '{}' copied to clipboard", name);
        },
        Position::End => {
            insert_to_file_end(&name, &file)?;
            println!("Snippet '{}' inserted at end of file", name);
        }
        Position::Start => {
            insert_to_file_start(&name, &file)?;
            println!("Snippet '{}' inserted at start of file", name);
        },
        Position::Cursor => {
            eprintln!("Cursor inserts are not available in CLI. Use a editor integration instead.")
        },
        Position::Marker => {
            println!("Snippet '{}' inserted at marker[s] in file", name);
        },
    }
    Ok(())
}

// === CRUD ===
/// Deletes the given snippet from disk storage if it exists
pub fn delete_snippet(name: String) -> Result<(), NibbError> {
    let mut snippets = load_snippets()?;
    let old_len = snippets.len();
    snippets.retain(|snippet| snippet.name != name);
    save_snippets(&snippets)?;
    if snippets.len() < old_len {
        println!("Snippet '{}' deleted", name);
    }
    else {
        Err(NibbError::NotFound(format!("Snippet {}", name)))?
    }
    Ok(())
}
/// Renames the given snippet if it exists
pub fn rename_snippet(old_name: String, new_name: String) -> Result<(), NibbError> {
    let mut snippets = load_snippets()?;
    for snippet in snippets.iter_mut() {
        if snippet.name == old_name {
            snippet.name = new_name.to_string();
        }
    }
    save_snippets(&snippets)?;
    Ok(())
}
/// Lists all snippets, or if it is given tags, it will only list 
pub fn list_snippets(tags: Option<Vec<String>>) -> Result<Vec<Snippet>, NibbError> {
    let snippets = load_snippets()?;
    let tags = tags.unwrap_or_default();

    let matching_snippets: Vec<Snippet> = snippets
        .into_iter()
        .filter(|snippet| {
            snippet.tags.iter().any(|tag| tags.contains(tag))
                || tags.is_empty()
                || snippet.name.contains(&tags[0])
        })
        .collect();

    Ok(matching_snippets)
}


pub fn new_snippet(name: String, tags: Option<Vec<String>>) -> Result<(), NibbError> {
    let mut snippets = load_snippets()?;
    let snippet = Snippet::create(name, tags);
    snippets.push(snippet);
    save_snippets(&snippets)?;
    Ok(())
}

pub fn edit_snippet(name: String, editor: &str) -> Result<(), NibbError> {
    let mut snippets = load_snippets()?;
    let index = snippets.iter().position(|s| s.name == name);
    if let Some(i) = index {
        let mut temp_file = NamedTempFile::new()
            .map_err(|e| NibbError::FSError(format!("Could not create temp file: {}", e)))?;
        write!(temp_file, "{}", snippets[i].content)
            .map_err(|e| NibbError::FSError(format!("Could not write to temp file: {}", e)))?;

        let status = Command::new(editor)
            .arg(temp_file.path())
            .status()
            .map_err(|e| NibbError::EditorError(format!("Could not execute editor: {}", e)))?;

        if !status.success() {
            return Err(NibbError::EditorError(
                format!("Editor exited with status {}", status.code().unwrap())
            ))
        }

        let new_content = fs::read_to_string(temp_file.path())
            .map_err(|e| NibbError::FSError(format!("Could not read temp file: {}", e)))?;
        snippets[i].content = new_content;
        save_snippets(&snippets)?;
        Ok(())
    }
    else {
        Err(NibbError::NotFound(format!("Snippet {}", name)))
    }
}

// === Utils ===

pub fn get_snippet<'a>(name: &str, snippets: &'a [Snippet]) -> Result<&'a Snippet, NibbError> {
    for snippet in snippets {
        if snippet.name == name {
            return Ok(snippet);
        }
    };
    Err(NibbError::NotFound(format!("Snippet {}", name)))
}

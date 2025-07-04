#![allow(dead_code)]

use crate::snippets::snippet::Snippet;
use crate::utils::clipboard::copy_to_clipboard;
use std::fs::{self};
use std::process::Command;
use tempfile::NamedTempFile;
use crate::errors::NibbError;
use std::io::Write;
use crate::utils::clipboard;
use crate::utils::markers::find_markers;

// === Insertions ===
/// Inserts the content of the given snippet, if found, into the systems clipboard
pub fn insert_to_clipboard(name: &str, snippets: &[Snippet]) -> Result<(), NibbError> {
    let snippet = get_snippet(name, &snippets)?;
    let content = snippet.content.clone();
    copy_to_clipboard(&content)?;
    Ok(())
}
/// Appends the content of the given snippet, if found, to the given file if it exists
pub fn insert_to_file_end(name: &str, file: &str, snippets: &[Snippet]) -> Result<(), NibbError> {
    let original = fs::read_to_string(file)?;
    let content = get_snippet(name, snippets)?.content.clone();
    let new_content = format!("{}\n{}", original, content);
    fs::write(file, new_content)?;
    Ok(())
}
/// Prepends the content of the given snippet, if found, to the given file if it exists
pub fn insert_to_file_start(name: &str, file: &str, snippets: &[Snippet]) -> Result<(), NibbError> {
    let original = fs::read_to_string(file)?;
    let content = get_snippet(name, snippets)?.content.clone();
    let new_content = format!("{}\n{}", content, original);
    fs::write(file, new_content)?;
    Ok(())
}

/// Inserts snippet content into the given marker in the given file and might prompt you to confirm,
/// depending on the passed in prompt_fn function.
pub fn insert_to_file_marker<F>(
    name: &str,
    file: &str,
    marker: &str,
    snippets: &[Snippet],
    prompt_fn: F
) -> Result<(), NibbError> 
where 
    F: Fn(&[usize]) -> Result<Vec<usize>, std::io::Error>
{
    let snippet = get_snippet(name, &snippets)?;
    let content = &snippet.content;
    find_markers(content, file, marker, prompt_fn)?;
    Ok(())
}

// === CRUD ===
/// Deletes the given snippet from disk storage if it exists
pub fn delete_snippet(name: String, snippets: &mut Vec<Snippet>) -> Result<(), NibbError> {
    let old_len = snippets.len();
    snippets.retain(|snippet| snippet.name != name);
    if snippets.len() < old_len {
        println!("Snippet '{}' deleted", name);
    }
    else {
        Err(NibbError::NotFound(format!("Snippet {}", name)))?
    }
    Ok(())
}
/// Renames the given snippet if it exists
pub fn rename_snippet(old_name: String, new_name: String, snippets: &mut [Snippet]) -> Result<(), NibbError> {
    let snippet = get_snippet_mut(&old_name, snippets)?;
    snippet.name = new_name;
    Ok(())
}
/// Lists all snippets that have at least one of the tags given.
/// If no tags are given, all snippets will be listed
pub fn list_snippets(
    tags: Option<Vec<String>>,
    snippets: &[Snippet],
) -> Result<Vec<&Snippet>, NibbError> {
    let tags = tags.unwrap_or_default();

    let matching_snippets = snippets
        .iter()
        .filter(|snippet| {
            snippet.tags.iter().any(|tag| tags.contains(tag))
                || tags.is_empty()
                || snippet.name.contains(&tags[0])
        })
        .collect::<Vec<&Snippet>>();

    Ok(matching_snippets)
}


/// Creates a new, empty snippet and saves it to disk
pub fn new_snippet(
    name: String,
    tags: Option<Vec<String>>,
    snippets: &mut Vec<Snippet>
) -> Result<&Snippet, NibbError> {
    let snippet = Snippet::create(name, tags);
    snippets.push(snippet);
    Ok(&snippets.last().unwrap())
}

/// Executes the specified editor on a temporary file, containing the snippet.
/// After saving the edited content in the temporary file, the content is saved into the snippet.
pub fn edit_snippet(snippet: &mut Snippet, editor: &str, clip: bool) -> Result<(), NibbError> {
    let new_content: String;
    if clip {
        new_content = clipboard::paste_from_clipboard()?;
    }
    else {
        let mut temp_file = NamedTempFile::new()
            .map_err(|e| NibbError::FSError(format!("Could not create temp file: {}", e)))?;
        write!(temp_file, "{}", snippet.content)
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

        new_content = fs::read_to_string(temp_file.path())
            .map_err(|e| NibbError::FSError(format!("Could not read temp file: {}", e)))?;
    }
    
    snippet.content = new_content;
    Ok(())
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

pub fn get_snippet_mut<'a>(
    name: &str, snippets: &'a mut [Snippet]
) -> Result<&'a mut Snippet, NibbError> {
    for snippet in snippets {
        if snippet.name == name {
            return Ok(snippet);
        }
    };
    Err(NibbError::NotFound(format!("Snippet {}", name)))
}

pub fn add_tag(snippet: &mut Snippet, tag: &str) -> Result<(), NibbError> {
    snippet.tags.insert(tag.to_string());
    Ok(())
}

pub fn remove_tag(snippet: &mut Snippet, tag: &str) -> Result<(), NibbError> {
    snippet.tags.remove(tag);
    Ok(())
}

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub fn fuzzy_search<'a>(query: &str, snippets: &'a [Snippet]) -> Vec<&'a Snippet> {
    let matcher = SkimMatcherV2::default();

    let mut results: Vec<(&Snippet, i64)> = snippets
        .iter()
        .filter_map(|snippet| {
            let haystack = format!(
                "{} {} {:?} {:?}",
                snippet.name,
                snippet.content,
                snippet.description,
                snippet.tags
            );
            matcher.fuzzy_match(&haystack, query).map(|score| (snippet, score))
        })
        .collect();
    
    results.sort_by_key(|(_, score)| -score);
    results.into_iter().map(|(snippet, _)| snippet).collect()
}















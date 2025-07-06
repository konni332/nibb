#![allow(dead_code)]

use crate::snippets::snippet::{Lang, Snippet};
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
pub fn insert_to_clipboard(name: &str, conn: &Connection) -> Result<(), NibbError> {
    let snippet = get_snippet_by_name(conn, name)?;
    let content = snippet.content.clone();
    copy_to_clipboard(&normalize_content(&content))?;
    Ok(())
}
/// Appends the content of the given snippet, if found, to the given file if it exists
pub fn insert_to_file_end(name: &str, file: &str, conn: &Connection) -> Result<(), NibbError> {
    let original = fs::read_to_string(file)?;
    let content = get_snippet_by_name(conn, name)?.content.clone();
    let new_content = format!("{}\n{}", original, content);
    fs::write(file, normalize_content(&new_content))?;
    Ok(())
}
/// Prepends the content of the given snippet, if found, to the given file if it exists
pub fn insert_to_file_start(name: &str, file: &str, conn: &Connection) -> Result<(), NibbError> {
    let original = fs::read_to_string(file)?;
    let content = get_snippet_by_name(conn, name)?.content.clone();
    let new_content = format!("{}\n{}", content, original);
    fs::write(file, normalize_content(&new_content))?;
    Ok(())
}

/// Inserts snippet content into the given marker in the given file and might prompt you to confirm,
/// depending on the passed in prompt_fn function.
pub fn insert_to_file_marker<F>(
    name: &str,
    file: &str,
    marker: &str,
    conn: &Connection,
    prompt_fn: F
) -> Result<(), NibbError> 
where 
    F: Fn(&[usize]) -> Result<Vec<usize>, std::io::Error>
{
    let snippet = get_snippet_by_name(conn, name)?;
    let content = &snippet.content;
    find_markers(content, file, marker, prompt_fn)?;
    Ok(())
}

// === CRUD ===
//

/// Renames the given snippet if it exists
pub fn rename_snippet(old_name: String, new_name: String, conn: &mut Connection) -> Result<(), NibbError> {
    let mut new_snippet = get_snippet_by_name(conn, &old_name)?.clone();
    let id = new_snippet.id;
    new_snippet.name = new_name;
    update_snippet(conn, &new_snippet, id)
}



/// Creates a new, empty snippet and saves it to disk
pub fn new_snippet(
    name: String,
    tags: Option<Vec<String>>,
    lang: Lang,
    conn: &mut Connection
) -> Result<Snippet, NibbError> {
    let snippet = Snippet::create(name, tags, lang);
    insert_snippet(conn, &snippet)?;
    Ok(snippet)
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



pub fn add_tag(conn: &mut Connection, snippet_name: &str, tag: &str) -> Result<(), NibbError> {
    add_tag_db(conn, snippet_name, tag)
}

pub fn remove_tag(conn: &mut Connection, snippet_name: &str, tag: &str) -> Result<(), NibbError> {
    rm_tag_db(conn, snippet_name, tag)
}

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use rusqlite::Connection;
use crate::snippets::storage::{add_tag_db, get_snippet, get_snippet_by_name, insert_snippet, rm_tag_db, update_snippet};
use crate::utils::fs::normalize_content;

pub fn fuzzy_search<'a>(query: &str, snippets: &'a [Snippet]) -> Vec<&'a Snippet> {
    let matcher = SkimMatcherV2::default();

    let mut results: Vec<(&Snippet, i64)> = snippets
        .iter()
        .filter_map(|snippet| {
            let haystack = format!(
                "{} {} {:?} {:?} {}",
                snippet.name,
                snippet.content,
                snippet.description,
                snippet.tags,
                snippet.lang,
            );
            matcher.fuzzy_match(&haystack, query).map(|score| (snippet, score))
        })
        .collect();
    
    results.sort_by_key(|(_, score)| -score);
    results.into_iter().map(|(snippet, _)| snippet).collect()
}















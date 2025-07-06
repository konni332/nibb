#![allow(dead_code)]

use std::collections::HashSet;
use rusqlite::{params, Connection};
use crate::errors::NibbError;
use crate::snippets::snippet::{Lang, Snippet};
use crate::utils::fs::{get_nibb_backups_dir, get_storage_path};



pub fn export_snippets(snippets: &[Snippet], name: Option<String>, pretty: bool) -> Result<(), NibbError> {
    let data: String;
    if pretty {
        data = serde_json::to_string_pretty(&snippets)
            .map_err(|e| NibbError::FSError(e.to_string()))?;
    }
    else {
        data = serde_json::to_string(&snippets)
            .map_err(|e| NibbError::FSError(e.to_string()))?;
    }
    let path = get_nibb_backups_dir()?.join(name.unwrap_or("snippets.json". to_string()));
    std::fs::write(path, data)
        .map_err(|e| NibbError::FSError(e.to_string()))
}

pub fn import_snippets(name: Option<String>) -> Result<Vec<Snippet>, NibbError> {
    let path = get_nibb_backups_dir()?.join(name.unwrap_or("snippets.json". to_string()));
    let data = std::fs::read_to_string(path)?;
    let snippets = serde_json::from_str(&data)
        .map_err(|e| NibbError::FSError(e.to_string()))?;
    Ok(snippets)
}

pub fn update_snippet(conn: &mut Connection, snippet: &Snippet, id: i32) -> Result<(), NibbError> {
    let tx = conn.transaction()?;

    tx.execute(
        "UPDATE snippets SET name = ?1, content = ?2, description = ?3, lang = ?4 WHERE id = ?5",
        params![snippet.name, snippet.content, snippet.description, snippet.lang.to_string(), id],
    )?;

    tx.execute("DELETE FROM snippets_tags WHERE snippet_id = ?1", params![id])?;

    for tag in &snippet.tags {
        tx.execute("INSERT OR IGNORE INTO tags (name) VALUES (?1)", params![tag])?;

        let tag_id: i64 = tx.query_row(
            "SELECT id FROM tags WHERE name = ?1",
            params![tag],
            |row| row.get(0),
        )?;

        tx.execute(
            "INSERT OR IGNORE INTO snippets_tags (snippet_id, tag_id) VALUES (?1, ?2)",
            params![id, tag_id],
        )?;
    }

    tx.commit()?;
    Ok(())
}


pub fn delete_snippet(conn: &mut Connection, id: i32) -> Result<(), NibbError> {
    let tx = conn.transaction()?;

    tx.execute("DELETE FROM snippets_tags WHERE snippet_id = ?1", params![id])?;

    tx.execute("DELETE FROM snippets WHERE id = ?1", params![id])?;

    tx.commit()?;
    Ok(())
}
pub fn list_snippets(conn: &Connection) -> Result<Vec<Snippet>, NibbError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, content, description, lang FROM snippets"
    )?;

    let snippet_rows = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let content: String = row.get(2)?;
        let description: Option<String> = row.get(3)?;
        let lang_str: String = row.get(4)?;
        let lang = Lang::from(lang_str.as_str());       


        Ok((id, name, content, description, lang))
    })?;

    let mut snippets = Vec::new();

    for row_result in snippet_rows {
        let (id, name, content, description, lang) = row_result?;

        let mut tag_stmt = conn.prepare(
            "
            SELECT tags.name
            FROM tags
            INNER JOIN snippets_tags ON tags.id = snippets_tags.tag_id
            WHERE snippets_tags.snippet_id = ?
            "
        )?;

        let tags_iter = tag_stmt.query_map(params![id], |tag_row| {
            let tag: String = tag_row.get(0)?;
            Ok(tag)
        })?;

        let mut tags = HashSet::new();
        for tag in tags_iter {
            tags.insert(tag?);
        }

        snippets.push(Snippet {
            id,
            name,
            content,
            description,
            tags,
            lang,
        });
    }
    Ok(snippets)
}

pub fn filter_snippets(snippets: Vec<Snippet>, filter: Option<String>) -> Vec<Snippet> {
    let filter = filter.unwrap_or_default();
    if filter.is_empty() {
        return snippets;       
    }
    let mut filtered = Vec::new();
    for snippet in snippets {
        if snippet.name.contains(&filter)
        || snippet.content.contains(&filter)
        || snippet.tags.contains(&filter)
        || snippet.lang.to_string().contains(&filter)       
        {
            filtered.push(snippet);
        }
        
    }
    filtered
}


pub fn insert_snippet(conn: &mut Connection, snippet: &Snippet) -> Result<(), NibbError> {
    let tx = conn.transaction()?;

    tx.execute(
        "INSERT INTO snippets (name, content, description, lang) VALUES (?1, ?2, ?3, ?4)",
        params![snippet.name, snippet.content, snippet.description, snippet.lang.to_string()]
    )?;
    tx.commit()?;
    for tag in &snippet.tags {
        add_tag_db(conn, &snippet.name, tag)?;       
    }
    Ok(())
}

pub fn get_snippet(conn: &Connection, id: i32) -> Result<Snippet, NibbError> {
    let mut stmt = conn.prepare(
        "SELECT name, content, description, lang FROM snippets WHERE id = ?1"
    )?;
    let mut rows = stmt.query(params![id])?;
    let row = rows.next()?.ok_or_else(|| NibbError::NotFound(id.to_string()))?;
    let name = row.get(0)?;
    let content: String = row.get(1)?;
    let description: Option<String> = row.get(2)?;
    let lang_str: String = row.get(3)?;
    let lang: Lang = Lang::from(lang_str.as_str());
    let mut tag_stmt = conn.prepare(
        "
        SELECT tags.name
        FROM tags
        INNER JOIN snippets_tags ON tags.id = snippets_tags.tag_id
        INNER JOIN snippets ON snippets.id = snippets_tags.snippet_id
        WHERE snippets.name = ?1
        "
    )?;
    let tag_iter = tag_stmt
        .query_map(params![name], |row| {row.get(0)})?;

    let tags: HashSet<String> = tag_iter.filter_map(|r| r.ok()).collect();
    Ok(Snippet {
        name,
        content,
        description,
        id,
        tags,
        lang,
    })
}


pub fn get_snippet_by_name(conn: &Connection, name: &str) -> Result<Snippet, NibbError> {
    let mut stmt = conn.prepare(
        "SELECT id, content, description, lang FROM snippets WHERE name = ?1"
    )?;
    let mut rows = stmt.query(params![name])?;
    let row = rows.next()?.ok_or_else(|| NibbError::NotFound(name.to_string()))?;
    let id = row.get(0)?;
    let content: String = row.get(1)?;
    let description: Option<String> = row.get(2)?;
    let lang_str: String = row.get(3)?;
    let lang = Lang::from(lang_str.as_str());
    let mut tag_stmt = conn.prepare(
        "
        SELECT tags.name
        FROM tags
        INNER JOIN snippets_tags ON tags.id = snippets_tags.tag_id
        INNER JOIN snippets ON snippets.id = snippets_tags.snippet_id
        WHERE snippets.name = ?1
        "
    )?;
    let tag_iter = tag_stmt
        .query_map(params![name], |row| {row.get(0)})?;

    let tags: HashSet<String> = tag_iter.filter_map(|r| r.ok()).collect();
    Ok(Snippet {
        name: name.to_string(),
        content,
        description,
        id,
        tags,
        lang,
    })
}

pub fn init_nibb_db() -> Result<Connection, NibbError> {
    let db_path = get_storage_path()?;
    let conn = Connection::open(db_path)?;
    conn.execute_batch(
        "
            CREATE TABLE IF NOT EXISTS snippets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            content TEXT,
            description TEXT,
            lang TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS snippets_tags (
                snippet_id INTEGER NOT NULL,
                tag_id INTEGER NOT NULL,
                FOREIGN KEY (snippet_id) REFERENCES snippets(id),
                FOREIGN KEY (tag_id) REFERENCES tags(id),
                UNIQUE (snippet_id, tag_id)
            );
            "
    )?;
    Ok(conn)
}
pub fn add_tag_db(conn: &mut Connection, snippet_name: &str, tag: &str) -> Result<(), NibbError> {
    let tx = conn.transaction()?;

    let snippet_id: i64 = tx.query_row(
        "SELECT id FROM snippets WHERE name = ?1",
        params![snippet_name],
        |row| row.get(0),
    )?;

    tx.execute(
        "INSERT OR IGNORE INTO tags (name) VALUES (?1)",
        params![tag],
    )?;

    let tag_id: i64 = tx.query_row(
        "SELECT id FROM tags WHERE name = ?1",
        params![tag],
        |row| row.get(0),
    )?;

    tx.execute(
        "INSERT OR IGNORE INTO snippets_tags (snippet_id, tag_id) VALUES (?1, ?2)",
        params![snippet_id, tag_id],
    )?;

    tx.commit()?;
    Ok(())
}
pub fn rm_tag_db(conn: &mut Connection, snippet_name: &str, tag: &str) -> Result<(), NibbError> {
    let tx = conn.transaction()?;

    let snippet_id: i64 = tx.query_row(
        "SELECT id FROM snippets WHERE name = ?1",
        params![snippet_name],
        |row| row.get(0),
    )?;

    let tag_id: i64 = tx.query_row(
        "SELECT id FROM tags WHERE name = ?1",
        params![tag],
        |row| row.get(0),
    )?;

    tx.execute(
        "DELETE FROM snippets_tags WHERE snippet_id = ?1 AND tag_id = ?2",
        params![snippet_id, tag_id],
    )?;

    tx.commit()?;
    Ok(())
}

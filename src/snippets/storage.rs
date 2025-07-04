#![allow(dead_code)]

use std::collections::HashSet;
use rusqlite::{params, Connection};
use crate::errors::NibbError;
use crate::snippets::snippet::Snippet;
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

pub fn update_snippet(conn: &mut Connection, snippet: &Snippet, name: &str) -> Result<(), NibbError> {
    let tx = conn.transaction()?;

    tx.execute(
        "UPDATE snippets SET name = ?1, content = ?2, description = ?3 WHERE name = ?4",
        params![snippet.name, snippet.content, snippet.description, name],
    )?;

    let snippet_id: i64 = tx.query_row(
        "SELECT id FROM snippets WHERE name = ?1",
        params![snippet.name],
        |row| row.get(0),
    )?;

    tx.execute("DELETE FROM snippets_tags WHERE snippet_id = ?1", params![snippet_id])?;

    for tag in &snippet.tags {
        tx.execute("INSERT OR IGNORE INTO tags (name) VALUES (?1)", params![tag])?;

        let tag_id: i64 = tx.query_row(
            "SELECT id FROM tags WHERE name = ?1",
            params![tag],
            |row| row.get(0),
        )?;

        tx.execute(
            "INSERT OR IGNORE INTO snippets_tags (snippet_id, tag_id) VALUES (?1, ?2)",
            params![snippet_id, tag_id],
        )?;
    }

    tx.commit()?;
    Ok(())
}


pub fn delete_snippet(conn: &mut Connection, name: &str) -> Result<(), NibbError> {
    let tx = conn.transaction()?;

    let snippet_id: i64 = tx.query_row(
        "SELECT id FROM snippets WHERE name = ?1",
        params![name],
        |row| row.get(0),
    )?;

    tx.execute("DELETE FROM snippets_tags WHERE snippet_id = ?1", params![snippet_id])?;

    tx.execute("DELETE FROM snippets WHERE id = ?1", params![snippet_id])?;

    tx.commit()?;
    Ok(())
}
pub fn list_snippets(conn: &Connection, tags: Option<&[String]>) -> Result<Vec<Snippet>, NibbError> {
    let mut snippets = Vec::new();

    let mut stmt = if let Some(tags) = tags {
        let placeholders = vec!["?"; tags.len()].join(",");
        conn.prepare(&format!(
            "
            SELECT DISTINCT s.id, s.name, s.content, s.description, s.path
            FROM snippets s
            JOIN snippets_tags st ON s.id = st.snippet_id
            JOIN tags t ON st.tag_id = t.id
            WHERE t.name IN ({})
            GROUP BY s.id
            HAVING COUNT(DISTINCT t.name) = ?
            ", placeholders
        ))?
    } else {
        conn.prepare(
            "
            SELECT s.id, s.name, s.content, s.description
            FROM snippets s
            "
        )?
    };

    let mut params: Vec<&dyn rusqlite::ToSql> = vec![];
    let tag_count: i64;
    if let Some(t) = tags {
        for tag in t {
            params.push(tag as &dyn rusqlite::ToSql);
        }
        tag_count = t.len() as i64;
        params.push(&tag_count);
    }

    let path = get_storage_path()?;

    let snippet_rows = stmt.query_map(&params[..], |row| {
        Ok((
            row.get::<_, i64>(0)?, // id
            Snippet {
                name: row.get(1)?,
                content: row.get(2)?,
                description: row.get(3)?,
                path: String::from(path.to_string_lossy()),
                tags: HashSet::new(),
            }
        ))
    })?;

    for row in snippet_rows {
        let (id, mut snippet) = row?;
        let mut tag_stmt = conn.prepare(
            "
            SELECT t.name
            FROM tags t
            JOIN snippets_tags st ON t.id = st.tag_id
            WHERE st.snippet_id = ?
            "
        )?;
        let tag_iter = tag_stmt.query_map(params![id], |row| row.get::<_, String>(0))?;

        for tag in tag_iter {
            snippet.tags.insert(tag?);
        }

        snippets.push(snippet);
    }

    Ok(snippets)
}



pub fn insert_snippet(conn: &mut Connection, snippet: &Snippet) -> Result<(), NibbError> {
    let tx = conn.transaction()?;

    tx.execute(
        "INSERT INTO snippets (name, content, description) VALUES (?1, ?2, ?3)",
        params![snippet.name, snippet.content, snippet.description]
    )?;
    tx.commit()?;
    for tag in &snippet.tags {
        add_tag_db(conn, &snippet.name, tag)?;       
    }
    Ok(())
}

pub fn get_snippet(conn: &Connection, name: &str) -> Result<Snippet, NibbError> {
    let mut stmt = conn.prepare(
        "SELECT content, description FROM snippets WHERE name = ?1"
    )?;
    let mut rows = stmt.query(params![name])?;
    let row = rows.next()?.ok_or_else(|| NibbError::NotFound(name.to_string()))?;
    let content: String = row.get(0)?;
    let description: Option<String> = row.get(1)?;
    let path = get_storage_path()?.to_str().unwrap().to_string();
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
        path,
        tags,
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
            description TEXT
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

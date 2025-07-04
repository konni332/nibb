#![allow(dead_code)]
#[derive(Debug)]
pub enum NibbError {
    IoError(std::io::Error),
    NotFound(String),
    ClipboardError(String),
    FSError(String),
    EditorError(String),
    GitError(String),
    ConfigError(String),
    DBError(String),
}

impl From<std::io::Error> for NibbError {
    fn from(e: std::io::Error) -> Self {
        NibbError::IoError(e)
    }
}

impl std::fmt::Display for NibbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { 
            NibbError::IoError(e) => write!(f, "IO Error: {}", e),
            NibbError::NotFound(s) => write!(f, "Not found: {}", s),
            NibbError::ClipboardError(s) => write!(f, "Clipboard Error: {}", s),
            NibbError::FSError(s) => write!(f, "File System Error: {}", s),
            NibbError::EditorError(s) => write!(f, "Editor Error: {}", s),   
            NibbError::GitError(s) => write!(f, "Git Error: {}", s),       
            NibbError::ConfigError(s) => write!(f, "Config Error: {}", s),
            NibbError::DBError(s) => write!(f, "Database Error: {}", s),
        }
    }   
}

impl std::error::Error for NibbError {
    
}

impl From<rusqlite::Error> for NibbError {
    fn from(e: rusqlite::Error) -> Self {
        match e {
            rusqlite::Error::QueryReturnedNoRows => NibbError::NotFound(e.to_string()),
            _ => NibbError::DBError(e.to_string()),
        }
    }
}

impl NibbError {
    pub fn to_json(&self) -> serde_json::Value {
        match self { 
            NibbError::IoError(e) => serde_json::json!({"type": "io", "message": e.to_string()}),
            NibbError::NotFound(s) => serde_json::json!({"type": "not_found", "message": s.to_string()}),
            NibbError::ClipboardError(s) => serde_json::json!({"type": "clipboard", "message": s.to_string()}),
            NibbError::FSError(s) => serde_json::json!({"type": "fs", "message": s.to_string()}),       
            NibbError::EditorError(s) => serde_json::json!({"type": "editor", "message": s.to_string()}),     
            NibbError::GitError(s) => serde_json::json!({"type": "git", "message": s.to_string()}),   
            NibbError::ConfigError(s) => serde_json::json!({"type": "config", "message": s.to_string()}),
            NibbError::DBError(s) => serde_json::json!({"type": "db", "message": s.to_string()}),
        }
    }
}
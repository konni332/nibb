#![allow(dead_code)]
#[derive(Debug)]
pub enum NibbError {
    IoError(std::io::Error),
    NotFound(String),
    ClipboardError(String),
    FSError(String),
    EditorError(String),
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
        }
    }   
}

impl std::error::Error for NibbError {
    
}

impl NibbError {
    pub fn to_json(&self) -> serde_json::Value {
        match self { 
            NibbError::IoError(e) => serde_json::json!({"type": "io", "message": e.to_string()}),
            NibbError::NotFound(s) => serde_json::json!({"type": "not_found", "message": s.to_string()}),
            NibbError::ClipboardError(s) => serde_json::json!({"type": "clipboard", "message": s.to_string()}),
            NibbError::FSError(s) => serde_json::json!({"type": "fs", "message": s.to_string()}),       
            NibbError::EditorError(s) => serde_json::json!({"type": "editor", "message": s.to_string()}),       
        }
    }
}
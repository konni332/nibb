use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use serde::Serialize;
use crate::snippets::storage::{init_nibb_db, list_snippets};

#[derive(Serialize)]
pub struct FFISnippet {
    pub name: String,
    pub content: String,
    pub description: String,
    pub tags: Vec<String>,
}


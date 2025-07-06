mod cli;
mod snippets;
mod utils;
mod config;
mod integration;
mod errors;
mod ffi;

use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
pub use utils::fs::ensure_nibb_structure;

pub use cli::command::NibbCli;
pub use cli::execute::execute;

pub use config::settings::Settings;

pub use snippets::snippet::Snippet;

pub use integration::utils::extern_setup;

pub use snippets::manager::*;
use crate::ffi::utils::FFISnippet;
use crate::integration::git::{nibb_git_post_actions, nibb_git_pre_actions};
use crate::snippets::snippet::Lang;
use crate::snippets::storage::{delete_snippet, filter_snippets, get_snippet, get_snippet_by_name, init_nibb_db, list_snippets, update_snippet};
use crate::utils::fs::{normalize_content};

#[unsafe(no_mangle)]
pub extern "C" fn ffi_list_snippets(filter: *const c_char) -> *mut c_char {
    let filter_str = unsafe {
        if filter.is_null() {
            None
        } else {
            Some(CStr::from_ptr(filter).to_str().unwrap_or("").to_string())
        }
    };

    let conn = match init_nibb_db() {
        Ok(c) => c,
        Err(_) => return std::ptr::null_mut(),
    };

    let result = match list_snippets(&conn) {
        Ok(snippets) => {
            let snippets = filter_snippets(snippets, filter_str);
            let ffi_snippets: Vec<FFISnippet> = snippets.into_iter().map(|s| FFISnippet {
                id: s.id,
                name: s.name.clone(),
                content: normalize_content(&s.content),
                description: s.description.clone().unwrap_or("Empty".to_string()),
                tags: s.tags.clone().into_iter().collect(),
                lang: s.lang.to_string()
            }).collect();
            
            serde_json::to_string(&ffi_snippets).unwrap_or("[]".to_string())
        }
        Err(_) => "[]".to_string(),
    };

    CString::new(result).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn ffi_new_snippet(
    name: *const c_char,
    tags_csv: *const c_char,
    lang: *const c_char,
) -> *mut c_char {
    let name_str = unsafe {
        if name.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(name).to_str().unwrap_or("")
    };
    let lang = unsafe {
        if lang.is_null() {
            Lang::Unknown
        }
        else {
            Lang::from(CStr::from_ptr(lang).to_str().unwrap_or(""))
        }
    };
    let tags_opt = unsafe {
        if tags_csv.is_null() {
            None
        }
        else {
            let tags_str = CStr::from_ptr(tags_csv).to_str().unwrap_or("");
            if tags_str.is_empty() {
                None
            }
            else {
                Some(tags_str.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>())
            }
        }
    };

    let mut conn = match init_nibb_db() {
        Ok(c) => c,
        Err(_) => return std::ptr::null_mut(),
    };

    let result = match new_snippet(name_str.to_string(), tags_opt, lang, &mut conn) {
        Ok(snippet) => {
            let ffi_snippet = FFISnippet {
                id: snippet.id,
                name: name_str.to_string(),
                content: snippet.content,
                description: snippet.description.unwrap_or("Empty".to_string()),
                tags: snippet.tags.into_iter().collect(),
                lang: snippet.lang.to_string()
            };
            serde_json::to_string(&ffi_snippet).unwrap_or_else(|_| "{}".to_string())
        }
        Err(_) => "{}".to_string(),
    };

    CString::new(result).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn ffi_delete_snippet(id: i32) -> bool {

    let mut conn = match init_nibb_db() {
        Ok(c) => c,
        Err(_) => return false,
    };
    match delete_snippet(&mut conn, id) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ffi_get_snippet_by_name(name: *const c_char) -> *mut c_char {
    let name_str = unsafe {
        if name.is_null() {
            return std::ptr::null_mut();
        }
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        }
    };
    let conn = match init_nibb_db() {
        Ok(c) => c,
        Err(_) => return std::ptr::null_mut(),
    };

    let snippet = match get_snippet_by_name(&conn, name_str) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    match serde_json::to_string(&snippet) {
        Ok(s) => CString::new(s).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn ffi_get_snippet(id: i32) -> *mut c_char {

    let conn = match init_nibb_db() {
        Ok(c) => c,
        Err(_) => return std::ptr::null_mut(),
    };

    let snippet = match get_snippet(&conn, id) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    match serde_json::to_string(&snippet) {
        Ok(s) => CString::new(s).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn ffi_update_snippet(
    id: i32,
    new_name: *const c_char,
    content: *const c_char,
    description: *const c_char,
    tags_json: *const c_char,
    lang: *const c_char,
) -> bool {
    let new_name = match c_str_to_str(new_name) {
        Some(s) => s.to_string(),
        None => return false,
    };

    let content = match c_str_to_str(content) {
        Some(s) => s.to_string(),
        None => return false,
    };

    let description = match c_str_to_str(description) {
        Some(s) => s.to_string(),
        None => return false,
    };

    let tags_json = unsafe {
        if tags_json.is_null() { "[]".to_string() }
        else {
            match CStr::from_ptr(tags_json).to_str() {
                Ok(s) => s.to_string(),
                Err(_) => "[]".to_string(),
            }
        }
    };
    let tags: HashSet<String> = match serde_json::from_str(&tags_json) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let lang = match c_str_to_str(lang) {
        Some(s) => Lang::from(s),
        None => Lang::Unknown,
    };
    let mut conn = match init_nibb_db() {
        Ok(c) => c,
        Err(_) => return false,
    };

    let snippet = Snippet {
        name: new_name,
        content,
        description: Some(description),
        tags,
        id,
        lang,
    };
    match update_snippet(&mut conn, &snippet,id) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ffi_fuzzy_search(query: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!query.is_null());
        CStr::from_ptr(query)
    };
    let query = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let conn = match init_nibb_db() {
        Ok(c) => c,
        Err(_) => return std::ptr::null_mut(),
    };
    let snippets = match list_snippets(&conn) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let found = fuzzy_search(&query, &snippets);
    match serde_json::to_string(&found) {
        Ok(s) => CString::new(s).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn ffi_git_post_actions(name: *const c_char) -> i32 {
    let c_str = unsafe { CStr::from_ptr(name) };
    let snippet_name = c_str.to_str().unwrap_or("");
    match extern_setup() {
        Ok(cfg) => {
            let conn = match init_nibb_db() {
                Ok(c) => c,
                Err(_) => return 1,
            };
            match nibb_git_post_actions(snippet_name, &conn, &cfg) {
                Ok(_) => 0,
                Err(_) => 1,
            }
        }
        Err(_) => 1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ffi_git_pre_actions() -> i32 {
    match extern_setup() {
        Ok(cfg) => {
            match nibb_git_pre_actions(&cfg) {
                Ok(_) => 0,
                Err(_) => 1,
            }
        },
        Err(_) => 1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nibb_free_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe { CString::from_raw(s); }
}

fn c_str_to_str<'a>(c_str: *const c_char) -> Option<&'a str> {
    if c_str.is_null() {
        None
    } else {
        unsafe {
            CStr::from_ptr(c_str).to_str().ok()
        }
    }
}
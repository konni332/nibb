use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::fs::get_nibb_dir;
use crate::FSRepo;

pub fn c_str_from_str(s: &str) -> *mut c_char {
    match CString::new(s) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut()
    }
}

pub fn str_from_c_str(s: *const c_char) -> String {
    if s.is_null() {
        return String::new();
    }
    unsafe {
        CStr::from_ptr(s).to_str().unwrap_or("").to_string()
    }
}


pub fn load_repo_ffi() -> Result<FSRepo, *mut c_char> {
    let path = match get_nibb_dir() {
        Ok(path) => path,
        Err(e) => return Err(c_str_from_str(&e.to_json()))
    };

    let repo = match FSRepo::new(path) {
        Ok(repo) => repo,
        Err(e) => return Err(c_str_from_str(&e.to_json()))
    };
    Ok(repo)
}
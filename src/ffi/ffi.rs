use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use slug::slugify;
use crate::ffi::ffi_utils::{c_str_from_str, load_repo_ffi, str_from_c_str};
use crate::fs::get_nibb_dir;
use crate::result::NibbFFIError;
use crate::{FSRepo, Snippet, SnippetRepository};
use crate::git::git_integration::nibb_git_generic;

/// Loads a snippet by name and returns its JSON representation.
///
/// # Arguments
/// - `name`: A null-terminated C string representing the name of the snippet.
///
/// # Returns
/// A newly allocated C string (`*mut c_char`) containing the snippet's JSON representation.
/// - On success: JSON-encoded `Snippet` as a C string (must be freed with `free_string_ffi`).
/// - On failure: JSON-encoded error object (must also be freed).
///
/// # Safety
/// - `name` must be a valid, null-terminated UTF-8 string.
/// - Caller is responsible for freeing the returned string using `free_string_ffi`.
#[unsafe(no_mangle)]
pub extern "C" fn load_snippet_ffi(name: *const c_char) -> *mut c_char {
    let repo = match load_repo_ffi() {
        Ok(repo) => repo,
        Err(e) => return e,
    };

    let name = unsafe { CStr::from_ptr(name) };
    let name = match name.to_str() {
        Ok(name) => name,
        Err(e) => return c_str_from_str(&NibbFFIError::FFIError(e.to_string()).to_json())
    };
    let slug = slugify(name);
    let snippet = match repo.load(&slug) {
        Ok(snippet) => snippet,
        Err(e) => return c_str_from_str(&e.to_json())
    };
    c_str_from_str(&snippet.to_json())
}

/// Saves a single snippet from its JSON representation.
///
/// # Arguments
/// - `snippet_json`: A null-terminated C string containing a JSON-encoded snippet.
///
/// # Returns
/// - `true` if the snippet was saved successfully.
/// - `false` if the input was invalid or saving failed.
///
/// # Safety
/// - `snippet_json` must be a valid, null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub extern "C" fn save_snippet_ffi(snippet_json: *const c_char) -> bool {
    let snippet_json = str_from_c_str(snippet_json);
    let snippet = match serde_json::from_str(&snippet_json) {
        Ok(snippet) => {
            snippet
        }
        Err(_) => {
            return false;
        }
    };
    let repo = match FSRepo::new(get_nibb_dir().unwrap()) {
        Ok(repo) => repo,
        Err(_) => return false
    };
    match repo.save(&snippet) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Deletes a snippet from the repo
///
/// # Arguments
/// - `name`: A Null terminated C String of the snippets' name.
///
/// # Returns
/// - `true` if the snippet was deleted successfully.
/// - `false` if an error occurred.
///
/// # Safety
/// - `name` needs to be a valid, null-terminated, UTF-8 string.
#[unsafe(no_mangle)]
pub extern "C" fn delete_snippet_ffi(name: *const c_char) -> bool {
    let repo = match FSRepo::new(get_nibb_dir().unwrap()) {
        Ok(repo) => repo,
        Err(_) => return false
    };
    let name = str_from_c_str(name);
    match repo.delete(&name) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Loads all snippets from the repository and returns them as a JSON array.
///
/// # Returns
/// A newly allocated C string (`*mut c_char`) containing the JSON array of all snippets.
/// - On success: JSON array of snippets (must be freed with `free_string_ffi`).
/// - On failure: JSON-encoded error object (must also be freed).
///
/// # Safety
/// - Caller is responsible for freeing the returned string using `free_string_ffi`.
#[unsafe(no_mangle)]
pub extern "C" fn load_all_ffi() -> *mut c_char {
    let repo = match load_repo_ffi() {
        Ok(repo) => repo,
        Err(e) => return e,
    };
    let snippets = match repo.load_all() {
        Ok(snippets) => snippets,
        Err(e) => return c_str_from_str(&e.to_json())
    };
    c_str_from_str(
        &serde_json::to_string(&snippets)
            .unwrap_or_else(|_| "{\"type\":\"Other\",\"message\":\"Serialization failed\"}"
                .to_string())
    )
}

/// Saves a list of snippets from a JSON array.
///
/// # Arguments
/// - `snippets_json`: A null-terminated C string containing a JSON array of snippets.
///
/// # Returns
/// - `true` if all snippets were saved successfully.
/// - `false` if deserialization or saving failed.
///
/// # Safety
/// - `snippets_json` must be a valid, null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub extern "C" fn save_all_ffi(snippets_json: *const c_char) -> bool {
    let snippets_json = str_from_c_str(snippets_json);
    let snippets: Vec<Snippet> = match serde_json::from_str(&snippets_json) {
        Ok(snippets) => {
            snippets
        }
        Err(_) => {
            return false;
        }
    };
    let repo = match load_repo_ffi() {
        Ok(repo) => repo,
        Err(_) => return false,
    };
    match repo.save_all(&snippets) {
        Ok(_) => true,
        Err(_) => false,
    }

}

/// Executes a generic Git command inside the `.nibb` directory and returns the output as JSON.
///
/// # Arguments
/// - `args`: A pointer to a null-terminated C string containing the Git arguments as a whitespace-separated string, e.g. `"status -s"`.
///
/// # Returns
/// - A pointer to a null-terminated C string containing a JSON object:
///   ```json
///   {
///     "stdout": "<Git stdout output>",
///     "stderr": "<Git stderr output>"
///   }
///   ```
///   - In case of error, `stderr` contains an error message and `stdout` is empty.
///
/// # Safety
/// - The input C string must be valid and null-terminated.
/// - The returned string must be freed by the caller using the appropriate function (e.g., `free_string_ffi`).
///
/// # Example (from Lua)
/// ```lua
/// local output_json = ffi.C.nibb_git_generic_ffi("status -s")
/// local output = vim.fn.json_decode(ffi.string(output_json))
/// print(output.stdout)
/// print(output.stderr)
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn nibb_git_generic_ffi(args: *const c_char) -> *const c_char{
    let args = str_from_c_str(args);
    let args: Vec<String> = args.split_whitespace().map(|s| s.to_string()).collect();
    let out = nibb_git_generic(args).unwrap_or_else(|e| e.to_json());
    c_str_from_str(&out)
}

/// Frees a string previously allocated and returned by an FFI function.
///
/// # Arguments
/// - `s`: A pointer returned by an FFI function like `load_snippet_ffi` or `load_all_ffi`.
///
/// # Safety
/// - `s` must be a pointer obtained from one of the FFI functions using `CString::into_raw`.
/// - Passing a null pointer is safe and does nothing.
/// - After calling this function, `s` must not be used again.
#[unsafe(no_mangle)]
pub extern "C" fn free_string_ffi(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        drop(CString::from_raw(s));
    }
}

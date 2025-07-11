pub mod snippets;
mod result;
pub mod fs;
pub mod ffi;
mod git;
mod config;

// === Lib ===
// ---
pub use fs::get_nibb_dir;

pub use snippets::repo::{FSRepo, SnippetRepository};

pub use snippets::snippet::{Meta, Visibility, Snippet};

pub use snippets::file_type::FileType;

pub use ffi::ffi::{
    load_all_ffi,
    save_all_ffi,
    load_snippet_ffi,
    save_snippet_ffi,
    free_string_ffi,
    delete_snippet_ffi
};
// ---
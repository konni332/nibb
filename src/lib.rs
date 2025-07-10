pub mod snippets;
mod result;
mod fs;
mod cli;
pub mod ffi;

// === CLI ===
// ---
pub use cli::cli::execute_cli;
pub use cli::arguments::Arguments;
// ---

// === Lib ===
// ---
pub use snippets::repo::{FSRepo, SnippetRepository};

pub use snippets::snippet::{Meta, Visibility, Snippet};

pub use snippets::file_type::FileType;

pub use ffi::ffi::{load_all_ffi, save_all_ffi, load_snippet_ffi, save_snippet_ffi, free_string_ffi};
// ---
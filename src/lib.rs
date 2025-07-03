mod cli;
mod snippets;
mod utils;
mod config;
mod integration;
mod errors;


pub use utils::fs::ensure_nibb_structure;

pub use cli::command::NibbCli;
pub use cli::execute::execute;

pub use config::settings::Settings;

pub use snippets::manager::*;
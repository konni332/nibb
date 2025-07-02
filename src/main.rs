use crate::cli::command::{parse_cli, Commands, NibbCli};
use crate::cli::execute::{execute, new_snippet};
use crate::utils::fs::ensure_nibb_structure;

mod cli;
mod snippets;
mod utils;
mod config;
mod integration;


fn main() {
    // parse command line input
    let cli = parse_cli();
    // ensure the necessary files and directories are in place
    ensure_nibb_structure().map_err(|e| println!("{:?}", e))
        .expect("Failed to ensure nibb structure");
    // execute commands
    execute(cli);
}


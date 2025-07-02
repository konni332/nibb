use clap::Parser;
use crate::cli::command::{NibbCli};
use crate::cli::execute::{execute};
use crate::utils::fs::ensure_nibb_structure;

mod cli;
mod snippets;
mod utils;
mod config;
mod integration;


fn main() {
    // parse command line input
    let cli = NibbCli::parse();
    // ensure the necessary files and directories are in place
    ensure_nibb_structure().map_err(|e| println!("{:?}", e))
        .expect("Failed to ensure nibb structure");
    // execute commands
    execute(cli);
}


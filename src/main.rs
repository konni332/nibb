use clap::Parser;
use anyhow::{Context, Result};
use nibb::{ensure_nibb_structure, execute, NibbCli};

mod cli;
mod snippets;
mod utils;
mod config;
mod integration;
mod errors;

fn main() -> Result<()>{
    // parse command line input
    let cli = NibbCli::parse();
    // ensure the necessary files and directories are in place
    ensure_nibb_structure().map_err(|e| println!("{:?}", e))
        .expect("Failed to ensure nibb structure");
    // execute commands
    execute(cli)
}


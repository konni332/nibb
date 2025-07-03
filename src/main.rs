use clap::Parser;
use anyhow::{Context, Result};
use nibb::*;

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
    ensure_nibb_structure().with_context(|| "Failed to ensure nibb structure")?;
    // load config
    let cfg = Settings::load().with_context(|| "Failed to load config")?;
    // execute commands
    execute(cli, cfg)
}


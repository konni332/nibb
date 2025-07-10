use clap::Parser;
use anyhow::Result;
mod cli;
use crate::cli::cli::execute_cli;
use crate::cli::arguments::Arguments;

fn main() -> Result<()>{
    let cli = Arguments::parse();
    execute_cli(cli)
}


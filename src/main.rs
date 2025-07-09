use clap::Parser;
use anyhow::Result;
use nibb_core::Arguments;
use nibb_core::execute_cli;


fn main() -> Result<()>{
    let cli = Arguments::parse();
    execute_cli(cli)
}


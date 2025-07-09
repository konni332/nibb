use clap::{Parser, Subcommand};
use crate::snippets::file_type::FileType;

#[derive(Parser, Debug, Clone)]
#[command(
name = "nibb",
about = "A simple and easy snippet engine, for CLI or editor integration",
author = "konni332",
version = "0.1.0",
)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: NibbCommand,

}
#[derive(Subcommand, Debug, Clone)]
pub enum NibbCommand {
    New {
        name: String,
        #[clap(short, long)]
        description: Option<String>,
        #[clap(short, long)]
        content: String,
        #[clap(short, long)]
        language: Option<String>,
        #[clap(short, long, value_delimiter = ',')]
        tags: Vec<String>,
        #[clap(short, long)]
        public: bool,
    },
    List {
        #[clap(short, long)]
        filter: Option<String>,
        #[clap(short, long)]
        json: bool,
    },
    Delete {
        name: String,
    },
    Edit {
        name: String,
        key: String,
        value: String,
    },
}
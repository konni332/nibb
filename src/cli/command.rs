use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "nibb")]
#[command(about = "Nibb - the friendly CLI for snippets of all kinds")]
#[clap(version = "0.1.0")]
pub struct NibbCli{
    #[command(subcommand)]
    pub command: Commands,
    #[clap(short, long, global = true)]
    pub verbose: bool,
    #[clap(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands{
    /// Create a new Snippet
    Create {
        name: String,
        #[clap(short, long)]
        tags: Option<Vec<String>>,
    },
    /// Insert a snippet at the specified position
    Insert {
        name: String,
        #[clap(short, long)]
        file: Option<String>,
        #[arg(value_enum)]
        at: Position,
    },
    /// Edit a snippets' content in the terminal, using the configured editor
    Edit {
        name: String,
    },
    /// Rename a snippet
    Rename {
        old_name: String,
        new_name: String,
    },
    /// List all snippets that conform with the specified tags
    List {
        #[clap(short, long)]
        tags: Option<Vec<String>>,
        #[clap(long)]
        json: bool,
    },
    /// Load snippets from the specified Markdown file
    #[clap(alias = "md")]
    LoadMd {
        file: String,
    },
    /// Configure Nibb
    Config {
        #[arg(value_enum)]
        op: ConfigOp,
        key: String,
        value: Option<String>,
    },
    /// Delete a snippet
    Delete {
        name: String,
    }
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum Position {
    Marker,
    Cursor,
    Start,
    End,
    Clipboard,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum ConfigOp {
    Get,
    Set,
    Reset,
}
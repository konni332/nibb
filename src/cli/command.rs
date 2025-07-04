use std::fmt::Display;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug, Clone)]
#[command(name = "nibb")]
#[command(about = "Nibb - the friendly CLI for snippets of all kinds")]
#[clap(version = "0.2.0")]
pub struct NibbCli{
    #[command(subcommand)]
    pub command: Commands,
    #[clap(short, long, global = true)]
    pub verbose: bool,
    #[clap(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands{
    /// Create a new Snippet
    New {
        name: String,
        #[clap(short, long)]
        tags: Option<Vec<String>>,
        #[clap(short, long, conflicts_with = "file")]
        clip: bool,
        #[clap(short, long, conflicts_with = "clip")]
        file: Option<String>,
    },
    /// Insert a snippet at the specified position
    Insert {
        name: String,
        file: String,
        #[arg(value_enum, default_value = "clipboard")]
        at: Position,
    },
    /// Copies the snippets' content to the systems clipboard
    Cpy {
        name: String,
    },
    /// Edit a snippets' content in the terminal, using the configured editor
    Edit {
        #[clap(short, long)]
        clip: bool,
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
    },
    /// Configure Nibb
    Config {
        #[clap(subcommand)]
        op: ConfigOp,
    },
    /// Delete a snippet
    Delete {
        name: String,
    },
    /// Adds the specified tags to an existing snippet
    Tag {
        #[arg(value_enum)]
        op: TagOp,
        name: String,
        tags: Vec<String>,
    },
    /// Fuzzy search in snippets
    Fuzz {
        query: String,
    },
    /// Execute git commands in the .nibb directory. 
    #[clap(trailing_var_arg = true)]
    Git {
        /// Arguments, passed to git.
        #[clap(num_args = 1)]
        git_args: Vec<String>,
    },
    /// Export the snippets as a JSON file.
    Export {
        #[clap(short, long = "file-name")]
        file_name: Option<String>,
        #[clap(short, long)]
        pretty: bool
    },
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum Position {
    /// Insert at markers in your file. The Marker can be changed in the configuration file
    /// or using the config subcommand
    Marker,
    /// Not supported via CLI. Use an integration in your editor to insert at cursor
    Cursor,
    Start,
    End,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Marker => write!(f, "marker"),
            Position::Cursor => write!(f, "cursor"),
            Position::Start => write!(f, "start"),
            Position::End => write!(f, "end"),
        }
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum ConfigOp {
    Get{
        key: String
    },
    Set{
        key: String,
        value: String,
    },
    Reset{
        key: Option<String>,
    },
}

impl Display for ConfigOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { 
            ConfigOp::Get {..} => write!(f, "get"),
            ConfigOp::Set {..} => write!(f, "set"),
            ConfigOp::Reset {..} => write!(f, "reset"),
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum TagOp {
    Add,
    Rm,
}
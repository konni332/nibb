use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug, Clone)]
#[command(
name = "nibb",
about = "A simple and easy snippet engine. Written in Rust.",
author = "konni332",
version = "0.5.0",
)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: NibbCommand,

}
#[derive(Subcommand, Debug, Clone)]
pub enum NibbCommand {
    /// Create a new snippet
    New {
        /// Name
        name: String,
        /// Description
        #[clap(short, long)]
        description: Option<String>,
        /// Content
        #[clap(short, long)]
        content: String,
        /// Snippet language/file-type
        #[clap(short, long)]
        language: Option<String>,
        /// Tags
        #[clap(short, long, value_delimiter = ',')]
        tags: Vec<String>,
        /// Initialize the snippet as public
        #[clap(short, long)]
        public: bool,
    },
    /// List snippets
    List {
        /// Filter the snippets. Lists snippets with their content, description, tags or name
        /// containing the filter item. If a valid (`chrono::NaiveDate`) Timestamp is given,
        /// it will list all snippets created before that time.
        #[clap(short, long)]
        filter: Option<String>,
        /// Output the results in JSON format, instead of a printed list. Will be written to stdout.
        #[clap(short, long)]
        json: bool,
    },
    /// Delete a snippet
    Delete {
        /// Name
        name: String,
    },
    /// Edit an existing snippet
    Edit {
        /// Name
        name: String,
        /// Key you want to edit
        #[clap(value_enum)]
        key: SnippetKey,
        /// New value
        value: String,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SnippetKey {
    Name,
    Description,
    Content,
    Language,
    Tags,
    Visibility,
}
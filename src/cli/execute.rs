use crate::cli::command::{Commands, NibbCli};
use crate::snippets::snippet::Snippet;
use crate::snippets::storage::{load_snippets, save_snippets};

pub fn new_snippet(name: String, tags: Option<Vec<String>>) {
    let mut snippets = load_snippets();
    let snippet = Snippet::create(name, tags);
    snippets.push(snippet);
    save_snippets(&snippets);
}

pub fn list_snippets(tags: Option<Vec<String>>, json: bool) {
    let snippets = load_snippets();
    let tags = tags.unwrap_or(vec![]);
    println!("=== Snippets ===");
    for snippet in snippets.iter() {
        if snippet.tags.iter().any(|tag| tags.contains(tag)) || tags.is_empty(){
            println!("{}", snippet.pretty());
        }
    }
}
pub fn execute(cli: NibbCli) {
    match cli.command {
        Commands::Create { name, tags } => {
            println!("Create {:?} {:?}", name, tags);
            new_snippet(name, tags);
        }
        Commands::List { tags, json } => {
            list_snippets(tags, json);
        }
        _ => {
            println!("Command {:?}", cli.command)
        }
    }
}

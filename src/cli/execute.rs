use crossterm::style::Stylize;
use crate::cli::command::{Commands, NibbCli, Position};
use crate::snippets::snippet::Snippet;
use crate::snippets::storage::{load_snippets, save_snippets};

fn new_snippet(name: String, tags: Option<Vec<String>>) {
    let mut snippets = load_snippets();
    let snippet = Snippet::create(name, tags);
    snippets.push(snippet);
    save_snippets(&snippets);
}

fn list_snippets(tags: Option<Vec<String>>, json: bool, verbose: bool) {
    let snippets = load_snippets();
    let tags = tags.unwrap_or(vec![]);
    if snippets.len() > 0 {println!("{}", "=== Snippets ===".bold().white());}
    else { println!("{}", "No snippets found".bold().yellow()); }
    for snippet in snippets.iter() {
        if snippet.tags.iter().any(|tag| tags.contains(tag)) || tags.is_empty(){
            snippet.pretty_print(verbose);
        }
    }
}

fn rename_snippet(old_name: String, new_name: String) {
    let mut snippets = load_snippets();
    for snippet in snippets.iter_mut() {
        if snippet.name == old_name {
            snippet.name = new_name.to_string();
        }
    }
    save_snippets(&snippets);
}

fn delete_snippet(name: String) {
    let mut snippets = load_snippets();
    snippets.retain(|snippet| snippet.name != name);
    save_snippets(&snippets);
}

fn insert_snippet(name: String, file: Option<String>, at: Position) {
    let mut snippets = load_snippets();
    todo!()
}

fn edit_snippet(name: String) {
    todo!()
}

pub fn execute(cli: NibbCli) {
    match cli.command {
        Commands::Create { name, tags } => {
            println!("Create {:?} {:?}", name, tags);
            new_snippet(name, tags);
        }
        Commands::List { tags, json } => {
            list_snippets(tags, json, cli.verbose);
        }
        Commands::Rename {old_name, new_name} => {
            rename_snippet(old_name, new_name);
        }
        Commands::Delete {name} => {
            delete_snippet(name);       
        }
        Commands::Insert {name, file, at} => {
            insert_snippet(name, file, at);       
        }
        Commands::Edit {name} => {
            edit_snippet(name);      
        }
        _ => {
            println!("Command {:?}", cli.command)
        }
    }
}

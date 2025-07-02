use std::io::Write;
use std::process::Command;
use crossterm::style::Stylize;
use tempfile::NamedTempFile;
use crate::cli::command::{Commands, NibbCli, Position};
use crate::snippets::snippet::Snippet;
use crate::snippets::storage::{load_snippets, save_snippets};
use anyhow::{anyhow, bail, Context, Result};
use crate::snippets::manager::{get_snippet, insert_to_clipboard, insert_to_file_end, insert_to_file_start};

fn new_snippet(name: String, tags: Option<Vec<String>>) -> Result<()> {
    let mut snippets = load_snippets()?;
    let snippet = Snippet::create(name, tags);
    snippets.push(snippet);
    save_snippets(&snippets)?;
    Ok(())
}

fn list_snippets(tags: Option<Vec<String>>, json: bool, verbose: bool) -> Result<()> {
    let snippets = load_snippets()?;
    let tags = tags.unwrap_or(vec![]);
    if snippets.len() > 0 {println!("{}", "=== Snippets ===".bold().white());}
    else { println!("{}", "No snippets found".bold().yellow()); }
    for snippet in snippets.iter() {
        if snippet.tags.iter().any(
            |tag| tags.contains(tag)) || tags.is_empty() || snippet.name.contains(&tags[0]) {
            snippet.pretty_print(verbose);
        }
    }
    Ok(())
}

fn rename_snippet(old_name: String, new_name: String) -> Result<()> {
    let mut snippets = load_snippets()?;
    for snippet in snippets.iter_mut() {
        if snippet.name == old_name {
            snippet.name = new_name.to_string();
        }
    }
    save_snippets(&snippets)?;
    Ok(())
}

fn delete_snippet(name: String) -> Result<()> {
    let mut snippets = load_snippets()?;
    let old_len = snippets.len();
    snippets.retain(|snippet| snippet.name != name);
    save_snippets(&snippets)?;
    if snippets.len() < old_len {
        println!("Snippet '{}' deleted", name);
    }
    else {
        bail!(format!("Snippet '{}' not found!", name))      
    }
    Ok(())
}

fn insert_snippet(name: String, file: Option<String>, at: Position) -> Result<()>{
    let file = if at != Position::Clipboard {
        file.as_ref().ok_or_else(|| anyhow!("No file specified! Can not insert snippet."))?
    }
    else {
        &"".to_string()
    };
    match at { 
        Position::Clipboard => {
            insert_to_clipboard(&name)?;
            println!("Snippet '{}' copied to clipboard", name);
        },
        Position::End => {
            insert_to_file_end(&name, file)?;
            println!("Snippet '{}' inserted at end of file", name);       
        }
        Position::Start => {
            insert_to_file_start(&name, file)?;
            println!("Snippet '{}' inserted at start of file", name);       
        },
        Position::Cursor => {
            println!("Snippet '{}' inserted at cursor in file", name);
        },
        Position::Marker => {
            println!("Snippet '{}' inserted at marker[s] in file", name);       
        },
        _ => {
            bail!("Unknown position: {}", at)
        }
    }
    Ok(())
}

fn edit_snippet(name: String, editor: &str) -> Result<()>{
    let mut snippets = load_snippets()?;
    let index = snippets.iter().position(|s| s.name == name);
    if let Some(i) = index {
        let mut temp_file = NamedTempFile::new()
            .context("Error: Could not create temp file")?;
        write!(temp_file, "{}", snippets[i].content).context("Error: Could not write to temp file")?;
        
        let status = Command::new(editor)
            .arg(temp_file.path())
            .status()
            .context("Error: Could not execute editor")?;
        
        if !status.success() {
            bail!(format!("Editor did not finish successfully: {}::{}", &editor, status));
        }
        
        let new_content = std::fs::read_to_string(temp_file.path())
            .expect("Error: Could not read from temp file");
        snippets[i].content = new_content;
        save_snippets(&snippets)?;
        Ok(())       
    }
    else { 
        bail!(format!("Snippet '{}' not found!", name))      
    }
}

pub fn execute(cli: NibbCli) -> Result<()>{
    match cli.command {
        Commands::Create { name, tags } => {
            println!("Create {:?} {:?}", name, tags);
            new_snippet(name, tags)?;
        }
        Commands::List { tags, json } => {
            list_snippets(tags, json, cli.verbose)?;
        }
        Commands::Rename {old_name, new_name} => {
            rename_snippet(old_name, new_name)?;
        }
        Commands::Delete {name} => {
            delete_snippet(name)?;       
        }
        Commands::Insert {name, file, at} => {
            insert_snippet(name, file, at)?;       
        }
        Commands::Edit {name} => {
            let editor = "nvim"; // TODO: get editor from env or config or cli
            edit_snippet(name, editor)?;      
        }
        _ => {
            println!("Command {:?}", cli.command)
        }
    }
    Ok(())
}

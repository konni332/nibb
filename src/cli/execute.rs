#![allow(dead_code)]

use crate::cli::command::{Commands, NibbCli};
use anyhow::{Result};
use crossterm::style::Stylize;
use dialoguer::MultiSelect;
use crate::snippets::manager::{
    new_snippet,
    rename_snippet,
    delete_snippet,
    list_snippets,
    insert_snippet,
    edit_snippet
};
use crate::snippets::snippet::Snippet;

pub fn prompt_markers_cli(marker_lines: &[usize], lines: &[&str]) -> Result<Vec<usize>, std::io::Error> {
    if marker_lines.is_empty() {
        return Ok(vec![])
    }
    let items = marker_lines.iter().map(|i| format!("Line{}: {}", i + 1, lines[*i])).collect::<Vec<String>>();
    
    let selections = MultiSelect::new()
        .with_prompt("Select markers to be replaced")
        .items(&items)
        .interact()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    let selected_lines = selections
        .into_iter()
        .map(|idx| marker_lines[idx])
        .collect();
    
    Ok(selected_lines)
}

fn print_snippet_list(snippets: &[Snippet], verbose: bool) {
    if snippets.is_empty() {
        println!("{}", "No snippets found".yellow());
        return;
    }
    println!("{}", "=== Snippets ===".bold());
    for snippet in snippets {
        snippet.pretty_print(verbose);
    }   
}

pub fn execute(cli: NibbCli) -> Result<()>{
    match cli.command {
        Commands::Create { name, tags } => {
            println!("Create {:?} {:?}", name, tags);
            new_snippet(name, tags)?;
        }
        Commands::List { tags, .. } => {
            let snippets = list_snippets(tags)?;
            print_snippet_list(&snippets, cli.verbose);
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

#![allow(dead_code)]

use crate::cli::command::{Commands, ConfigOp, NibbCli, Position};
use anyhow::{anyhow, bail, Context, Result};
use crossterm::style::Stylize;
use dialoguer::MultiSelect;
use crate::config::settings::Settings;
use crate::errors::NibbError;
use crate::snippets::manager::{
    new_snippet,
    rename_snippet,
    delete_snippet,
    list_snippets,
    edit_snippet,
    insert_to_clipboard,
    insert_to_file_end,
     insert_to_file_start
};
use crate::snippets::snippet::Snippet;
use crate::snippets::manager::insert_to_file_marker;

pub fn prompt_markers_cli(marker_lines: &[usize]) -> Result<Vec<usize>, std::io::Error> {
    if marker_lines.is_empty() {
        return Ok(vec![])
    }
    let items = marker_lines.iter()
        .map(|i| format!("Line {}", i + 1)).collect::<Vec<String>>();
    
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

/// Execute a CLI command
pub fn execute(cli: NibbCli, mut cfg: Settings) -> Result<()>{
    match cli.command {
        Commands::Create { name, tags } => {
            if !cli.quiet {println!("Create {:?} {:?}", name, tags.clone().unwrap_or(vec![]));}
            new_snippet(name, tags)?;
        }
        Commands::List { tags, .. } => {
            let snippets = list_snippets(tags)?;
            print_snippet_list(&snippets, cli.verbose);
        }
        Commands::Rename {old_name, new_name} => {
            if !cli.quiet {println!("Rename {:?} {:?}", old_name, new_name);}
            rename_snippet(old_name, new_name)?;
        }
        Commands::Delete {name} => {
            delete_snippet(name)?;       
        }
        Commands::Insert {name, file, at} => {
            insert_snippet(name, file, at, &cfg)?;
        }
        Commands::Edit {name} => {
            let editor = cfg.editor();
            edit_snippet(name, editor)?;      
        }
        Commands::Config {op, key, value} => {
            match op { 
                ConfigOp::Set => {
                    if let Some(val) = value {
                        cfg.set(&key, &val)?;
                    }
                    else {
                        bail!("No value specified for config")
                    }
                },
                ConfigOp::Get => {
                    let val = cfg.get(&key).with_context(|| "Error getting config value: ")?;
                    println!("{}: {}", key, val);
                },
                ConfigOp::Reset => {
                    cfg.reset(Some(&key))?;
                    if !cli.quiet {println!("Config reset: {}", key);}
                },
            }
            cfg.save()?;       
        }
        _ => {
            println!("Command {:?}", cli.command)
        }
    }
    Ok(())
}

/// Genric insert function that matches the given Position and calls the appropriate insert function
pub fn insert_snippet(name: String, file: Option<String>, at: Position, cfg: &Settings) -> std::result::Result<(), NibbError> {
    let file = if at != Position::Clipboard {
        if let Some(file) = file {
            file
        }
        else {
            return Err(NibbError::FSError("No file specified for insertion".to_string()))
        }
    }
    else {
        "".to_string()
    };
    match at {
        Position::Clipboard => {
            insert_to_clipboard(&name)?;
            println!("Snippet '{}' copied to clipboard", name);
        },
        Position::End => {
            insert_to_file_end(&name, &file)?;
            println!("Snippet '{}' inserted at end of file", name);
        }
        Position::Start => {
            insert_to_file_start(&name, &file)?;
            println!("Snippet '{}' inserted at start of file", name);
        },
        Position::Marker => {
            insert_to_file_marker(&name, &file, cfg.marker(), prompt_markers_cli)?;
        },
        Position::Cursor => {
            eprintln!("Cursor inserts are not available in CLI. Use a editor integration instead.")
        },
    }
    Ok(())
}

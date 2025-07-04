#![allow(dead_code)]

use crate::cli::command::{Commands, ConfigOp, NibbCli, Position, TagOp};
use anyhow::{Context, Result};
use crossterm::style::Stylize;
use dialoguer::MultiSelect;
use rusqlite::Connection;
use crate::config::settings::Settings;
use crate::errors::NibbError;
use crate::integration::git::{nibb_git, nibb_git_post_actions, nibb_git_pre_actions};
use crate::snippets::manager::{
    new_snippet,
    rename_snippet,
    edit_snippet,
    insert_to_clipboard,
    insert_to_file_end,
    insert_to_file_start,
    remove_tag,
    add_tag,
    fuzzy_search,
};
use crate::snippets::snippet::Snippet;
use crate::snippets::manager::insert_to_file_marker;
use crate::snippets::storage::{delete_snippet, export_snippets, get_snippet, init_nibb_db, list_snippets, update_snippet};
use crate::utils::clipboard::paste_from_clipboard;

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

fn print_snippet_list(snippets: &[&Snippet], verbose: bool) {
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
    nibb_git_pre_actions(&cfg)?;
    let mut changed: Option<String> = None;
    let mut conn = init_nibb_db()?;
    match cli.command {
        Commands::New { name, tags, clip, file } => {
            changed = Some(name.clone());
            if !cli.quiet {println!("Create {:?} {:?}", name, tags.clone().unwrap_or(vec![]));}
            new_snippet(name.clone(), tags, &mut conn)?;
            if clip {
                let content = paste_from_clipboard()?;
                let mut snippet = get_snippet(&conn, &name)?;
                snippet.content = content;
            }
            if file.is_some() {
                let content = std::fs::read_to_string(&file.unwrap())?;
                let mut snippet = get_snippet(&conn, &name)?;
                snippet.content = content;
            }
        }
        Commands::List { tags } => {
            let snippets = list_snippets(&conn, None)?;
            let snippets = filter_snippets(&snippets, tags);
            print_snippet_list(&snippets, cli.verbose);
            return Ok(()) // nothing was modified, no saves to disk necessary
        }
        Commands::Rename {old_name, new_name} => {
            changed = Some(new_name.clone());
            if !cli.quiet {println!("Rename {:?} {:?}", old_name, new_name);}
            rename_snippet(old_name, new_name, &mut conn)?;
        }
        Commands::Delete {name} => {
            changed = Some(name.clone());
            delete_snippet(&mut conn, &name)?;       
        }
        Commands::Insert {name, file, at} => {
            if !cli.quiet {println!("Insert {} at {} in {}", name, at, file);}
            dbg!(std::env::current_dir()?);
            insert_snippet(&mut conn, name, file, at, &cfg)?;
        }
        Commands::Cpy {name} => {
            insert_to_clipboard(&name, &conn)?;      
        }
        Commands::Tag {op, name, tags} => {
            changed = Some(name.clone());
            match op { 
                TagOp::Add => {
                    for tag in &tags {
                        add_tag(&mut conn, &name, tag)?;
                    }
                },
                TagOp::Rm => {
                    for tag in &tags {
                        remove_tag(&mut conn, &name, tag)?;
                    }   
                },
            }
        }
        Commands::Edit {name, clip} => {
            changed = Some(name.clone());
            let editor = cfg.editor();
            let mut snippet = get_snippet(&conn, &name)?;
            edit_snippet(&mut snippet, editor, clip)?;
            let name = &snippet.name;
            update_snippet(&mut conn, &snippet, name)?;
        }
        Commands::Config {op} => {
            match op { 
                ConfigOp::Set { key, value } => {
                    cfg.set(&key, &value)?;
                    if !cli.quiet {println!("Set config key: {} to: {}", key, value);}  
                },
                ConfigOp::Get { key } => {
                    let val = cfg.get(&key).with_context(|| "Error getting config value: ")?;
                    println!("{}: {}", key, val);
                    return Ok(()); // nothing is mutated, so no saves are necessary
                },
                ConfigOp::Reset { key }=> {
                    let key = key.unwrap_or("all".to_string());
                    cfg.reset(Some(key.clone()))?;
                    if !cli.quiet {println!("Reset config key: {}", key);}
                },
            }
            cfg.save()?;
            return Ok(());
        }
        Commands::Fuzz { query } => {
            let snippets = list_snippets(&conn, None)?;
            let found = fuzzy_search(&query, &snippets);
            print_snippet_list(found.as_slice(), cli.verbose);
        }
        Commands::Git { git_args } => {
            nibb_git(git_args, true)?; // direct git commands should always be verbose!
        }
        Commands::Export { file_name, pretty } => {
            changed = Some(String::from("backup"));
            let snippets = list_snippets(&conn, None)?;
            export_snippets(&snippets, file_name, pretty)?;
        }
    }
    match changed {
        Some(name) => {
            nibb_git_post_actions(&name, &conn, &cfg)?;
        },
        None => {},
    }
    Ok(())
}

/// Genric insert function that matches the given Position and calls the appropriate insert function
pub fn insert_snippet(
    conn: &mut Connection,
    name: String,
    file: String,
    at: Position,
    cfg: &Settings
) -> std::result::Result<(), NibbError> {
    match at {
        Position::End => {
            insert_to_file_end(&name, &file, conn)?;
            println!("Snippet '{}' inserted at end of file", name);
        }
        Position::Start => {
            insert_to_file_start(&name, &file, conn)?;
            println!("Snippet '{}' inserted at start of file", name);
        },
        Position::Marker => {
            insert_to_file_marker(&name, &file, cfg.marker(), conn, prompt_markers_cli)?;
        },
        Position::Cursor => {
            eprintln!("Cursor inserts are not available in CLI. Use a editor integration instead.")
        },
    }
    Ok(())
}

fn filter_snippets(snippets: &[Snippet], tags: Option<Vec<String>>) -> Vec<&Snippet> {
    let mut filtered = Vec::new();
    let tags = tags.unwrap_or(vec![]);
    for snippet in snippets {
        if tags.is_empty() || snippet.tags.iter().any(|t| tags.contains(t)){
            filtered.push(snippet);
        }
    }
    filtered
}

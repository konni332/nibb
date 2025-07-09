use crate::cli::arguments::{Arguments, NibbCommand};
use anyhow::{bail, Context, Result};
use chrono::Utc;
use colored::Colorize;
use crate::fs::get_nibb_dir;
use crate::snippets::file_type::FileType;
use crate::snippets::repo::{FSRepo, SnippetRepository};
use crate::snippets::snippet::{Meta, Snippet, Visibility};
use crate::snippets::utils::filter_snippets;

pub fn execute_cli(cli_args: Arguments) -> Result<()> {
    let repo = FSRepo::new(get_nibb_dir()?).with_context(|| "Failed to create repo")?;

    match cli_args.command {
        NibbCommand::New {
            name,
            description,
            content,
            language,
            tags,
            public
        } => {
            cli_new(&repo, name, description, content, language, tags, public)?;
        }
        NibbCommand::List {filter, json } => {
            cli_list(&repo, filter, json)?;
        }
        NibbCommand::Edit {name, key, value} => {
            println!("unimplemented");
        }
        NibbCommand::Delete {name} => {
            repo.delete(&name)?;
        }
    }
    Ok(())
}

fn cli_list(repo: &FSRepo, filter: Option<String>, json: bool) -> Result<()> {
    let snippets = filter_snippets(
        repo.load_all()?,
        &filter.unwrap_or("".to_string())
    );
    if json {
        if snippets.is_empty() {
            println!("[]");
            return Ok(());
        }
        println!(
            "{}",
            serde_json::to_string_pretty(&snippets)
                .with_context(|| "Failed to serialize snippets to JSON")?
        );
        Ok(())
    }
    else {
        if snippets.is_empty() {
            #[cfg(feature = "ansi")]
            println!("{}", "No snippets found".bold().yellow());
            #[cfg(not(feature = "ansi"))]
            println!("No snippets found");
            return Ok(());
        }
        #[cfg(feature = "ansi")]
        println!("{}", "Snippets:".bold().green());
        #[cfg(not(feature = "ansi"))]
        println!("Snippets:");
        for snippet in snippets {
            let display = format!("{}", snippet);
            for line in display.lines() {
                println!("  {}", line);
            }
        }
        Ok(())
    }
}

fn cli_new(
    repo: &FSRepo,
    name: String,
    description: Option<String>,
    content: String,
    language: Option<String>,
    tags: Vec<String>,
    public: bool,
) -> Result<()> {
    let meta = Meta::new(
        name.clone(),
        description.unwrap_or("".to_string()),
        tags,
        FileType::from(language.unwrap_or("".to_string()).as_str()),
        if public { Some(Visibility::Public) } else { Some(Visibility::Private) }
    );
    let new_snippet = Snippet::new(meta, content);
    return repo.save(&new_snippet).with_context(|| format!("Failed to save snippet: {}", name))
}

fn cli_edit(repo: &FSRepo, name: String, key: String, value: String) -> Result<()> {
    let mut snippet = repo.load(&name)?;
    match key.as_str() {
        "name" => {
            repo.delete(&snippet.meta.name)?;
            snippet.meta.name = value;
        }
        "tags" => {
            snippet.meta.tags = value.split(",").map(|s| s.to_string()).collect();
        }
        "content" => {
            snippet.content = value;
        }
        "language" => {
            snippet.meta.language = FileType::from(value.as_str());
        },
        "visibility" => {
            snippet.meta.visibility = Visibility::from(value.as_str());
        }
        "description" => {
            snippet.meta.description = value;
        }
        _ => {
            bail!("Key not found: {}", key);
        }
    }
    snippet.meta.modified = Utc::now();
    repo.save(&snippet)?;
    Ok(())
}
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::process::{Command};
use std::io::Write;
use chrono::Local;
use rusqlite::Connection;
use crate::config::settings::Settings;
use crate::errors::NibbError;
use crate::snippets::snippet::Snippet;
use crate::snippets::storage::{get_snippet, get_snippet_by_name};
use crate::utils::fs::get_nibb_dir;

pub fn nibb_git(command: Vec<String>, verbose: bool) -> Result<(), NibbError> {
    let old_cwd = std::env::current_dir()?;
    let nibb_dir = get_nibb_dir()?;
    std::env::set_current_dir(&nibb_dir)?;

    let mut cmd = Command::new("git");
    cmd.args(&command);

    if verbose {
        println!("{:?}", cmd);
    }

    let output = cmd.output()?; 

    std::env::set_current_dir(old_cwd)?;

    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_nibb_dir()?.join("git_log.txt"))?;

    writeln!(log_file, "{} git {}", timestamp, command.join(" "))?;
    if !output.stdout.is_empty() {
        writeln!(log_file, "stdout:\n{}", String::from_utf8_lossy(&output.stdout))?;
    }
    if !output.stderr.is_empty() {
        writeln!(log_file, "stderr:\n{}", String::from_utf8_lossy(&output.stderr))?;
    }

    if !output.status.success() {
        return Err(NibbError::GitError(format!("exited with {}", output.status)));
    }

    if verbose {
        print!("{}", String::from_utf8_lossy(&output.stdout));
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}


pub fn format_commit_message(message: &str, snippet: &Snippet) -> String {
    let mut msg = message.replace("{name}", &snippet.name);  
    let tags = snippet.tags.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    msg = msg.replace("{tags}", &tags.join(", "));
    msg = msg.replace("{description}", snippet.description.clone().unwrap_or("".to_string()).as_str());
    msg
}

pub fn auto_commit(message: &str, snippet: &Snippet) -> Result<(), NibbError> {
    nibb_git(vec!["add".to_string(), ".".to_string()], false)?;
    let commit_msg = format_commit_message(message, snippet);
    nibb_git(vec!["commit".to_string(), "-m".to_string(), commit_msg], false)?;
    Ok(())
}

pub fn auto_push(remote: &str, branch: &str) -> Result<(), NibbError> {
    nibb_git(vec!["push".to_string(), remote.to_string(), branch.to_string()], false)?;
    Ok(())   
}

pub fn auto_pull(remote: &str, branch: &str) -> Result<(), NibbError> {
    nibb_git(vec!["pull".to_string(), remote.to_string(), branch.to_string()], false)?;
    Ok(())
}

pub fn nibb_git_pre_actions(cfg: &Settings) -> Result<(), NibbError> {
    if !cfg.git_enabled() {
        return Ok(())
    }
    if cfg.auto_pull() {
        auto_pull(cfg.remote(), cfg.branch())?;
    }
    Ok(())
}

pub fn nibb_git_post_actions(
    name: &str,
    conn: &Connection,
    cfg: &Settings
) -> Result<(), NibbError>{
    if !cfg.git_enabled() {
        return Ok(())   
    }
    let snippet = get_snippet_by_name(conn, name).unwrap_or(Snippet::new(
        name.to_string(),
        "".to_string(),
        HashSet::new(),
        None,
        0,
    ));
    
    if cfg.auto_commit() {
        auto_commit(cfg.commit_message(), &snippet)?;
    }
    if cfg.auto_push() {
        auto_push(cfg.remote(), cfg.branch())?;
    }
    Ok(())
}

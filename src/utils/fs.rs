#![allow(dead_code)]

use std::path::PathBuf;
use crossterm::style::Stylize;
use crate::config::settings::Settings;
use crate::errors::NibbError;
pub fn get_nibb_dir() -> Result<PathBuf, NibbError> {
    Ok(
        dirs::home_dir()
            .ok_or_else(|| return NibbError::NotFound("Home directory".to_string()))?
            .join(".nibb")
    )
}

pub fn get_snippets_dir() -> Result<PathBuf, NibbError> {
    Ok(get_nibb_dir()?.join("snippets"))
}

pub fn get_nibb_toml_path() -> Result<PathBuf, NibbError> {
    Ok(get_nibb_dir()?.join("nibb.toml"))
}

pub fn get_nibb_backups_dir() -> Result<PathBuf, NibbError> {
    Ok(get_nibb_dir()?.join("backups"))
}

pub fn get_storage_path() -> Result<PathBuf, NibbError> {
    Ok(get_snippets_dir()?.join("nibb.db"))
}

pub fn create_necessary_directories() -> Result<(), NibbError>{
    std::fs::create_dir_all(get_nibb_dir()?)?;
    std::fs::create_dir_all(get_snippets_dir()?)?;
    std::fs::create_dir_all(get_nibb_backups_dir()?)?;
    Ok(())
}

pub fn create_default_toml() -> Result<(), NibbError>{
    let path = get_nibb_dir()?.join("nibb.toml");
    if path.exists(){
        return Ok(());      
    }
    let settings = Settings::default();
    settings.save()?;
    Ok(())
}


/// Creates all necessary files, i.e., storage.json, nibb.toml
pub fn create_necessary_files() -> Result<(), NibbError>{
    if get_storage_path()?.exists(){
        return Ok(());       
    }
    std::fs::File::create(get_storage_path()?)?;
    create_default_toml()?;
    Ok(())
}

/// Checks whether a given root is a git repository
pub fn is_git_repo(path: &PathBuf) -> bool {
    path.join(".git").exists()
}

pub fn create_git_repo() -> Result<(), NibbError>{
    let old_cwd = std::env::current_dir()?;
    let path = get_nibb_dir()?;
    std::env::set_current_dir(&path)?;
    
    if is_git_repo(&path){
        return Ok(());      
    }
    
    let status = std::process::Command::new("git")
        .arg("init")
        .status()?;
    if !status.success(){
        return Err(NibbError::GitError(format!("exited with {}", status)))
    }
    std::env::set_current_dir(&old_cwd)?;
    print_git_notes();
    Ok(())
}

fn print_git_notes() {
    println!("{}", "Initialized git repository in .nibb".green().bold());
    println!("{}", "If you want to use auto push and pull:".bold().yellow());
    println!("{}", "  - Add a remote repository with: nibb git remote add <remote> <url>".cyan());
    println!("{}", "  - Please ensure, that <remote> matches the remote name in your nibb.toml file".cyan());
    println!("{}", "  - Set an upstream using: nibb git push --set-upstream <remote> <branch>".cyan());
    println!("{}", "  - Please ensure, that <branch> matches the branch name in your nibb.toml file".cyan());
    println!("{}", "  - You can now use: nibb git push and nibb git pull or enable auto push and pull in your nibb.toml file".cyan());
}

/// ensures the necessary structure, for all operations with Nibb
pub fn ensure_nibb_structure() -> Result<(), NibbError>{
    create_necessary_directories()?;
    create_necessary_files()?;
    create_git_repo()?;
    Ok(())
}

#![allow(dead_code)]

use std::path::PathBuf;
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

pub fn get_storage_path() -> Result<PathBuf, NibbError> {
    Ok(get_snippets_dir()?.join("storage.json"))
}

pub fn create_necessary_directories() -> Result<(), NibbError>{
    std::fs::create_dir_all(get_nibb_dir()?)?;
    std::fs::create_dir_all(get_snippets_dir()?)?;
    Ok(())
}
pub fn create_necessary_files() -> Result<(), NibbError>{
    if get_storage_path()?.exists(){
        return Ok(());       
    }
    std::fs::File::create(get_storage_path()?)?;
    std::fs::write(get_storage_path()?, "[]")?;
    Ok(())
}

/// ensures the necessary structure, for all operations with Nibb
pub fn ensure_nibb_structure() -> Result<(), NibbError>{
    create_necessary_directories()?;
    create_necessary_files()?;
    Ok(())
}
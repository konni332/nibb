use std::path::PathBuf;
use crate::snippets::snippet::Snippet;
pub fn get_nibb_dir() -> PathBuf {
    dirs::home_dir().expect("Error: Unable to find home directory").join(".nibb")
}

pub fn get_snippets_dir() -> PathBuf {
    get_nibb_dir().join("snippets")
}

pub fn get_storage_path() -> PathBuf {
    get_snippets_dir().join("storage.json")
}

pub fn create_necessary_directories() -> Result<(), std::io::Error>{
    std::fs::create_dir_all(get_nibb_dir())?;
    std::fs::create_dir_all(get_snippets_dir())?;
    Ok(())
}
pub fn create_necessary_files() -> Result<(), std::io::Error>{
    std::fs::File::create(get_storage_path())?;
    std::fs::write(get_storage_path(), "[]")?;
    Ok(())
}

/// ensures the necessary structure, for all operations with Nibb
pub fn ensure_nibb_structure() -> Result<(), std::io::Error>{
    create_necessary_directories()?;
    create_necessary_files()?;
    Ok(())
}
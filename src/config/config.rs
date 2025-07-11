use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::result::NibbResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub git: GitConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub auto_commit: bool,
    #[serde(default)]
    pub commit_message: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub author_email: String,
    #[serde(default)]
    pub push_on_commit: bool,
    #[serde(default)]
    pub remote: Option<String>,
    #[serde(default)]
    pub branch: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            git: GitConfig::default(),
        }
    }
    pub fn load(path: &Path)-> NibbResult<Self> {
        if !path.exists() {
            std::fs::File::create(path)?;
            let cfg = Config::default();
            cfg.save(path)?;
            return Ok(cfg);
        }
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    pub fn save(&self, path: &Path) -> NibbResult<()> {
        let content = toml::to_string(self)?;
        if !path.exists() {
            std::fs::File::create(path)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            git: GitConfig::default(),
        }
    }
}

impl GitConfig {
    pub fn new() -> Self {
        GitConfig {
            enabled: false,
            auto_commit: false,
            commit_message: String::new(),
            author: String::new(),
            author_email: String::new(),
            push_on_commit: false,
            remote: None,
            branch: String::new(),       
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            auto_commit: false,
            commit_message: String::from("update: {name} @{modified}"),
            author: String::from("<AUTHOR NAME>"),
            author_email: String::from("<AUTHOR EMAIL>"),
            push_on_commit: false,
            remote: None,
            branch: String::from("master"),       
        }
    }
}
#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use crate::errors::NibbError;
use crate::utils::fs::get_nibb_dir;
use crate::utils::os::editor_available;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    editor: String,
    marker: String,
    git: Git,
}

#[derive(Serialize, Deserialize)]
pub struct Git {
    pub enabled: bool,
    pub remote: String,
    #[serde(rename = "auto-commit")]
    pub auto_commit: bool,
    #[serde(rename = "auto-push")]   
    pub auto_push: bool,
    #[serde(rename = "auto-pull")]   
    pub auto_pull: bool,   
    #[serde(rename = "commit-message")]  
    pub commit_message: String,
    pub branch: String,
}

impl Git {
    pub fn default() -> Self {
        Git {
            enabled: true,
            remote: String::from("origin"),
            auto_commit: true,
            auto_push: false,
            auto_pull: false,
            commit_message: String::from("update: {name}"),
            branch: String::from("master"),
        }   
    }
}

impl Settings {
    pub fn default() -> Self {
        let default_editor = get_default_editor();
        Settings {
            editor: String::from(default_editor),
            marker: String::from("//NIBB"),
            git: Git::default(),       
        }
    }
    pub fn load() -> Result<Self, NibbError> {
        let path = get_nibb_dir()?.join("nibb.toml");
        let content = std::fs::read_to_string(path)?;
        let cfg: Settings = toml::from_str(&content)
            .map_err(|e| NibbError::FSError(e.to_string()))?;
        Ok(cfg)
    }
    pub fn save(&self) -> Result<(), NibbError> {
        let path = get_nibb_dir()?.join("nibb.toml");
        let content = toml::to_string(&self)
            .map_err(|e| NibbError::FSError(e.to_string()))?;
        std::fs::write(path, content)?;
        Ok(())       
    }
    pub fn editor(&self) -> &str {
        &self.editor
    }
    pub fn marker(&self) -> &str {
        &self.marker
    }   
    pub fn reset(&mut self, key: Option<&str>) -> Result<(), NibbError> {
        match key {
            Some(key) => self.set(key, "default"),
            None => {
                self.editor = get_default_editor();
                self.marker = get_default_marker();
                Ok(())
            }
        }
    }
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), NibbError> {
        match key { 
            "editor" => {
                if value == "default" {
                    self.editor = get_default_editor();
                }
                else {
                    self.editor = String::from(value);   
                }
                Ok(())
            },
            "marker" => {
                if value == "default" {
                    self.marker = get_default_marker();  
                }
                else {
                    self.marker = String::from(value);
                }
                Ok(())
            },
            "git.enabled" => {
                if value == "false" || value == "0" || value == "default" {
                    self.git.enabled = false;
                    return Ok(())
                }
                if value == "true" || value == "1" {
                    self.git.enabled = true;
                    return Ok(())
                }
                Err(NibbError::ConfigError(format!("Invalid value for git.enabled: {}", value)))
            }
            "git.remote" => {
                self.git.remote = String::from(value);
                Ok(())
            }
            "git.auto-commit" => {
                if value == "false" || value == "0" || value == "default" {
                    self.git.auto_commit = false;
                    return Ok(())
                }
                if value == "true" || value == "1" {
                    self.git.auto_commit = true;
                    return Ok(())
                }
                Err(NibbError::ConfigError(format!("Invalid value for git.auto-commit: {}", value)))
            }
            "git.auto-push" => {
                if value == "false" || value == "0" || value == "default" {
                    self.git.auto_push = false;
                    return Ok(())
                }
                if value == "true" || value == "1" {
                    self.git.auto_push = true;
                    return Ok(())
                }
                Err(NibbError::ConfigError(format!("Invalid value for git.auto-push: {}", value)))
            }
            "git.auto-pull" => {
                if value == "false" || value == "0" || value == "default" {
                    self.git.auto_pull = false;
                    return Ok(())
                }
                if value == "true" || value == "1" {
                    self.git.auto_pull = true;
                    return Ok(())
                }
                Err(NibbError::ConfigError(format!("Invalid value for git.auto-pull: {}", value)))
            }
            "git.commit-message" => {
                self.git.commit_message = String::from(value);
                Ok(())
            }
            "git.branch" => {
                self.git.branch = String::from(value);
                Ok(())
            }
            _ => {
                Err(NibbError::ConfigError(format!("Key '{}' not found in settings", key)))
            }
        }
    }
    pub fn get(&self, key: &str) -> Result<String, NibbError> {
        match key {
            "editor" => Ok(self.editor.clone()),
            "marker" => Ok(self.marker.clone()),
            "git.enabled" => Ok(self.git.enabled.to_string()),
            "git.remote" => Ok(self.git.remote.clone()),
            "git.auto-commit" => Ok(self.git.auto_commit.to_string()),
            "git.auto-push" => Ok(self.git.auto_push.to_string()),
            "git.auto-pull" => Ok(self.git.auto_pull.to_string()),
            "git.commit-message" => Ok(self.git.commit_message.clone()),
            "git.branch" => Ok(self.git.branch.clone()),
            _ => {
                Err(NibbError::NotFound(format!("Key '{}' in settings", key)))
            }
        }
    }
    pub fn git_enabled(&self) -> bool {
        self.git.enabled
    }
    pub fn auto_commit(&self) -> bool {
        self.git.auto_commit
    }
    pub fn auto_push(&self) -> bool {
        self.git.auto_push
    }
    pub fn auto_pull(&self) -> bool {
        self.git.auto_pull
    }
    pub fn commit_message(&self) -> &str {
        &self.git.commit_message
    }
    pub fn branch(&self) -> &str {
        &self.git.branch
    }
    pub fn remote(&self) -> &str {
        &self.git.remote
    }
}

pub fn get_default_marker() -> String {
    String::from("//NIBB")
}

pub fn get_default_editor() -> String {
    if let Ok(editor) = std::env::var("EDITOR").or_else(|_| std::env::var("VISUAL")) {
        if editor_available(editor.as_str()) {
            return editor;       
        }
    }
    
    let candidates = if cfg!(target_os = "windows") {
        vec![
            "nvim",
            "code",
            "notepad++",
            "notepad",
            "notepad.exe",
        ]
    }
    else { 
        vec![
            "nvim",
            "vim",
            "micro",
            "helix",
            "nano",
            "emacs",
            "code", // VSCode CLI
            "gedit",
            "kate",
            "kak",
            "vi",   // ultimate fallback
        ]
    };
    
    for candidate in candidates {
        if  editor_available(candidate) {
            return String::from(candidate);
        }
    }
    String::from("vi") // final fallback
}

use serde::{Serialize, Deserialize};
use crate::errors::NibbError;
use crate::utils::fs::get_nibb_dir;
use crate::utils::os::editor_available;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    editor: String,
    marker: String,
}

impl Settings {
    pub fn default() -> Self {
        let default_editor = get_default_editor();
        Settings {
            editor: String::from(default_editor),
            marker: String::from("//NIBB"),
        }
    }
    pub fn load() -> Result<Self, NibbError> {
        let path = get_nibb_dir()?.join("nibb.toml");
        let content = std::fs::read_to_string(path)?;
        let cfg: Settings = toml::from_str(&content)
            .map_err(|e| NibbError::FSError(e.to_string()))?;
        Ok(cfg)
    }
    pub fn save(self) -> Result<(), NibbError> {
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

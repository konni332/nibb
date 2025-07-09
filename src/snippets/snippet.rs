use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use slug::slugify;
use crate::snippets::file_type::FileType;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub language: FileType, // e.g., "rust", "bash", "sql"
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    #[serde(default = "default_visibility")]
    pub visibility: Visibility,
}

impl Meta {
    pub fn new(
        name: String,
        description: String,
        tags: Vec<String>,
        language: FileType,
        visibility: Option<Visibility>) -> Self {
        Self {
            name,
            description,
            tags,
            language,
            created: Utc::now(),
            modified: Utc::now(),
            visibility: visibility.unwrap_or(default_visibility()),
        }
    }
    pub fn get_content_extension(&self) -> String {
        self.language.extension().to_string()
    }

    pub fn get_slug(&self) -> String {
        slugify(self.name.as_str())
    }
}




fn default_visibility() -> Visibility {
    Visibility::Private
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Private,
    Public,
    Archived,
}

impl From<&str> for Visibility {
    fn from(s: &str) -> Self {
        match s {
            "private" => Visibility::Private,
            "public" => Visibility::Public,
            "archived" => Visibility::Archived,
            _ => Visibility::Private,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub meta: Meta,
    pub content: String,
}

impl Snippet {
    pub fn new(meta: Meta, content: String) -> Self {
        Self { meta, content }
    }
}


#[cfg(feature = "ansi")]
use colored::*;

impl fmt::Display for Meta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "ansi")]
        {
            writeln!(f, "{}: {}", "Name".bold().cyan(), self.name)?;
            writeln!(f, "{}: {}", "Description".bold().cyan(), self.description)?;
            writeln!(f, "{}: {}", "Tags".bold().cyan(), self.tags.join(", "))?;
            writeln!(f, "{}: {}", "Language".bold().cyan(), self.language)?;
            writeln!(f, "{}: {}", "Created".bold().cyan(), self.created)?;
            writeln!(f, "{}: {}", "Modified".bold().cyan(), self.modified)?;
            writeln!(f, "{}: {}", "Visibility".bold().cyan(), format!("{:?}", self.visibility))?;
            Ok(())
        }

        #[cfg(not(feature = "ansi"))]
        {
            writeln!(f, "Name: {}", self.name)?;
            writeln!(f, "Description: {}", self.description)?;
            writeln!(f, "Tags: {}", self.tags.join(", "))?;
            writeln!(f, "Language: {}", self.language)?;
            writeln!(f, "Created: {}", self.created)?;
            writeln!(f, "Modified: {}", self.modified)?;
            writeln!(f, "Visibility: {:?}", self.visibility)?;
            Ok(())
        }
    }
}


impl fmt::Display for Snippet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "ansi")]
        {
            writeln!(f, "{}\n", self.meta)?;
            writeln!(f, "{}\n{}", "Content:".bold().green(), self.content)
        }

        #[cfg(not(feature = "ansi"))]
        {
            writeln!(f, "{}\n", self.meta)?;
            writeln!(f, "Content:\n{}", self.content)
        }
    }
}

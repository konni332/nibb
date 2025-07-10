use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use slug::slugify;
use crate::snippets::file_type::FileType;
/// Metadata associated with a snippet.
///
/// Includes name, description, tags, programming language, creation and modification timestamps,
/// as well as visibility status (e.g., public, private).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    /// Name of the snippet (used for display and slug generation).
    pub name: String,
    /// Short description of the snippet's purpose or context.
    pub description: String,
    /// Tags associated with the snippet, e.g. `["sql", "orm", "diesel"]`.
    pub tags: Vec<String>,
    /// Language or file type of the snippet (e.g. `rust`, `bash`, `sql`).
    pub language: FileType,
    /// Timestamp indicating when the snippet was created (UTC).
    pub created: DateTime<Utc>,
    /// Timestamp indicating when the snippet was last modified (UTC).
    pub modified: DateTime<Utc>,
    /// Visibility status (e.g. `Private`, `Public`, `Archived`). Defaults to `Private`.
    #[serde(default = "Visibility::default")]
    pub visibility: Visibility,
}

impl Meta {
    /// Creates a new `Meta` object with the current timestamp for `created` and `modified`.
    ///
    /// If no visibility is provided, defaults to `Private`.
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
            visibility: visibility.unwrap_or_default(),
        }
    }
    /// Returns the standard file extension (without a dot) for the snippet's language.
    pub fn get_content_extension(&self) -> String {
        self.language.extension().to_string()
    }
    /// Returns a slugified version of the snippet's name.
    pub fn get_slug(&self) -> String {
        slugify(self.name.as_str())
    }
}




impl Default for Visibility {
    fn default() -> Self {
        Visibility::Private
    }
}

/// Defines visibility levels for a snippet.
///
/// Used to control whether a snippet is accessible to the public or not.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    /// Visible only to the user.
    Private,
    /// Publicly visible and accessible.
    Public,
    /// Archived, i.e., visible but no longer actively maintained.
    Archived,
}

impl From<&str> for Visibility {
    /// Parses a string into a `Visibility` enum variant.
    ///
    /// Unknown values default to `Private`.
    fn from(s: &str) -> Self {
        match s {
            "private" => Visibility::Private,
            "public" => Visibility::Public,
            "archived" => Visibility::Archived,
            _ => Visibility::Private,
        }
    }
}
/// Represents a complete code snippet, including metadata and the actual content.
///
/// Combines [`Meta`] with the snippet's textual content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    /// Associated metadata (name, language, visibility, etc.).
    pub meta: Meta,
    /// Raw code content of the snippet.
    pub content: String,
}

impl Snippet {
    /// Creates a new snippet from the given metadata and content.
    pub fn new(meta: Meta, content: String) -> Self {
        Self { meta, content }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| "{\"type\":\"Other\",\"message\":\"Serialization failed\"}".to_string())
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
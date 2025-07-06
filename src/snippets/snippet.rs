use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
#[cfg(feature = "ansi")]
use crossterm::style::Stylize;
use crossterm::{
    style::{Attribute, Print, SetAttribute},
    ExecutableCommand,
};
use std::io::{stdout};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snippet {
    pub name: String,
    pub content: String,
    pub tags: HashSet<String>,
    pub description: Option<String>,
    pub id: i32,
    pub lang: Lang,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    Rust,
    Python,
    Bash,
    C,
    CPP,
    Java,
    JavaScript,
    TypeScript,
    Go,
    Unknown,
}

impl Display for Lang {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Lang::Rust => write!(f, "rust"),
            Lang::Python => write!(f, "python"),
            Lang::Bash => write!(f, "bash"),
            Lang::C => write!(f, "c"),
            Lang::CPP => write!(f, "cpp"),
            Lang::Java => write!(f, "java"),
            Lang::JavaScript => write!(f, "javascript"),
            Lang::TypeScript => write!(f, "typescript"),
            Lang::Go => write!(f, "go"),
            _ => write!(f, "unknown"),
        }
    }
}

impl From<&str> for Lang {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "rust" => Lang::Rust,
            "python" => Lang::Python,
            "bash" => Lang::Bash,
            "c" => Lang::C,
            "c++" => Lang::CPP,
            "java" => Lang::Java,
            "javascript" => Lang::JavaScript,
            "typescript" => Lang::TypeScript,
            "go" => Lang::Go,
            _ => Lang::Unknown,
        }
    }
}

impl Snippet {
    pub fn new(
        name: String,
        content: String,
        tags: HashSet<String>,
        description: Option<String>,
        id: i32,
        lang: Lang,
    ) -> Snippet {
        Snippet {
            name,
            content,
            tags,
            description,
            id,
            lang,
        }
    }
    pub fn create(name: String, tags: Option<Vec<String>>, lang: Lang) -> Snippet {
        let hashed_tags = HashSet::from_iter(tags.unwrap_or_default());

        Snippet::new(
            name,
            String::new(),
            hashed_tags,
            None,
            1,
            lang
        )
    }
    #[cfg(feature = "ansi")]
    pub fn pretty_print(&self, verbose: bool) {
        let max_lines = 5;
        let lines: Vec<&str> = self.content.lines().take(max_lines).collect();

        println!("    {}:", self.name.clone().bold().green());
        if verbose {
            println!("      {} {}", "path".yellow(), self.path);
        }
        if let Some(description) = &self.description {
            println!("        {} {}", "description:".cyan(), description);
        }

        let tags = self
            .tags
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        println!("        {} {}", "tags:".cyan(), tags);

        let mut stdout = stdout();
        if lines.len() == 1 {
            println!("        {} {}", "content:".cyan(), lines[0]);
        } else {
            println!("        {} ", "content:".cyan());
            for line in lines {
                stdout
                    .execute(SetAttribute(Attribute::Italic)).unwrap();
                stdout
                    .execute(Print(format!("                 {}\n", line))).unwrap();
                stdout
                    .execute(SetAttribute(Attribute::Reset)).unwrap();
            }
        }
    }
    #[cfg(not(feature = "ansi"))]
    pub fn pretty_print(&self, verbose: bool) {
        let max_lines = 5;
        let lines: Vec<&str> = self.content.lines().take(max_lines).collect();

        println!("    {}:", self.name.clone());
        if verbose {
            println!("      {} {}", "id", self.id);
        }
        if let Some(description) = &self.description {
            println!("        {} {}", "description:", description);
        }

        println!("        {} {}", "Language:", &self.lang.to_string());
        let tags = self
            .tags
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        println!("        {} {}", "tags:", tags);

        let mut stdout = stdout();
        if lines.len() == 1 {
            println!("        {} {}", "content:", lines[0]);
        } else {
            println!("        {} ", "content:");
            for line in lines {
                stdout
                    .execute(SetAttribute(Attribute::Italic)).unwrap();
                stdout
                    .execute(Print(format!("                 {}\n", line))).unwrap();
                stdout
                    .execute(SetAttribute(Attribute::Reset)).unwrap();
            }
        }
    }
}

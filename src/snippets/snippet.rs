use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use crossterm::style::Stylize;
use crate::utils::fs::get_snippets_dir;
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
    pub path: String,
}

impl Snippet {
    pub fn new(
        name: String,
        content: String,
        tags: HashSet<String>,
        description: Option<String>,
        path: String,
    ) -> Snippet {
        Snippet {
            name,
            content,
            tags,
            description,
            path,       
        }
    }
    pub fn create(name: String, tags: Option<Vec<String>>) -> Snippet {
        let hashed_tags = HashSet::from_iter(tags.unwrap_or_default());
        let path = get_snippets_dir().expect("Unable to get snippets dir").join(&name);
        
        Snippet::new(
            name,
            String::new(),
            hashed_tags,
            None,
            path.to_str().unwrap().to_string(),       
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
            println!("      {} {}", "path", self.path);
        }
        if let Some(description) = &self.description {
            println!("        {} {}", "description:", description);
        }

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

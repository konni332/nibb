use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use crossterm::style::Stylize;
use crate::utils::fs::get_snippets_dir;

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
        let path = get_snippets_dir().join(&name);
        
        Snippet::new(
            name,
            String::new(),
            hashed_tags,
            None,
            path.to_str().unwrap().to_string(),       
        )
    }
    pub fn pretty_print(&self, verbose: bool) {
        let line = self.content.lines().last().unwrap_or("None");
        println!("    {}:", self.name.clone().bold().green());
        if verbose {
            println!("      {} {}", "path".yellow(), self.path);
        }
        if let Some(description) = &self.description {
            println!("        {} {}", "description:".cyan() ,description);       
        }
        let tags = self.tags.iter()
            .map(|t| t.to_string()).collect::<Vec<String>>().join(", ");
        println!("        {} {}", "tags:".cyan(), tags);
        println!("        {} {}", "content:".cyan(), line);
    }
}

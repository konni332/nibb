use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use crate::snippets::storage::{load_snippets, save_snippets};
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
    pub fn pretty(&self) -> String {
        todo!()
    }
}

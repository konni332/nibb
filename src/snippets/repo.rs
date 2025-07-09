use std::path::{Path, PathBuf};
use slug::slugify;
use crate::result::{NibbError, NibbResult};
use crate::snippets::snippet::{Meta, Snippet};

pub trait SnippetRepository {
    fn load_all(&self) -> NibbResult<Vec<Snippet>>;
    fn load(&self, slug: &str) -> NibbResult<Snippet>;
    fn save(&self, snippet: &Snippet) -> NibbResult<()>;
    fn save_all(&self, snippets: &[Snippet]) -> NibbResult<()>;
    fn delete(&self, slug: &str) -> NibbResult<()>;
}

pub struct FSRepo {
    pub base_dir: PathBuf,
}

impl FSRepo {
    pub fn new<P: AsRef<Path>>(path: P) -> NibbResult<Self> {
        let repo = Self {
            base_dir: path.as_ref().to_path_buf(),
        };
        repo.ensure_structure()?;
        Ok(repo)
    }
    fn snippet_path(&self, slug: &str) -> PathBuf {
        self.snippets_dir().join(slug)
    }
    fn snippets_dir(&self) -> PathBuf {
        self.base_dir.join("snippets")
    }
    fn history_dir(&self) -> PathBuf {
        self.base_dir.join("history")
    }
    fn config_path(&self) -> PathBuf {
        self.base_dir.join("config.toml")
    }
    fn ensure_structure(&self) -> NibbResult<()> {
        std::fs::create_dir_all(&self.base_dir)?;
        std::fs::create_dir_all(self.snippets_dir())?;
        std::fs::create_dir_all(self.history_dir())?;
        let config_path = self.config_path();
        if !config_path.exists() {
            std::fs::File::create(config_path)?;
        }
        Ok(())
    }
    fn get_content_path(&self, slug: &str, extension: &str) -> PathBuf {
        let snippet_path = self.snippet_path(slug);
        snippet_path.join(format!("content.{}", extension))
    }
    fn get_meta_path(&self, slug: &str) -> PathBuf {
        let snippet_path = self.snippet_path(slug);
        snippet_path.join("meta.toml")
    }
}

impl SnippetRepository for FSRepo {
    fn load_all(&self) -> NibbResult<Vec<Snippet>> {
        let entries = std::fs::read_dir(self.snippets_dir())
            .map_err(|e| NibbError::NotFound("Snippets directory".to_string()))?;
        let mut snippets = Vec::new();
        for entry in entries {
            let entry = entry?;
            let slug = slugify(entry.file_name().to_str().unwrap());
            snippets.push(self.load(&slug)?);
        }
        Ok(snippets)
    }
    fn load(&self, slug: &str) -> NibbResult<Snippet> {
        let meta_path = self.get_meta_path(slug);
        let meta_str = std::fs::read_to_string(meta_path)?;
        let meta: Meta = toml::from_str(&meta_str)?;

        let content_path = self.get_content_path(slug, &meta.get_content_extension());
        let content = std::fs::read_to_string(content_path)?;
        Ok(Snippet {
            meta,
            content,
        })
    }
    fn save(&self, snippet: &Snippet) -> NibbResult<()> {
        let slug = snippet.meta.get_slug();
        let extension = snippet.meta.get_content_extension();
        let meta_path = self.get_meta_path(&slug);
        std::fs::write(meta_path, toml::to_string(&snippet.meta)?)?;

        let content_path = self.get_content_path(&slug, &extension);
        std::fs::write(content_path, &snippet.content)?;

        Ok(())
    }
    fn save_all(&self, snippets: &[Snippet]) -> NibbResult<()> {
        for snippet in snippets {
            self.save(snippet)?;
        }
        Ok(())
    }
    fn delete(&self, slug: &str) -> NibbResult<()> {
        let snippet_path = self.snippet_path(slug);
        std::fs::remove_dir_all(snippet_path)?;
        Ok(())
    }
}

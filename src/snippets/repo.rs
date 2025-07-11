use std::fs;
use std::path::{Path, PathBuf};
use slug::slugify;
use crate::config::config::Config;
use crate::get_nibb_dir;
use crate::git::git_integration::GitRepo;
use crate::result::{NibbError, NibbResult};
use crate::snippets::snippet::{Meta, Snippet};
/// Defines the interface for a snippet repository backend.
///
/// Allows loading, saving, and deleting individual or multiple [`Snippet`]s
/// via a consistent API, independent of the concrete storage type (e.g. FS, DB, etc.).
pub trait SnippetRepository {
    /// Load all available snippets from the repository.
    fn load_all(&self) -> NibbResult<Vec<Snippet>>;
    /// Load a single snippet by its slugified name.
    fn load(&self, slug: &str) -> NibbResult<Snippet>;
    /// Save or update a single snippet.
    fn save(&self, snippet: &Snippet) -> NibbResult<()>;
    /// Save or update a batch of snippets.
    fn save_all(&self, snippets: &[Snippet]) -> NibbResult<()>;
    /// Delete a snippet by its slug.
    fn delete(&self, slug: &str) -> NibbResult<()>;
}
/// Filesystem-backed implementation of [`SnippetRepository`].
///
/// Snippets are stored under a root `base_dir`, each in a dedicated subdirectory named by slug.
/// Each snippet directory contains:
/// - `meta.toml`: metadata (name, tags, language, etc.)
/// - `content.<ext>`: raw snippet content file
/// Additionally, `base_dir` contains:
/// - `snippets/`: all snippet folders
/// - `history/`: reserved for future versioning/history features
/// - `config.toml`: configuration file (created if missing)
pub struct FSRepo {
    /// Root directory containing all snippet data.
    pub base_dir: PathBuf,
    pub config: Config,
    pub git_repo: GitRepo,
}

impl FSRepo {
    /// Creates a new [`FSRepo`] and ensures the necessary folder structure exists.
    ///
    /// Will create `snippets/`, `history/`, and `config.toml` if missing.
    pub fn new<P: AsRef<Path>>(path: P) -> NibbResult<Self> {
        let config = Config::load(&path.as_ref().join("config.toml"))?;
        let git_repo = GitRepo::init_or_open(get_nibb_dir()?)?;
        let repo = Self {
            git_repo,
            config,
            base_dir: path.as_ref().to_path_buf(),
        };
        repo.ensure_structure()?;
        Ok(repo)
    }
    /// Returns the path to a snippet's directory based on its slug.
    pub fn snippet_path(&self, slug: &str) -> PathBuf {
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
        fs::create_dir_all(&self.base_dir)?;
        fs::create_dir_all(self.snippets_dir())?;
        fs::create_dir_all(self.history_dir())?;
        let config_path = self.config_path();
        if !config_path.exists() {
            fs::File::create(config_path)?;
        }
        Ok(())
    }
    fn auto_commit(&self, snippet: &Snippet) -> NibbResult<()> {
        if !self.config.git.enabled || !self.config.git.auto_commit {
            return Ok(());
        }
        self.git_repo.add_and_commit(snippet, &self.config)?;
        self.auto_push()?;
        Ok(())
    }
    fn auto_push(&self) -> NibbResult<()> {
        if !self.config.git.enabled || !self.config.git.push_on_commit {
            return Ok(());
        }
        match &self.config.git.remote {
            Some(remote) => {
                Ok(self.git_repo.push(remote, &self.config.git.branch)?)
            },
            None => Ok(())
        }
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
    /// Loads all snippets by iterating through the `snippets/` directory
    /// and deserializing each snippet from `meta.toml` and its content file.
    fn load_all(&self) -> NibbResult<Vec<Snippet>> {
        let entries = std::fs::read_dir(self.snippets_dir())
            .map_err(|e| NibbError::NotFound(format!("{}:{:?}", e.to_string(), self.snippets_dir())))?;
        let mut snippets = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| NibbError::NotFound(format!("{}:{:?}", e.to_string(), self.snippets_dir())))?;
            let slug = slugify(entry.file_name().to_str().unwrap());
            snippets.push(self.load(&slug)?);
        }
        Ok(snippets)
    }
    /// Loads a specific snippet by slug.
    ///
    /// Reads metadata from `meta.toml` and content from `content.<ext>`.
    fn load(&self, slug: &str) -> NibbResult<Snippet> {
        let slug = slugify(slug); // just to be sure
        let meta_path = self.get_meta_path(&slug);
        let meta_str = std::fs::read_to_string(&meta_path)
            .map_err(|e| NibbError::NotFound(format!("{}:{:?}", e.to_string(), &meta_path)))?;
        let meta: Meta = toml::from_str(&meta_str)
            .map_err(|e| NibbError::NotFound(format!("{}:{:?}", e.to_string(), &meta_path)))?;

        let content_path = self.get_content_path(&slug, &meta.get_content_extension());
        let content = std::fs::read_to_string(&content_path)
            .map_err(|e| NibbError::NotFound(format!("{}:{:?}", e.to_string(), &content_path)))?;
        Ok(Snippet {
            meta,
            content,
        })
    }
    /// Saves a single snippet to disk.
    ///
    /// Creates the snippet folder and both metadata/content files if they don't exist.
    fn save(&self, snippet: &Snippet) -> NibbResult<()> {
        let slug = snippet.meta.get_slug();
        let snippet_path = self.snippet_path(&slug);

        if !snippet_path.exists() {
            fs::create_dir_all(&snippet_path)
                .map_err(|e| NibbError::NotFound(format!("{}:{:?}", e.to_string(), &snippet_path)))?;
        }

        let meta_path = self.get_meta_path(&slug);
        fs::write(&meta_path, toml::to_string(&snippet.meta)?)
            .map_err(|e| NibbError::NotFound(format!("{}:{:?}", e.to_string(), &meta_path)))?;

        for entry in fs::read_dir(&snippet_path)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.starts_with("content.") {
                    fs::remove_file(&path)
                        .map_err(|e| NibbError::NotFound(format!("Failed to remove old content file {}: {}", path.display(), e)))?;
                }
            }
        }

        let extension = snippet.meta.get_content_extension();
        let content_path = self.get_content_path(&slug, &extension);

        fs::write(&content_path, &snippet.content)
            .map_err(|e| NibbError::NotFound(format!("{}:{:?}", e.to_string(), &content_path)))?;

        // git actions (handles config)
        self.auto_commit(snippet)?;
        Ok(())
    }
    /// Saves a list of snippets.
    ///
    /// Calls [`save()`](Self::save) for each snippet.
    fn save_all(&self, snippets: &[Snippet]) -> NibbResult<()> {
        for snippet in snippets {
            self.save(snippet)?;
        }
        Ok(())
    }
    /// Deletes the snippet directory and all its contents.
    fn delete(&self, slug: &str) -> NibbResult<()> {
        let slug = slugify(slug); // just to be sure
        let snippet_path = self.snippet_path(&slug);
        fs::remove_dir_all(snippet_path)?;
        Ok(())
    }
}

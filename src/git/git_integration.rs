use std::path::{Path, PathBuf};
use git2::{Repository, Signature};
use walkdir::WalkDir;
use crate::config::config::Config;
use crate::Snippet;

pub struct GitRepo {
    repo: Repository,
    root: PathBuf,
}

impl GitRepo {
    pub fn init_or_open<P: AsRef<Path>>(path: P) -> Result<Self, git2::Error> {
        let path = path.as_ref();
        let repo = match Repository::open(path) {
            Ok(r) => r,
            Err(e) => Repository::init(path)?,
        };
        Ok(GitRepo {
            repo,
            root: path.to_path_buf(),
        })
    }
    pub fn add_and_commit(&self, snippet: &Snippet, cfg: &Config) -> Result<(), git2::Error> {
        let rel_path = PathBuf::from("snippets").join(snippet.meta.get_slug());
        let abs_path = self.repo.path().parent().unwrap().join(&rel_path);

        let message = &format_commit_msg(&cfg.git.commit_message, snippet);

        let mut index = self.repo.index()?;

        for entry in WalkDir::new(&abs_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let abs_file_path = entry.path();
            let rel_repo_path = abs_file_path.strip_prefix(self.repo.path().parent().unwrap()).unwrap();
            index.add_path(rel_repo_path)?;
        }

        index.write()?;
        let oid = index.write_tree()?;
        let tree = self.repo.find_tree(oid)?;
        let sig = Signature::now(&cfg.git.author, &cfg.git.author_email)?;

        let parent_commit = self.repo.head()
            .ok()
            .and_then(|h| h.target())
            .and_then(|oid| self.repo.find_commit(oid).ok());

        if let Some(parent) = parent_commit {
            self.repo.commit(
                Some("HEAD"),
                &sig,
                &sig,
                message,
                &tree,
                &[&parent],
            )?;
        } else {
            self.repo.commit(
                Some("HEAD"),
                &sig,
                &sig,
                message,
                &tree,
                &[],
            )?;
        }

        Ok(())
    }
    pub fn push(&self, remote_name: &str, branch: &str) -> Result<(), git2::Error> {
        let mut remote = self.repo.find_remote(remote_name)?;
        remote.push(&[format!("refs/head/{}", branch)], None)?;
        Ok(())
    }
}

fn format_commit_msg(msg: &str, snippet: &Snippet) -> String {
    let formatted = String::from(msg);
    formatted.replace("{name}", &snippet.meta.name)
        .replace(" {slug}", &snippet.meta.get_slug())
        .replace(" {description}", &snippet.meta.description)
        .replace(" {tags}", &snippet.meta.tags.join(","))
        .replace("{modified}", &snippet.meta.modified.to_string())
        .replace("{created}", &snippet.meta.created.to_string())
}
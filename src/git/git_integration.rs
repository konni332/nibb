use std::path::{Path, PathBuf};
use git2::{Repository, Signature};
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

        let message = &format_commit_msg(&cfg.git.commit_message, snippet);

        let mut index = self.repo.index()?;
        index.add_path(rel_path.as_path())?;
        index.write()?;

        let oid = index.write_tree()?;
        let tree = self.repo.find_tree(oid)?;
        let sig = Signature::now(&cfg.git.author, &cfg.git.author_email)?;
        let parent_commit = self.repo.head()
            .ok()
            .and_then(|h| h.target())
            .and_then(|oid| self.repo.find_commit(oid).ok());

        let committer = &sig;

                if let Some(parent) = parent_commit {
            self.repo.commit(
                Some("HEAD"),
                &sig,
                committer,
                message,
                &tree,
                &[&parent],
            )?;
        } else {
            // Initial commit
            self.repo.commit(
                Some("HEAD"),
                &sig,
                committer,
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
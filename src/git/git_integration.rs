use std::path::{Path, PathBuf};
use std::process::Command;
use git2::{Repository, Signature};
use serde_json::json;
use walkdir::WalkDir;
use crate::config::config::Config;
use crate::result::{NibbError, NibbResult};
use crate::{get_nibb_dir, Snippet};

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

/// Executes a generic Git command inside the `.nibb` directory.
///
/// # Arguments
/// - `args`: A vector of strings representing the Git command arguments, e.g. `["status", "-s"]`.
///
/// # Returns
/// - `Ok(String)`: JSON string containing fields `"stdout"` and `"stderr"`.
/// - `Err(NibbError)`: An error occurred during command execution or validation.
///
/// # Validation
/// - Ensures only allowed Git subcommands and safe path arguments are accepted.
///
/// # Example
/// ```rust
/// let output = nibb_git_generic(vec!["status".into(), "-s".into()])?;
/// println!("{}", output);
/// ```
pub fn nibb_git_generic(args: Vec<String>) -> NibbResult<String> {
    validate_git_args(&args).map_err(|e| NibbError::NibbGitError(e))?;
    let path = get_nibb_dir()?;
    let old_cwd = std::env::current_dir()?;
    std::env::set_current_dir(&path)?;

    let output = Command::new("git")
        .args(&args)
        .output();

    std::env::set_current_dir(old_cwd)?;

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();

            let result = json!({
                "stdout": stdout.trim(),
                "stderr": stderr.trim()
            });

            Ok(result.to_string())
        }
        Err(e) => {
            let err_json = json!({
                "stdout": "",
                "stderr": format!("Failed to execute git command: {e}")
            });
            Ok(err_json.to_string())
        }
    }
}



const ALLOWED_COMMANDS: &[&str] = &[
    "status", "log", "diff", "show", "branch", "checkout", "add", "commit", "push", "pull", "fetch"
];

fn validate_git_args(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("No git command provided".into());
    }

    let subcommand = &args[0];
    if !ALLOWED_COMMANDS.contains(&subcommand.as_str()) {
        return Err(format!("Git subcommand '{}' is not allowed", subcommand));
    }

    for arg in args.iter().skip(1) {
        if arg.starts_with('-') {
            continue;
        }

        if arg.contains("..") {
            return Err(format!("Path argument '{}' contains disallowed '..'", arg));
        }
        if arg.starts_with('/') || arg.contains(':') {
            return Err(format!("Absolute path argument '{}' is not allowed", arg));
        }
    }

    Ok(())
}
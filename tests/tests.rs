

use nibb_core::{FSRepo, FileType, Meta, Snippet};
use tempfile::TempDir;

fn get_test_repo() -> FSRepo {
    let temp_dir = TempDir::new().unwrap();
    FSRepo::new(temp_dir.path().join(".nibb")).unwrap()
}

fn get_test_snippet() -> Snippet {
    let meta = Meta::new(
        "Test Snippet".to_string(),
        "test-desc".to_string(),
        vec!["test-tag, another-tag".to_string()],
        FileType::Rust,
        None,
    );
    Snippet::new(meta, "main() {\n println!(\"Hello world\"); \n}".to_string())
}

#[cfg(test)]
mod tests {
    use nibb_core::{SnippetRepository};
    use super::*;

    #[test]
    fn test_new_and_delete() {
        let repo = get_test_repo();
        let snippet = get_test_snippet();
        repo.save(&snippet).unwrap();

        let snippet_path = repo.snippet_path(&snippet.meta.get_slug());
        assert!(snippet_path.exists());
        let meta_path = snippet_path.join("meta.toml");
        assert!(meta_path.exists());
        let content_path = snippet_path.join("content.rs");
        assert!(content_path.exists());

        repo.delete(&snippet.meta.get_slug()).unwrap();
        assert!(!snippet_path.exists());
        assert!(!meta_path.exists());
    }

    #[test]
    fn test_override() {
        let repo = get_test_repo();
        let mut snippet = get_test_snippet();
        repo.save(&snippet).unwrap();
        snippet.meta.language = FileType::Python;
        repo.save(&snippet).unwrap();
        assert!(repo.snippet_path(&snippet.meta.get_slug()).exists());
        assert!(repo.snippet_path(&snippet.meta.get_slug()).join("meta.toml").exists());
        assert!(repo.snippet_path(&snippet.meta.get_slug()).join("content.py").exists());
    }

    #[test]
    fn test_load() {
        let repo = get_test_repo();
        let snippet = get_test_snippet();
        repo.save(&snippet).unwrap();
        let snippet = repo.load(&snippet.meta.get_slug()).unwrap();
        assert_eq!(snippet.meta.language, FileType::Rust);
        assert_eq!(snippet.content, "main() {\n println!(\"Hello world\"); \n}");
        assert_eq!(snippet.meta.name, "Test Snippet");
        assert_eq!(snippet.meta.description, "test-desc");
        assert_eq!(snippet.meta.tags, vec!["test-tag, another-tag".to_string()]);
    }
}
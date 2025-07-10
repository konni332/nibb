use serde::{Deserialize, Serialize};

/// This enum unifies a broad set of programming, markup, config, and plain/binary file types
/// into a single representation. It's a type-safe, ergonomic API instead of relying on raw string labels.
///
/// The enum supports:
/// - Case-insensitive string conversion via `From<&str>`,
/// - File extension retrieval via `.extension()`,
/// - Printable output via `Display`.
///
/// # Variants
/// Programming languages:
/// - Rust, C, Cpp, CSharp, Java, Python, JavaScript, TypeScript, Go, Ruby, PHP, Swift, Kotlin, Scala, Lua, Perl, Haskell, Shell, Bash, PowerShell, Objective-C, Dart
///
/// Markup / Config:
/// - HTML, CSS, XML, JSON, YAML, Toml, Markdown, Latex
///
/// Data / Build / Meta:
/// - SQL, Csv, Ini, Dockerfile, Makefile, Gradle, Maven, Gitignore, EditorConfig
///
/// Other:
/// - TextPlain: plain text, unstructured
/// - Binary: binary file (e.g., `.exe`)
/// - Unknown: fallback type if no match is found
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Rust,
    C,
    Cpp,
    CSharp,
    Java,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Ruby,
    PHP,
    Swift,
    Kotlin,
    Scala,
    Lua,
    Perl,
    Haskell,
    Shell,
    Bash,
    PowerShell,
    ObjectiveC,
    Dart,

    Html,
    Css,
    Xml,
    Json,
    Yaml,
    Toml,
    Markdown,
    Latex,

    Sql,
    Csv,
    Ini,
    Dockerfile,
    Makefile,
    Gradle,
    Maven,

    Gitignore,
    EditorConfig,

    TextPlain,
    Binary,
    #[serde(other)]
    Unknown,
}

impl Default for FileType {
    /// Returns `FileType::Unknown`.
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<&str> for FileType {
    /// Converts a lowercase-insensitive string into a `FileType` variant.
    ///
    /// Supports alternative aliases like "js" → `JavaScript`, "sh" → `Shell`, etc.
    ///
    /// Returns `FileType::Unknown` for any unrecognized string.
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "rust" => FileType::Rust,
            "c" => FileType::C,
            "cpp" => FileType::Cpp,
            "csharp" | "cs" => FileType::CSharp,
            "java" => FileType::Java,
            "python" => FileType::Python,
            "javascript" | "js" => FileType::JavaScript,
            "typescript" | "ts" => FileType::TypeScript,
            "go" => FileType::Go,
            "ruby" => FileType::Ruby,
            "php" => FileType::PHP,
            "swift" => FileType::Swift,
            "kotlin" => FileType::Kotlin,
            "scala" => FileType::Scala,
            "lua" => FileType::Lua,
            "perl" => FileType::Perl,
            "haskell" => FileType::Haskell,
            "shell" | "sh" => FileType::Shell,
            "bash" => FileType::Bash,
            "powershell" | "ps1" => FileType::PowerShell,
            "objective-c" | "objc" => FileType::ObjectiveC,
            "dart" => FileType::Dart,

            "html" => FileType::Html,
            "css" => FileType::Css,
            "xml" => FileType::Xml,
            "json" => FileType::Json,
            "yaml" | "yml" => FileType::Yaml,
            "toml" => FileType::Toml,
            "markdown" | "md" => FileType::Markdown,
            "latex" | "tex" => FileType::Latex,

            "sql" => FileType::Sql,
            "csv" => FileType::Csv,
            "ini" => FileType::Ini,
            "dockerfile" => FileType::Dockerfile,
            "makefile" => FileType::Makefile,
            "gradle" => FileType::Gradle,
            "maven" => FileType::Maven,

            "gitignore" => FileType::Gitignore,
            "editorconfig" => FileType::EditorConfig,

            "text" | "plain" | "txt" => FileType::TextPlain,
            "binary" | "bin" => FileType::Binary,

            _ => FileType::Unknown,
        }
    }
}

impl std::fmt::Display for FileType {
    /// Converts the `FileType` variant into a printable string using `Debug` format (e.g., `"Rust"`).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl FileType {
    /// Returns the standard file extension (without a dot) associated with the `FileType`.
    ///
    /// # Examples
    /// ```rust
    /// use nibb_core::FileType;
    ///
    /// assert_eq!(FileType::Rust.extension(), "rs");
    /// assert_eq!(FileType::Markdown.extension(), "md");
    /// ```
    ///
    /// For special types like `Dockerfile`, `Makefile`, and `Maven`, the value matches the filename:
    /// - `Dockerfile` → `"dockerfile"`
    /// - `Maven` → `"pom.xml"`
    ///
    /// For `Binary`, the extension is:
    /// - `"exe"` on Windows
    /// - `""` (no extension) on Unix
    ///
    /// Returns an empty string for `Unknown`.
    pub fn extension(&self) -> &'static str {
        match self {
            FileType::Rust => "rs",
            FileType::C => "c",
            FileType::Cpp => "cpp",
            FileType::CSharp => "cs",
            FileType::Java => "java",
            FileType::Python => "py",
            FileType::JavaScript => "js",
            FileType::TypeScript => "ts",
            FileType::Go => "go",
            FileType::Ruby => "rb",
            FileType::PHP => "php",
            FileType::Swift => "swift",
            FileType::Kotlin => "kt",
            FileType::Scala => "scala",
            FileType::Lua => "lua",
            FileType::Perl => "pl",
            FileType::Haskell => "hs",
            FileType::Shell => "sh",
            FileType::Bash => "bash",
            FileType::PowerShell => "ps1",
            FileType::ObjectiveC => "m",
            FileType::Dart => "dart",

            FileType::Html => "html",
            FileType::Css => "css",
            FileType::Xml => "xml",
            FileType::Json => "json",
            FileType::Yaml => "yaml",
            FileType::Toml => "toml",
            FileType::Markdown => "md",
            FileType::Latex => "tex",

            FileType::Sql => "sql",
            FileType::Csv => "csv",
            FileType::Ini => "ini",
            FileType::Dockerfile => "dockerfile",
            FileType::Makefile => "makefile",
            FileType::Gradle => "gradle",
            FileType::Maven => "pom.xml",

            FileType::Gitignore => "gitignore",
            FileType::EditorConfig => "editorconfig",

            FileType::TextPlain => "txt",
            #[cfg(windows)]
            FileType::Binary => "exe",
            #[cfg(unix)]
            FileType::Binary => "",
            FileType::Unknown => "",
        }
    }
    /// Attempts to guess the [`FileType`] from a file extension (case-insensitive).
    ///
    /// # Arguments
    ///
    /// * `ext` - A file extension, with or without a leading dot (e.g. `"rs"` or `".rs"`).
    ///
    /// # Returns
    ///
    /// The corresponding [`FileType`] if recognized, otherwise [`FileType::Unknown`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nibb_core::FileType;
    ///
    /// assert_eq!(FileType::from_extension("rs"), FileType::Rust);
    /// assert_eq!(FileType::from_extension(".py"), FileType::Python);
    /// assert_eq!(FileType::from_extension("unknown_ext"), FileType::Unknown);
    /// ```
    pub fn from_extension(ext: &str) -> Self {
        let ext = ext.trim_start_matches('.').to_lowercase();

        match ext.as_str() {
            "rs" => FileType::Rust,
            "c" => FileType::C,
            "cpp" => FileType::Cpp,
            "cs" => FileType::CSharp,
            "java" => FileType::Java,
            "py" => FileType::Python,
            "js" => FileType::JavaScript,
            "ts" => FileType::TypeScript,
            "go" => FileType::Go,
            "rb" => FileType::Ruby,
            "php" => FileType::PHP,
            "swift" => FileType::Swift,
            "kt" => FileType::Kotlin,
            "scala" => FileType::Scala,
            "lua" => FileType::Lua,
            "pl" => FileType::Perl,
            "hs" => FileType::Haskell,
            "sh" => FileType::Shell,
            "bash" => FileType::Bash,
            "ps1" => FileType::PowerShell,
            "m" => FileType::ObjectiveC,
            "dart" => FileType::Dart,

            "html" => FileType::Html,
            "css" => FileType::Css,
            "xml" => FileType::Xml,
            "json" => FileType::Json,
            "yaml" | "yml" => FileType::Yaml,
            "toml" => FileType::Toml,
            "md" => FileType::Markdown,
            "tex" => FileType::Latex,

            "sql" => FileType::Sql,
            "csv" => FileType::Csv,
            "ini" => FileType::Ini,
            "dockerfile" => FileType::Dockerfile,
            "makefile" => FileType::Makefile,
            "gradle" => FileType::Gradle,
            "pom.xml" => FileType::Maven,

            "gitignore" => FileType::Gitignore,
            "editorconfig" => FileType::EditorConfig,

            "txt" | "text" | "plain" => FileType::TextPlain,
            "bin" | "exe" => FileType::Binary,

            _ => FileType::Unknown,
        }
    }
}

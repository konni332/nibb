# Nibb

A fast and extensible code snippet manager - written in Rust.  
Primarily for editor integration, such as NeoVim, VsCode and JetBrains IDEs, but
it provides a CLI as well.

### Features

- CRUD CLI for managing code snippets
- Toml based configuration
- `Git` integration (auto commit, push on commit, ...)

---

### Installation


### Usage
```text
Usage: nibb.exe <COMMAND>

Commands:
new     Create a new snippet
list    List snippets
delete  Delete a snippet
edit    Edit an existing snippet
help    Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version
```

### Example

````shell
nibb new "My new snippet" 
````


---
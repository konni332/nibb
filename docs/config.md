# Nibb config guide

This guide gives an explanation for the `Nibb` `config.toml`.  
This config should be located in `$HOMEDIR/.nibb/` on your system.


## Default config.toml
`````toml
[git]
enabled = false
auto_commit = false
commit_message = "update: {name} @{modified}"
author = "AUTHOR NAME"
author_email = "AUTHOR EMAIL"
push_on_commit = false
branch = "master"
`````

---

## `[git]` Configuration

| Key              | Type   | Default                        | Description                                                 |
|------------------|--------|--------------------------------|-------------------------------------------------------------|
| `enabled`        | bool   | `false`                        | Enables or disables Git integration.                        |
| `auto_commit`    | bool   | `false`                        | Automatically commit changes after snippet updates.         |
| `commit_message` | string | `"update: {name} @{modified}"` | Commit message template. Supports placeholders (see below). |
| `author`         | string | `"AUTHOR NAME"`                | The name to use in Git commits.                             |
| `author_email`   | string | `"AUTHOR EMAIL"`               | The email to use in Git commits.                            |
| `push_on_commit` | bool   | `false`                        | Whether to push changes to the remote after commit.         |
| `branch`         | string | `"master"`                     | The branch to which changes are pushed.                     |
| `remote`         | string | `None`                         | The remote url to push to                                   |
---

### Available Template Placeholders for `commit_message`

You can include the following placeholders in your commit message using `{...}` syntax:

| Placeholder     | Description                         |
|-----------------|-------------------------------------|
| `{name}`        | Snippet name                        |
| `{description}` | Snippet description                 |
| `{tags}`        | Comma-separated list of tags        |
| `{created}`     | Creation timestamp (RFC3339)        |
| `{modified}`    | Last modification timestamp         |
| `{language}`    | Programming language of the snippet |
| `{content}`     | The snippet's full content          |

---


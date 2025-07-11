![License](https://img.shields.io/github/license/konni332/nibb)

# Nibb

A fast and extensible code snippet manager written in Rust.  
Primarily for editor integration such as NeoVim, VsCode and JetBrains IDEs, but
it provides a CLI as well.

## Features

- CRUD CLI for managing code snippets
- Toml based configuration
- `Git` integration (auto commit, push on commit, ...)

---

## Installation

### Quickstart

Install the nibb package using cargo install, for fastest setup

````shell
cargo install nibb
````

### Build

````shell
git clone https://github.com/konni332/nibb
cd nibb
cargo build --release
````

This will produce the following in `target/release`:
- ``nibb`` & ``libnibb_core.so`` (Linux)
- ``nibb`` & ``libnibb_core.dylib`` (macOS)
- ``nibb.exe`` & ``nibb_core.dll`` (Windows)


**Now you have a working version of the Nibb-CLI(`nibb(.exe)`)**
**Now you can use the shared libraries (`nibb_core.(dll/so)`) FFI functions. See [FFI](./docs/ffi.md) for more information**

---

#### Optional

You can move the binaries to global paths

##### Linux / macOS users

````shell
sudo cp target/release/nibb /usr/local/bin/nibb
sudo cp target/release/libnibb_core.so /usr/local/lib/libnibb_core.so
````

---

##### Windows users
Copy nibb.exe and nibb_core.dll somewhere into your PATH or link to them directly in your application.

---

## Usage

### CLI

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

#### Example

````shell
nibb new "My new snippet" --tags "new-snippet, test-snippet" --content "Put your code here" --d "An ambigiuous description"
nibb list --filter "test"
nibb edit "My new snippet" language "C"
````

---

### Library

See [FFI](./docs/ffi.md) for information on how to use the *Foreign Function Interface*

---

## Info

**Nibb is not intended as an *"CLI first"* snippet tool. Nibb supplies an engine, that can be used to manage snippets in
editor integrations / plugins.**

*For an example of such integration have a look at [LuaNibb](https://github.com/konni332/luanibb)*

*Nibb can be compiled as a C compatible shared library (`.dll`/`.so`/`.dylib`) and provides `FFI` functions to
manage snippets on a **Systems** level.*


---

## Documentation

- [FFI Overview](./docs/ffi.md)
- [Configuration Guide](./docs/config.md)
- [Plugin Example](https://github.com/konni332/luanibb)

---

## License

This project is licensed under either of

- MIT license ([LICENSE-MIT](./LICENSE-MIT) or https://opensource.org/license/MIT)
- Apache license, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE.md) or https://www.apache.org/license/LICENSE-2.0)

---
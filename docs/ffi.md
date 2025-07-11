# Foreign Function Interface (FFI) for `Nibb`

This document describes the C-compatible functions, exposed by ``Nibb``  for integration with  
Lua, NeoVim, Typescript, Python, or any other **FFI-capable** environments

---

## Building with FFI enabled

Compile ``nibb`` as a dynamic library:

````shell
cargo build --release --lib
````
This will produce the shared library in `target/release`:  
- ``nibb_core.so`` (Linux)  
- ``nibb_core.dylib`` (macOS)  
- ``nibb_core.dll`` (Windows)


Or to compile both library and binary:
````shell
cargo build --release
````
This will produce the following in `target/release`:
- The above
- ``nibb`` (Linux, macOS)
- ``nibb.exe`` (Windows)

---

### Safety & Memory Management

All functions returning string return pointers to heap-allocated memory (`char*`).

You **must** free them manually using:
```c
void free_string_ffi(char* s);
```
Failure to do so may lead to **memory leaks!**

---

## Available FFI functions

All FFI functions have detailed docs in [ffi.rs](../src/ffi/ffi.rs) or the ``C`` header file [nibb.h](../include/nibb.h)  
To see detailed safety documentation, arguments, and returns please visit these code documentations

---

#### load_snippet_ffi

```rust
pub extern "C" fn load_snippet_ffi(name: *const c_char) -> *mut c_char {}
```
```c
char *load_snippet_ffi(const char *name);
```

*Loads snippet by name and returns its JSON representation*

---

#### save_snippet_ffi

```rust
pub extern "C" fn save_snippet_ffi(snippet_json: *const c_char) -> bool {}
```
```c
bool save_snippet_ffi(const char *snippet_json);
```

*Saves a single snippet from its JSON representation*

---

#### delete_snippet_ffi

```rust
pub extern "C" fn delete_snippet_ffi(name: *const c_char) -> bool {}
```
```c
bool delete_snippet_ffi(const char *name);
```

*Deletes a snippet by its name from the repo*

---

#### load_all_ffi

````rust
pub extern "C" fn load_all_ffi() -> *mut c_char {}
````
````c
char *load_all_ffi(void);
````

*Loads all snippets from the repo and returns them as JSON array*

---

#### save_all_ffi

````rust
pub extern "C" fn save_all_ffi(snippets_json: *const c_char) -> bool {}
````
````c
bool save_all_ffi(const char *snippets_json);
````

*Saves a list of snippets from a JSON array*

---

#### nibb_git_generic_ffi

````rust
pub extern "C" fn nibb_git_generic_ffi(args: *const c_char) -> *const c_char{}
````
````c
const char *nibb_git_generic_ffi(const char *args);
````

*Executes a generic Git command inside the `.nibb` directory and returns the output as JSON*

#### free_string_ffi

````rust
pub extern "C" fn free_string_ffi(s: *mut c_char) {}
````
````c
void free_string_ffi(char *s);
````

*Frees a string previously allocated and returned by an FFI function.*

---

## Example (Lua)

**You can use the FFI in Lua as follows**

````lua
local ffi = require("ffi")

ffi.cdef[[
    char *load_snippet_ffi(const char *name);
    bool save_snippet_ffi(const char *snippet_json);
    char *load_all_ffi(void);
    bool save_all_ffi(const char *snippet_json);
    bool delete_snippet_ffi(const char *name);
    void free_string_ffi(char *s);
    char *nibb_git_generic_ffi(const char *args);
]]

local ok, engine = pcall(ffi.load, "PATH/TO/LIB/nibb_core.<ext>")
if not ok then
        error("Failed to load nibb_core library backend: " .. tostring(engine))
end
````
**This is a code snippet from [LuaNibb](https://github.com/konni332/luanibb).**  
**FFI usage will differ based on the environment!**

---

## Example (Kotlin)

**You can use the FFI in Kotlin as follows**

````kotlin
dependencies {
    implementation("net.java.dev.jna:jna:5.13.0") // or more recent version
}
````
`````kotlin
import com.sun.jna.*
import com.sun.jna.ptr.PointerByReference

// Interface binding to nibb_core
interface NibbLib : Library {
    companion object {
        val INSTANCE: NibbLib = Native.load(
            System.getenv("NIBB_LIB_PATH") ?: "nibb_core",  // e.g. "libnibb_core.so" or "nibb_core.dll"
            NibbLib::class.java
        )
    }

    fun load_all_ffi(): Pointer
    fun free_string_ffi(ptr: Pointer)
}

fun main() {
    val lib = NibbLib.INSTANCE

    val resultPtr = lib.load_all_ffi()
    val json = resultPtr.getString(0)
    println("Snippets: $json")

    // Important: Free the string!
    lib.free_string_ffi(resultPtr)
}
`````

**This is a fictional example in Kotlin**

---

---
/* Generated with cbindgen:0.29.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Loads a snippet by name and returns its JSON representation.
 *
 * # Arguments
 * - `name`: A null-terminated C string representing the name of the snippet.
 *
 * # Returns
 * A newly allocated C string (`*mut c_char`) containing the snippet's JSON representation.
 * - On success: JSON-encoded `Snippet` as a C string (must be freed with `free_string_ffi`).
 * - On failure: JSON-encoded error object (must also be freed).
 *
 * # Safety
 * - `name` must be a valid, null-terminated UTF-8 string.
 * - Caller is responsible for freeing the returned string using `free_string_ffi`.
 */
char *load_snippet_ffi(const char *name);

/**
 * Saves a single snippet from its JSON representation.
 *
 * # Arguments
 * - `snippet_json`: A null-terminated C string containing a JSON-encoded snippet.
 *
 * # Returns
 * - `true` if the snippet was saved successfully.
 * - `false` if the input was invalid or saving failed.
 *
 * # Safety
 * - `snippet_json` must be a valid, null-terminated UTF-8 string.
 */
bool save_snippet_ffi(const char *snippet_json);

/**
 * Deletes a snippet from the repo
 *
 * # Arguments
 * - `name`: A Null terminated C String of the snippets' name.
 *
 * # Returns
 * - `true` if the snippet was deleted successfully.
 * - `false` if an error occurred.
 *
 * # Safety
 * - `name` needs to be a valid, null-terminated, UTF-8 string.
 */
bool delete_snippet_ffi(const char *name);

/**
 * Loads all snippets from the repository and returns them as a JSON array.
 *
 * # Returns
 * A newly allocated C string (`*mut c_char`) containing the JSON array of all snippets.
 * - On success: JSON array of snippets (must be freed with `free_string_ffi`).
 * - On failure: JSON-encoded error object (must also be freed).
 *
 * # Safety
 * - Caller is responsible for freeing the returned string using `free_string_ffi`.
 */
char *load_all_ffi(void);

/**
 * Saves a list of snippets from a JSON array.
 *
 * # Arguments
 * - `snippets_json`: A null-terminated C string containing a JSON array of snippets.
 *
 * # Returns
 * - `true` if all snippets were saved successfully.
 * - `false` if deserialization or saving failed.
 *
 * # Safety
 * - `snippets_json` must be a valid, null-terminated UTF-8 string.
 */
bool save_all_ffi(const char *snippets_json);

/**
 * Frees a string previously allocated and returned by an FFI function.
 *
 * # Arguments
 * - `s`: A pointer returned by an FFI function like `load_snippet_ffi` or `load_all_ffi`.
 *
 * # Safety
 * - `s` must be a pointer obtained from one of the FFI functions using `CString::into_raw`.
 * - Passing a null pointer is safe and does nothing.
 * - After calling this function, `s` must not be used again.
 */
void free_string_ffi(char *s);

//! # FFI interface for Nib
//!
//! These functions provide a C-compatible API for interacting with the snippet system.
//! All returned strings must be freed using [`free_string_ffi`].
//!
//! Designed for use in Lua, C, Python, TypeScript, and other native environments.
pub mod ffi;
mod ffi_utils;
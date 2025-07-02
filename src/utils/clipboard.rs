#![allow(dead_code)]
use arboard::Clipboard;
use crate::errors::NibbError;

/// Copies the string to the systems clipboard
pub fn copy_to_clipboard(str: &str) -> Result<(), NibbError>{
    let mut clipboard = Clipboard::new()
        .map_err(|e| NibbError::ClipboardError(e.to_string()))?;
    clipboard.set_text(str.to_owned()).map_err(|e| NibbError::ClipboardError(e.to_string()))?;
    Ok(())
}
/// Gets content from the systems clipboard
pub fn paste_from_clipboard() -> Result<String, NibbError> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| NibbError::ClipboardError(e.to_string()))?;
    Ok(clipboard.get_text().map_err(|e| NibbError::ClipboardError(e.to_string()))?)
}
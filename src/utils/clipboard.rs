use arboard::Clipboard;

pub fn copy_to_clipboard(str: &str) -> Result<(), String>{
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(str.to_owned()).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn paste_from_clipboard() -> Result<String, String> {
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    Ok(clipboard.get_text().map_err(|e| e.to_string())?)
}
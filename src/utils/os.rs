use which::which;

pub fn editor_available(editor: &str) -> bool {
    which(editor).is_ok()
}

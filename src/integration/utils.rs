use crate::config::settings::Settings;
use crate::utils::fs::ensure_nibb_structure;

pub fn extern_setup() -> Result<Settings, String> {
    ensure_nibb_structure()?;
    let cfg = Settings::load()?;
    Ok(cfg)
}
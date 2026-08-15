use crate::config::assembler::assemble_config;
use crate::sys::profile::SystemProfile;
use std::fs;
use std::path::PathBuf;

pub fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config/ch/config.toml")
}

pub fn clean_config() -> Result<(), String> {
    let path = get_config_path();
    if path.exists() {
        fs::remove_file(&path)
            .map_err(|e| format!("Failed to remove config file: {}", e))?;
        println!("[ch] Configuration file was removed successfully");
    } else {
        println!("[ch] There's no configuration file found to be cleaned, like there isn't at all");
    }
    Ok(())
}

pub fn update_config(profile: &SystemProfile) -> Result<PathBuf, String> {
    let path = get_config_path();
    if path.exists() {
        fs::remove_file(&path)
            .map_err(|e| format!("Failed to remove old config file: {}", e))?;
    }
    init_config(profile)
}

pub fn init_config(profile: &SystemProfile) -> Result<PathBuf, String> {
    let config_path = get_config_path();

    if config_path.exists() {
        return Err("Config file already exists. Use '--update' to... update it.".to_string());
    }

    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    // Inject assembled fragments
    let assembled_toml = assemble_config(profile)?;

    fs::write(&config_path, assembled_toml.trim())
        .map_err(|e| format!("Failed to write config file hmph!: {}", e))?;

    Ok(config_path)
}
use crate::config::assembler::assemble_config;
use crate::sys::profile::SystemProfile;
use std::fs;
use std::path::PathBuf;
use include_dir::{include_dir, Dir};
use crate::sys::backup::BackupManager;

static SCRIPTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/scripts");

pub fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config/ch/config.toml")
}

fn extract_scripts(os_name: &str) -> Result<(), String> { // now there's support for specific scripts
    let mut target_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    target_dir.push("ch");
    target_dir.push("scripts");
    target_dir.push(os_name);

    if let Some(os_scripts) = SCRIPTS_DIR.get_dir(os_name) {
        fs::create_dir_all(&target_dir).map_err(|e| format!("Failed to create scripts dir: {}", e))?;

        for file in os_scripts.files() {
            if let Some(file_name) = file.path().file_name() {
                let dest_path = target_dir.join(file_name);
                fs::write(&dest_path, file.contents())
                    .map_err(|e| format!("Failed to write script: {}", e))?;

                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = fs::metadata(&dest_path) {
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o755);
                        let _ = fs::set_permissions(&dest_path, perms);
                    }
                }
            }
        }
        println!("[ch] Scripts for '{}' extracted and made executable.", os_name);
    }
    Ok(())
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

pub fn backup(category: &str, file_path: &str) -> Result<(), String> {
    let manager = BackupManager::new();
    let dest = manager.backup(category, file_path)?;
    println!("[ch] Backup successful.");
    println!("Saved at: {}", dest.display());
    Ok(())
}

pub fn clean_backups(category: Option<&str>) -> Result<(), String> {
    let manager = BackupManager::new();
    manager.clean(category)?;
    println!("[ch] Backups cleaned successfully.");
    Ok(())
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
    extract_scripts(&profile.os)?;

    Ok(config_path)
}
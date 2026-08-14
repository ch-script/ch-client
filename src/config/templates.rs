use std::fs;
use std::path::PathBuf;


// IK IK ITS QUITE UNORGANIZED FOR NOW BUT IT WORKS
pub const ARCH_TEMPLATE: &str = include_str!("../../templates/archlinux.toml");
pub const FALLBACK_TEMPLATE: &str = include_str!("../../templates/fallback.toml");
pub const NIXOS_TEMPLATE: &str = include_str!("../../templates/nixos.toml");
pub const SOLUS_TEMPLATE: &str = include_str!("../../templates/solus.toml");
pub const UBUNTU_TEMPLATE: &str = include_str!("../../templates/ubuntu.toml");
pub const VOID_TEMPLATE: &str = include_str!("../../templates/void.toml");

pub fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config/ch/config.toml")
}

pub fn get_template(distro_id: &str) -> &'static str {
    // NORMALIZE INPUT lowercase and strip spaces, dashes, underscores
    let normalized: String = distro_id
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-' && *c != '_')
        .collect();

    match normalized.as_str() {
        "nixos" | "nix" => NIXOS_TEMPLATE,
        "arch" | "archlinux" => ARCH_TEMPLATE,
        "solus" | "soluslinux" => SOLUS_TEMPLATE,
        "ubuntu" | "debian" => UBUNTU_TEMPLATE,
        "void" | "voidlinux" => VOID_TEMPLATE,
        _ => FALLBACK_TEMPLATE,
    }
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

pub fn update_config(distro_id: &str) -> Result<PathBuf, String> {
    let path = get_config_path();
    if path.exists() {
        fs::remove_file(&path)
            .map_err(|e| format!("Failed to remove old config file: {}", e))?;
    }
    init_config(distro_id)
}

pub fn init_config(distro_id: &str) -> Result<PathBuf, String> {
    let config_path = get_config_path();

    if config_path.exists() {
        return Err("Config file already exists. Use '--update' to... update it.".to_string());
    }

    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let template = get_template(distro_id);

    fs::write(&config_path, template.trim())
        .map_err(|e| format!("Failed to write config file hmph!: {}", e))?;

    Ok(config_path)
}
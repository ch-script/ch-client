use super::templates::get_config_path;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(rename = "active_distro")]
    pub _active_distro: String,
    #[serde(flatten)]
    pub modules: HashMap<String, HashMap<String, String>>,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let path = get_config_path();
        if !path.exists() {
            return Err("No configuration file found. Run 'ch --create' to initialize.".to_string());
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse TOML config: {}", e))
    }
}
use crate::config::model::Config;
use crate::sys::profile::SystemProfile;
use include_dir::{include_dir, Dir};
use std::collections::HashMap;

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

// Now we're dynamic!!!!! 
fn get_fragment(category: &str, name: &str) -> Option<String> {
    let path = format!("{}/{}.toml", category, name);
    
    TEMPLATES_DIR
        .get_file(&path)
        .and_then(|file| file.contents_utf8().map(|s| s.to_string()))
}

pub fn assemble_config(profile: &SystemProfile) -> Result<String, String> {
    let mut merged_modules: HashMap<String, HashMap<String, String>> = HashMap::new();

    let mut fragment_keys = vec![
        ("core", profile.core_utils.as_str()),
        ("pkg", profile.pkg_manager.as_str()),
        ("init", profile.init_system.as_str()),
        ("os", profile.os.as_str()),
        ("user", profile.user_manager.as_str()),
        ("net", profile.net_manager.as_str()),
    ];

    if let Some(gpu) = &profile.gpu {
        fragment_keys.push(("gpu", gpu.as_str()));
    }
    if let Some(wm) = &profile.wm_de {
        fragment_keys.push(("wm", wm.as_str()));
    }

    for (category, name) in fragment_keys {
        if let Some(template_str) = get_fragment(category, name) {
            if let Ok(parsed_modules) = toml::from_str::<HashMap<String, HashMap<String, String>>>(&template_str) {
                for (module_name, actions) in parsed_modules {
                    merged_modules
                        .entry(module_name)
                        .or_default()
                        .extend(actions);
                }
            } else {
                eprintln!("[ch] Warning: Failed to parse fragment {}/{}", category, name);
            }
        }
    }

    let final_config = Config {
        _active_distro: profile.os.clone(),
        modules: merged_modules,
    };

    toml::to_string(&final_config).map_err(|e| format!("Failed to build final config: {}", e))
}
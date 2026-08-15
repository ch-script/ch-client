use crate::config::model::Config;
use inquire::{Confirm, Select, Text};

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn run_interactive_help(config: &Config) -> Option<(String, String, Vec<String>)> {
    if config.modules.is_empty() {
        println!("No modules available in your config file, that's strange... Maybe we couldn't detect your OS :c");
        return None;
    }
    navigate_menu("Main Menu", &config.modules, vec![])
}

fn navigate_menu(
    menu_name: &str,
    current_table: &toml::map::Map<String, toml::Value>,
    path: Vec<String>,
) -> Option<(String, String, Vec<String>)> {
    loop {
        clear_screen();
        crate::ui::theme::print_logo();

        let mut keys: Vec<String> = current_table.keys().cloned().collect();
        keys.sort();

        if keys.is_empty() {
            println!("Under construction '{}'.", menu_name);
            return None;
        }

        let mut options: Vec<String> = vec!["Go back".to_string()];
        
        for key in &keys {
            match current_table.get(key).unwrap() {
                toml::Value::Table(t) => {
                    if !t.contains_key("cmd") {
                        options.push(format!("{:<18}", key));
                    } else {
                        let cmd_str = t.get("cmd").and_then(|v| v.as_str()).unwrap_or("");
                        options.push(format!("{:<18} - ({})", key, cmd_str));
                    }
                }
                toml::Value::String(s) => {
                    options.push(format!("{:<18} - ({})", key, s));
                }
                _ => {}
            }
        }

        let prompt_msg = if path.is_empty() {
            "Select a module to explore:".to_string()
        } else {
            format!("Path: {} > Select an option:", path.join(" > "))
        };

        let selected_option = match Select::new(&prompt_msg, options).prompt() {
            Ok(opt) => opt,
            Err(_) => return None,
        };

        if selected_option == "Go back" {
            return None;
        }

        let selected_key = selected_option.split_whitespace().next().unwrap().to_string();
        let value = current_table.get(&selected_key).unwrap();

        match value {
            // submenu or advanced command
            toml::Value::Table(t) => {
                if let Some(cmd_val) = t.get("cmd").and_then(|v| v.as_str()) {
                    // advanced comm
                    if let Some(confirm_type) = t.get("confirm").and_then(|v| v.as_str()) {
                        let msg = t.get("msg").and_then(|v| v.as_str()).unwrap_or("Are you sure?");
                        
                        if confirm_type == "yesorno" {
                            let ans = Confirm::new(msg).with_default(false).prompt();
                            if ans.is_err() || !ans.unwrap() { continue; }
                        } else if confirm_type.starts_with("match:") {
                            let required = confirm_type.strip_prefix("match:").unwrap();
                            let ans = Text::new(msg).prompt();
                            if ans.is_err() || ans.unwrap() != required {
                                println!("Confirmation failed. Press Enter to continue...");
                                let _ = Text::new("").prompt();
                                continue;
                            }
                        }
                    }
                    
                    // advance
                    if let Some(extra_args) = prompt_required_tokens(cmd_val) {
                        return Some((menu_name.to_string(), selected_key, extra_args));
                    }
                } else {
                    // submenu
                    let mut new_path = path.clone();
                    new_path.push(selected_key.clone());
                    
                    if let Some(result) = navigate_menu(&selected_key, t, new_path) {
                        return Some(result);
                    }
                }
            }
            // normal command
            toml::Value::String(cmd_raw) => {
                if let Some(extra_args) = prompt_required_tokens(cmd_raw) {
                    return Some((menu_name.to_string(), selected_key, extra_args));
                }
            }
            _ => continue,
        }
    }
}

fn prompt_required_tokens(cmd_raw: &str) -> Option<Vec<String>> {
    let mut extra_args = Vec::new();
    let mut i = 1;

    loop {
        let token = format!("{{{}}}", i);
        if cmd_raw.contains(&token) {
            let input = match Text::new(&format!("Enter value for {}:", token)).prompt() {
                Ok(val) => val,
                Err(_) => return None,
            };

            if input.trim().is_empty() {
                println!("[ch] The argument cannot be empty.");
                return None;
            }

            extra_args.push(input.trim().to_string());
            i += 1;
        } else {
            break;
        }
    }
    Some(extra_args)
}
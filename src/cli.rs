use crate::config::model::Config;
use crate::config::templates::{clean_config, init_config, update_config, backup, clean_backups};
use crate::sys::executor::execute;
use crate::ui::interactive::run_interactive_help;
use std::env;

pub fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    // Process flags
    if args.len() > 1 && process_flags(&args)? {
        return Ok(());
    }

    let config = Config::load()?;

    let (module, action, extra_args) = resolve_target(&args, &config)?;

    let actions = config
        .modules
        .get(&module)
        .ok_or_else(|| format!("Module '{}' not found in config", module))?;

    let cmd_raw = actions
        .get(&action)
        .ok_or_else(|| format!("Action '{}' not found in module '{}'", action, module))?;

    let actual_cmd_str = match cmd_raw {
        toml::Value::String(s) => s.as_str(),
        toml::Value::Table(t) => {
            t.get("cmd")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| panic!("Error: The advanced command does not have a cmd valid camp"))
        },
        _ => panic!("Error: Invalid command format for the TOML"),
    };

    execute(actual_cmd_str, &extra_args)
}

fn process_flags(args: &[String]) -> Result<bool, String> {
    let mut forced_distro = None;

    for i in 1..args.len() {
        if let Some(distro) = args[i].strip_prefix("--force=") {
            forced_distro = Some(distro.to_string());
        } else if args[i] == "--force" && i + 1 < args.len() {
            forced_distro = Some(args[i + 1].clone());
        }
    }

    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];

        if arg == "--create" {
            // tuf logo
            crate::ui::theme::print_logo(); // ik ik ill use "use" later i js want tuf logo


            let profile = crate::sys::detector::build_system_profile(forced_distro.clone());
            println!("[ch] Profile detected: {:?}", profile);
            init_config(&profile)?;
            println!("[ch] Conf created! :D all good!");
            return Ok(true);
            
        } else if arg == "--update" {
            let profile = crate::sys::detector::build_system_profile(forced_distro.clone());
            update_config(&profile)?;
            println!("[ch] Upgraded :D");
            return Ok(true);
            
        } else if arg == "--clean" {
            clean_config()?;
            println!("[ch] Cleaned! :c");
            return Ok(true);
            
        } else if arg == "--backup" {
            if i + 2 < args.len() {
                let category = &args[i + 1];
                let file_path = &args[i + 2];
                backup(category, file_path)?;
                return Ok(true);
            } else {
                return Err("Usage: ch --backup <category> <file_path>".to_string());
            }
            
        } else if arg == "--clean-backups" {
            let category = if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                Some(args[i + 1].as_str())
            } else {
                None
            };
            clean_backups(category)?;
            return Ok(true);
        }
        
        i += 1;
    }

    Ok(false)
}

fn resolve_target(
    args: &[String],
    config: &Config,
) -> Result<(String, String, Vec<String>), String> {
    if args.len() >= 3 {
        Ok((args[1].clone(), args[2].clone(), args[3..].to_vec()))
    } else if args.len() == 1 || (args.len() == 2 && args[1] == "help") {
        match run_interactive_help(config) {
            Some((m, a, extra_args)) => Ok((m, a, extra_args)),
            None => Err("Canceled operation".to_string()),
        }
    } else {
        Err("Usage: ch <module> <action> [args...]\n   or: ch (for interactive super easy mode :D)".to_string())
    }
}
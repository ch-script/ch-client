use crate::config::model::Config;
use inquire::{InquireError, Select, Text};

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn run_interactive_help(config: &Config) -> Option<(String, String, Vec<String>)> {
    let mut modules: Vec<String> = config.modules.keys().cloned().collect();
    modules.sort();

    if modules.is_empty() {
        println!("No modules available in your config file, that's strange... Maybe we couldn't detect your OS :c");
        return None;
    }

    loop {
        clear_screen();
        crate::ui::theme::print_logo(); // tuf logo
        let selected_module = match Select::new("Select a module to explore:", modules.clone()).prompt() {
            Ok(m) => m,
            Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => return None,
            Err(_) => return None,
        };

        let actions_map = match config.modules.get(&selected_module) {
            Some(map) => map,
            None => continue,
        };

        let mut action_keys: Vec<String> = actions_map.keys().cloned().collect();
        action_keys.sort();

        if action_keys.is_empty() {
            println!("No actions available inside '{}'.", selected_module);
            continue;
        }

        let mut options: Vec<String> = vec!["Go back".to_string()];
        options.extend(action_keys.iter().map(|key| {
            let cmd = actions_map.get(key).unwrap();
            format!("{:<18} - ({})", key, cmd)
        }));

        let selected_option = match Select::new(
            &format!("Select an action inside '{}':", selected_module),
            options,
        )
        .prompt()
        {
            Ok(opt) => opt,
            Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => return None,
            Err(_) => return None,
        };

        if selected_option == "Go back" {
            continue;
        }

        let selected_action = match selected_option.split_whitespace().next() {
            Some(act) => act.to_string(),
            None => continue,
        };

        let cmd_raw = match actions_map.get(&selected_action) {
            Some(cmd) => cmd,
            None => continue,
        };

        match prompt_required_tokens(cmd_raw) {
            Some(extra_args) => return Some((selected_module, selected_action, extra_args)),
            None => return None,
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
                Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => return None,
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
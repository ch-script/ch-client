use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn detect_os_id() -> Option<String> {
    let file = File::open("/etc/os-release").ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        if let Some(id) = line.strip_prefix("ID=") {
            return Some(id.trim_matches('"').to_string());
        }
    }
    None
}
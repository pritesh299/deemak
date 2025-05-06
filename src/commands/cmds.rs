use super::*;

pub fn cmd_manager(parts: &[&str]) -> String {
    match parts[0] {
        "echo" => echo(&parts[1..]),
        _ => "Command not found. Try `help`.".to_string(),
    }
}

use crate::config;
use std::process::Command;

pub fn run() {
    let pattern = std::env::args().nth(2).expect("No name provided");
    Command::new(config::FIND_COMMAND)
        .arg(format!("{}", pattern))
        .current_dir(config::DIRECTORY)
        .status()
        .expect("Failed to open file.");
}

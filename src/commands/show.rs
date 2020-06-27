use crate::config;
use std::process::Command;

pub fn run() {
    let name = std::env::args().nth(2).expect("No name provided");
    Command::new(config::SHOW_COMMAND)
        .arg(format!("{}.{}", name, config::EXTENSION))
        .current_dir(config::LOCATION)
        .status()
        .expect("Failed to open file.");
}

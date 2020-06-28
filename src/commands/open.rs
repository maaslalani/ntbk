use crate::config;
use crate::util::*;
use std::process::Command;

pub fn run() {
    let name = if args_len() < 3 {
        choose_note()
    } else {
        get_argument(2)
    };

    Command::new(config::EDITOR)
        .arg(format!("{}{}", name, config::EXTENSION))
        .current_dir(&config::directory())
        .status()
        .expect("Failed to open file.");
}

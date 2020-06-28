use crate::config;
use crate::util::*;
use std::process::Command;

pub fn run() {
    let pattern = if args_len() < 3 {
        get_input()
    } else {
        get_argument(2)
    };

    Command::new(config::FIND_COMMAND)
        .arg(format!("{}", pattern))
        .current_dir(&config::directory())
        .status()
        .expect("Failed to open file.");
}

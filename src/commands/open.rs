use crate::config;
use crate::util::{args_len,choose_note,get_argument};
use std::process::Command;

pub fn run() {
    let name = if args_len() < 3 {
        choose_note()
    } else {
        get_argument(2)
    };

    Command::new(config::EDITOR)
        .arg(format!("{}{}", name, config::EXTENSION))
        .current_dir(config::DIRECTORY)
        .status()
        .expect("Failed to open file.");
}

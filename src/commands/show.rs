use crate::config;
use crate::util;
use std::process::Command;

pub fn run() {
    let name = if std::env::args().len() < 3 {
        util::choose_note()
    } else {
        std::env::args().nth(2).unwrap()
    };

    Command::new(config::SHOW_COMMAND)
        .arg(format!("{}.{}", name, config::EXTENSION))
        .current_dir(config::DIRECTORY)
        .status()
        .expect("Failed to open file.");
}

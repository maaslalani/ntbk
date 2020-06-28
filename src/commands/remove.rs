use crate::config;
use crate::util;
use std::fs;

pub fn run() {
    let name = if std::env::args().len() < 3 {
        util::choose_note()
    } else {
        std::env::args().nth(2).unwrap()
    };

    match fs::remove_file(format!("{}{}{}", config::DIRECTORY, name, config::EXTENSION)) {
        Ok(_) => println!("Note {} removed.", name),
        Err(_) => println!("Failed to remove note."),
    };
}

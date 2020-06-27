use crate::config;
use std::fs;

pub fn run() {
    let name = std::env::args().nth(2).expect("No name provided");
    match fs::remove_file(format!("{}/{}.{}", config::DIRECTORY, name, config::EXTENSION)) {
        Ok(_) => println!("Note {} removed.", name),
        Err(_) => println!("Failed to remove note."),
    };
}

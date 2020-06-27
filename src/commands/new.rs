use crate::config;
use std::fs::File;

pub fn run() {
    let name = std::env::args().nth(2).expect("No name provided");
    match File::create(format!("{}/{}.{}", config::LOCATION, name, config::EXTENSION)) {
        Ok(_) => println!("Note {} created.", name),
        Err(_) => println!("Failed to create note."),
    };
}

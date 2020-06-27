use dialoguer::Input;
use crate::config;
use std::fs::File;

pub fn run() {
    let name = if std::env::args().len() < 3 {
        Input::<String>::new().with_prompt("Name").interact().unwrap()
    } else {
        std::env::args().nth(2).unwrap()
    };

    match File::create(format!("{}/{}.{}", config::LOCATION, name, config::EXTENSION)) {
        Ok(_) => println!("Note {} created.", name),
        Err(_) => println!("Failed to create note."),
    };
}

use crate::config;
use crate::util::*;
use std::fs::File;

pub fn run() {
    let name = if args_len() < 3 {
        get_input()
    } else {
        get_argument(2)
    };

    match File::create(format!("{}{}{}", config::DIRECTORY, name, config::EXTENSION)) {
        Ok(_) => println!("Note {} created.", name),
        Err(_) => println!("Failed to create note."),
    };
}

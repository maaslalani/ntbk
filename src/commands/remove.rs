use crate::config;
use crate::util::{args_len,choose_note,get_argument};
use std::fs;

pub fn run() {
    let name = if args_len() < 3 {
        choose_note()
    } else {
        get_argument(2)
    };

    match fs::remove_file(format!("{}{}{}", config::DIRECTORY, name, config::EXTENSION)) {
        Ok(_) => println!("Note {} removed.", name),
        Err(_) => println!("Failed to remove note."),
    };
}

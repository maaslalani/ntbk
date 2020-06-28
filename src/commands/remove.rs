use crate::config;
use crate::util::*;
use std::fs;

pub fn run() {
    let name = if args_len() < 3 {
        choose_note()
    } else {
        get_argument(2)
    };

    if !get_confirmation("Confirm") {
        println!("Notes {} was not removed.", name);
        return
    }

    match fs::remove_file(format!("{}{}{}", &config::directory(), name, config::EXTENSION)) {
        Ok(_) => println!("Note {} removed.", name),
        Err(_) => println!("Failed to remove note."),
    };
}

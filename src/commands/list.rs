use crate::config;
use crate::util;
use std::fs;

pub fn run() {
    let files = fs::read_dir(config::DIRECTORY).unwrap();
    let notes = files.map(util::extract_path).collect::<Vec<String>>();
    println!("{}", notes.join("\n"));
}

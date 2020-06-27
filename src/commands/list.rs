use crate::config;
use std::fs;

pub fn run() {
    let files = fs::read_dir(config::LOCATION).unwrap();

    for file in files {
        println!("{}", file.unwrap().path().display())
    }
}

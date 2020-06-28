use crate::config;
use dialoguer::Select;
use dialoguer::Input;
use std::{fs,result,io,env};

pub fn choose_note() -> String {
    let options = list_notes();
    let selection = Select::new()
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    options[selection].to_string()
}

pub fn get_input() -> String {
    Input::<String>::new().interact().unwrap()
}

pub fn get_argument(n: usize) -> String {
    env::args().nth(n).unwrap()
}

pub fn list_notes() -> Vec<String> {
    let files = fs::read_dir(config::DIRECTORY).unwrap();
    files.map(extract_path).collect::<Vec<String>>()
}

pub fn args_len() -> usize {
    env::args().len() 
}

pub fn extract_path(file: result::Result<fs::DirEntry, io::Error>) -> String {
    let path = file.unwrap().path().display().to_string();
    extract_name(path)
}

fn extract_name(path: String) -> String {
    path.replace(config::DIRECTORY, "")
        .replace(config::EXTENSION, "")
}

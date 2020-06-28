use crate::config;
use dialoguer::Select;
use std::fs;
use std::result;
use std::io;

pub fn choose_note() -> String {
    let files = fs::read_dir(config::DIRECTORY).unwrap();
    let options = files.map(extract_path).collect::<Vec<String>>();

    let selection = Select::new()
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    options[selection].to_string()
}

pub fn extract_path(file: result::Result<fs::DirEntry, io::Error>) -> String {
    let path = file.unwrap().path().display().to_string();
    extract_name(path)
}

fn extract_name(path: String) -> String {
    path.replace(config::DIRECTORY, "")
        .replace(config::EXTENSION, "")
        .replace("/", "")
        .replace(".", "")
}

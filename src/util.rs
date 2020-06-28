use crate::config;
use dialoguer::{Select,Input,Confirm,theme};
use std::{fs,result,io,env};

pub fn choose_note() -> String {
    let options = list_notes();
    let theme = theme::ColorfulTheme::default();
    let selection = match Select::with_theme(&theme)
        .default(0)
        .items(&options[..])
        .interact() {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Error: Failed to open prompt");
                panic!(err)
            }
        };

    options[selection].to_string()
}

pub fn get_confirmation(prompt: &str) -> bool {
    let theme = theme::ColorfulTheme::default();
    match Confirm::with_theme(&theme)
        .with_prompt(prompt)
        .interact() {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Error: Failed to open prompt");
            panic!(err)
        }
    }
}

pub fn get_input() -> String {
    let theme = theme::ColorfulTheme::default();
    match Input::<String>::with_theme(&theme).interact() {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Error: Failed to open prompt");
            panic!(err)
        }
    }
}

pub fn get_argument(n: usize) -> String {
    match env::args().nth(n) {
        Some(val) => val,
        None => {
            eprintln!("Error: Failed to get argument in position {}", n);
            eprintln!("Arguments provided: {}", args_len());
            panic!()
        }
    }
}

pub fn list_notes() -> Vec<String> {
    let files = match fs::read_dir(&config::directory()) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Error: Failed to open {}", &config::directory());
            panic!(err)
        }
    };
    files.map(extract_path).collect::<Vec<String>>()
}

pub fn args_len() -> usize {
    env::args().len() 
}

pub fn extract_path(file: result::Result<fs::DirEntry, io::Error>) -> String {
    let path = match file {
        Ok(val) => val.path().display().to_string(),
        Err(err) => {
            eprintln!("Error: Failed to get file information");
            panic!(err)
        },
    };
    extract_name(path)
}

fn extract_name(path: String) -> String {
    path.replace(&config::directory(), "")
        .replace(config::EXTENSION, "")
}

use dirs;

pub fn directory() -> String {
    match std::env::var("NOTES_DIRECTORY") {
        Ok(val) => {
            let suffix = if val.ends_with("/") { "" } else { "/" };
            val + suffix
        },
        Err(_) => {
            let home_dir = match dirs::home_dir() {
                Some(path) => path.display().to_string(),
                None => panic!("Error: Please specify $NOTES_DIRECTORY"),
            };
            format!("{}/{}/", home_dir, "notes")
        }
    }
}

pub const EXTENSION: &str = ".md";
pub const EDITOR: &str = "nvim";
pub const GREP_COMMAND: &str = "rg";
pub const SHOW_COMMAND: &str = "bat";
pub const FIND_COMMAND: &str = "fd";

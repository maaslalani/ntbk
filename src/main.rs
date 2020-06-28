mod commands;
mod config;
mod util;

use commands::*;

fn main() {
    let action = util::get_argument(1);

    match action.as_str() {
        "f" | "find" => find::run(),
        "g" | "grep" => grep::run(),
        "l" | "list" => list::run(),
        "n" | "new" => new::run(),
        "o" | "open" => open::run(),
        "r" | "remove" => remove::run(),
        "s" | "show" => show::run(),
        _ => help::run(),
    }
}

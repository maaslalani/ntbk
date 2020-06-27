mod commands;

fn main() {
    let action = std::env::args().nth(1).expect("No action provided");

    match action.as_str() {
        "find" => commands::find::run(),
        "grep" => commands::grep::run(),
        "list" => commands::list::run(),
        "new" => commands::new::run(),
        "open" => commands::open::run(),
        "remove" => commands::remove::run(),
        "show" => commands::show::run(),
        _ => commands::help::run(),
    }
}

mod commands;

fn main() {
    let action = std::env::args().nth(1).expect("No action provided");
    let name = std::env::args().nth(2).expect("No name provided");

    match action.as_str() {
        "new" => commands::new::run(name),
        "list" => commands::list::run(),
        "find" => commands::find::run(),
        "grep" => commands::grep::run(),
        "open" => commands::open::run(),
        "show" => commands::show::run(),
        "remove" => commands::remove::run(),
        "help" => commands::help::run(),
        _ => println!("Unexpected action"),
    }
}

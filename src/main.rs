mod commands;

fn main() {
    let action = std::env::args().nth(1).expect("No action provided");
    let name = std::env::args().nth(2).expect("No name provided");

    match action.as_str() {
        "new" => commands::new(),
        "list" => commands::list(),
        "find" => commands::find(),
        "grep" => commands::grep(),
        "open" => commands::open(),
        "show" => commands::show(),
        "remove" => commands::remove(),
        "help" => commands::help(),
        _ => println!("Unexpected action"),
    }
}

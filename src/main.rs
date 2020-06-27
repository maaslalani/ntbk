mod commands;

fn main() {
    let action = std::env::args().nth(1).expect("No action provided");
    let name = std::env::args().nth(2).expect("No name provided");

    match action.as_str() {
        "new" => commands::new(),
        "list" => println!("TODO"),
        "find" => println!("TODO"),
        "grep" => println!("TODO"),
        "open" => println!("TODO"),
        "show" => println!("TODO"),
        "remove" => println!("TODO"),
        "help" => println!("TODO"),
        _ => println!("Unexpected action"),
    }
}

mod commands;

fn main() {
    let action = std::env::args().nth(1).expect("No action provided");
    let operand = std::env::args().nth(2);

    let pattern_error = "No pattern provided";
    let name_error = "No pattern provided";

    match action.as_str() {
        "find" => commands::find::run(operand.expect(pattern_error)),
        "grep" => commands::grep::run(operand.expect(pattern_error)),
        "list" => commands::list::run(),
        "new" => commands::new::run(operand.expect(name_error)),
        "open" => commands::open::run(operand.expect(name_error)),
        "remove" => commands::remove::run(operand.expect(name_error)),
        "show" => commands::show::run(operand.expect(name_error)),
        _ => commands::help::run(),
    }
}

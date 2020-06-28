pub fn run() {
    println!("ntbk is a terminal notebook manager.\n");

    println!("Usage: ntbk [ACTION]");
    println!("  ntbk find   <pattern>  - Find notes by path");
    println!("  ntbk grep   <pattern>  - Search through notes content");
    println!("  ntbk help              - Print this usage information");
    println!("  ntbk list              - List all notes");
    println!("  ntbk new    <name>     - Create a new note with <name>");
    println!("  ntbk open   <name>     - Open a note");
    println!("  ntbk remove <name>     - Remove a note");
    println!("  ntbk show   <name>     - Show a note's content");

    println!("\nAll <name> arguments are optional");
    println!("if none is provided then you will be presented with a list or an input box");
}

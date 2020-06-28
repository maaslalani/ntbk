const HELP: &str = r#"
ntbk is a terminal notebook manager.

Usage: ntbk [ACTION]

    ntbk find   <pattern>  - Find notes by path
    ntbk grep   <pattern>  - Search through notes content
    ntbk help              - Print this usage information
    ntbk list              - List all notes
    ntbk new    <name>     - Create a new note with <name>
    ntbk open   <name>     - Open a note
    ntbk remove <name>     - Remove a note
    ntbk show   <name>     - Show a note's content

Arguments <name> and <pattern> are optional.
if none are provided, ntbk will prompt for a choice / input.
"#;

pub fn run() {
    println!("{}", HELP);
}

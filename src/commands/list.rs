use crate::util::list_notes;

pub fn run() {
    println!("{}", list_notes().join("\n"));
}

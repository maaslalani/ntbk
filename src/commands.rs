pub mod find;
pub mod grep;
pub mod help;
pub mod list;
pub mod new;
pub mod open;
pub mod remove;
pub mod show;

pub fn new() {
    new::command()
}

pub fn list() {
    list::command()
}

pub fn find() {
    find::command()
}

pub fn grep() {
    grep::command()
}

pub fn open() {
    open::command()
}

pub fn show() {
    show::command()
}

pub fn remove() {
    remove::command()
}

pub fn help() {
    help::command()
}

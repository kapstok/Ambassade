#[macro_use]
extern crate serde_json;

mod filesystem;
mod arguments;

fn main() {
    match filesystem::get_project_root() {
        Some(_) => println!("You are in a project."),
        None => println!("You are not in a project.")
    }

    arguments::parse();
}

#[macro_use]
extern crate serde_json;

mod backend;
mod shell;
mod parser;

fn main() {
    match backend::filesystem::get_current_project_root() {
        Some(_) => println!("You are in a project."),
        None => println!("You are not in a project.")
    }

    parser::parse_argv();
}

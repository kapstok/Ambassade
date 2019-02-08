#[macro_use]
extern crate serde_json;

mod backend;
mod shell;
mod parser;

fn main() {
    backend::system::watch();

    match backend::filesystem::get_current_project_root() {
        Some(_) => backend::log("You are in a project."),
        None => backend::log("You are not in a project.")
    }

    parser::parse_argv();
    backend::output::clear();
    // No need to flush stdout, since this is the end of the main loop.
}

#[macro_use]
extern crate serde_json;

mod backend;
use std::env;

fn main() {
    match backend::filesystem::get_project_root() {
        Some(_) => println!("You are in a project."),
        None => println!("You are not in a project.")
    }

    parse();
}

fn parse() {
    let mut argv = env::args();
    let _ = argv.next();
    let arg = argv.next();

    match arg {
        Some(argument) => {
            if &argument == "--help" || &argument == "-h" {
                backend::project::help();
            }
            else if &argument == "init" {
                backend::project::init(&mut argv);
            }
            else if &argument == "build" {
                match backend::project::build() {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("Build failed: {}", e)
                }
            }
        },
        None => backend::project::help()
    }
}
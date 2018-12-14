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
            else if &argument == "run" {
                match backend::project::run(&mut argv) {
                    Ok(_) => {},
                    Err(e) => println!("Running project failed: {}", e)
                }
            }
            else if &argument == "exe" {
                match backend::project::exe(&mut argv) {
                    Ok(_) => {},
                    Err(e) => println!("Running project failed: {}", e)
                }
            }
            else if &argument == "add" {
                match backend::add::add(&mut argv) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => println!("Could not add dependency: {}", e)
                }
            }
            else if &argument == "dep-tree" {
                match backend::project::dep_tree(&mut argv) {
                    Ok(tree) => println!("{}", tree),
                    Err(e) => println!("Could not deduce dependency tree: {}", e)
                }
            }
        },
        None => backend::project::help()
    }
}

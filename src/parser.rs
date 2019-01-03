use backend;
use shell;
use std::env;

pub fn parse_argv() {
    let mut argv = env::args();
    let _ = argv.next();

    if !parse(&mut argv, true) {
        backend::project::help();
    }
}

pub fn parse_command<'a, I>(args: &mut I) -> bool where I: Iterator<Item=&'a str> {
    parse(&mut args.map(|arg| String::from(arg)), false)
}

fn parse<I>(args: &mut I, open_shell: bool) -> bool where I: Iterator<Item=String> {
    match args.next() {
        Some(argument) => {
            if argument == "--help" || argument == "-h" {
                backend::project::help();
            }
            else if argument == "init" {
                backend::project::init(args);
            }
            else if argument == "build" {
                match backend::project::build(args) {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("Build failed: {}", e)
                }
            }
            else if argument == "run" {
                match backend::project::run(args) {
                    Ok(_) => {},
                    Err(e) => println!("Running project failed: {}", e)
                }
            }
            else if argument == "exe" {
                match backend::project::exe(args) {
                    Ok(_) => {},
                    Err(e) => println!("Running project failed: {}", e)
                }
            }
            else if argument == "add" {
                match backend::project::add(&args.collect()) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => println!("Could not add dependency: {}", e)
                }
            }
            else if argument == "hide" {
                let args: Vec<String> = args.collect();
                match backend::project::add(&args) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => println!("Could not add dependency: {}", e)
                }

                match backend::project::ignore(&args) {
                    Ok(_) => println!("DONE! Dependency hidden."),
                    Err(e) => println!("Could not hide dependency: {}", e)
                }
            }
            else if argument == "delete" {
                match backend::project::delete(args) {
                    Ok(module) => println!("Module '{}' deleted.", module),
                    Err(e) => println!("Deleting module failed: {}", e)
                }
            }
            else if argument == "dep-tree" {
                match backend::project::dep_tree(args) {
                    Ok(tree) => println!("{}", tree),
                    Err(e) => println!("Could not deduce dependency tree: {}", e)
                }
            }
            else if argument == "git" {
                backend::git::to_shell(args);
            }
            else {
                return false;
            }
        },
        None if open_shell => shell::shell(),
        None => return false
    }
    true
}

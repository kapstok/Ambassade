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
            if argument == "--help" || argument == "-h" || argument == "help" {
                backend::project::help();
            }
            else if argument == "version" {
                const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

                match VERSION {
                    Some(version) => backend::normal(format!("Ambassade version v{}", version)),
                    None => backend::normal("Could not load version.")
                }
            }
            else if argument == "init" {
                backend::project::init(args);
            }
            else if argument == "build" {
                match backend::project::build(args) {
                    Ok(_) => {},
                    Err(e) => backend::log(format!("Build failed: {}", e))
                }
            }
            else if argument == "run" {
                match backend::project::run(&mut args.collect()) {
                    Ok(_) => {},
                    Err(e) => backend::log(format!("Running project failed: {}", e))
                }
            }
            else if argument == "exe" {
                match backend::project::exe(args) {
                    Ok(_) => {},
                    Err(e) => backend::log(format!("Running project failed: {}", e))
                }
            }
            else if argument == "add" {
                match backend::project::add(&args.collect()) {
                    Ok(msg) => backend::log(msg),
                    Err(e) => backend::log(format!("Could not add dependency: {}", e))
                }
            }
            else if argument == "hide" {
                let args: Vec<String> = args.collect();
                match backend::project::hide(&args) {
                    Ok(msg) => backend::log(msg),
                    Err(e) => backend::log(format!("Could not hide dependency: {}", e))
                }
            }
            else if argument == "delete" {
                match backend::project::delete(args) {
                    Ok(module) => backend::log(format!("Module '{}' deleted.", module)),
                    Err(e) => backend::log(format!("Deleting module failed: {}", e))
                }
            }
            else if argument == "dep-tree" {
                match backend::project::dep_tree(args) {
                    Ok(tree) => backend::normal(tree),
                    Err(e) => backend::log(format!("Could not deduce dependency tree: {}", e))
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

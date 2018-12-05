mod config;

use std::env;

pub fn parse() {
    let mut argv = env::args();
    let _ = argv.next();
    let arg = argv.next();

    match arg {
        Some(argument) => {
            if &argument == "--help" || &argument == "-h" {
                show_help();
            }
            else if &argument == "init" {
                init(&mut argv);
            }
        },
        None => show_help()
    }
}

fn show_help() {
    println!("Syntax:");
    println!("$ beheer [FLAG] [COMMAND [ARGUMENTS]]");
    println!("");

    println!("--help -h\t\tShow this message");
    println!("");

    println!("init [DIRECTORY]\tInitialize new project in specified directory. Defaults to current directory.");
}

fn init(args: &mut env::Args) {
    let mut directory = env::current_dir().unwrap();

    if let Some(projectname) = args.next() {
        directory.push(&projectname);
    }

    match config::create(directory) {
        Ok(_) => println!("Initialized project!"),
        Err(e) => println!("Initializing project failed: {}", e)
    }
}

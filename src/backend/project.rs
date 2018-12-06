use std::env;
use std::io::{Result, Error, ErrorKind};

pub fn init(args: &mut env::Args) {
    let mut directory = env::current_dir().unwrap();

    if let Some(projectname) = args.next() {
        directory.push(&projectname);
    }

    match super::config::create(directory) {
        Ok(_) => println!("Initialized project!"),
        Err(e) => println!("Initializing project failed: {}", e)
    }
}

pub fn build() -> Result<String> {
    match super::filesystem::get_project_root() {
        Some(dir) => {
            match super::config::read(dir) {
                Ok(configfile) => return super::build::build(configfile),
                Err(e) => Err(e)
            }
        },
        None => Err(Error::new(ErrorKind::NotFound, "not in a project (sub)directory."))
    }
}

pub fn help() {
    println!("Syntax:");
    println!("$ beheer [FLAG] [COMMAND [ARGUMENTS]]");
    println!("");

    println!("--help -h\t\tShow this message");
    println!("");

    println!("init [DIRECTORY]\tInitialize new project in specified directory. Defaults to current directory.");
    println!("build\t\t\tBuild current project.");
    println!("run [ARGUMENTS]\t\tBuild and run current project with ARGUMENTS to run project with.");
    println!("exe\t\t\tRun current project without building.", );
}

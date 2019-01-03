use std::env;
use std::result::Result;
use super::deptree;

pub fn init<I>(args: &mut I) where I: Iterator<Item=String> {
    let mut directory = env::current_dir().unwrap();

    if let Some(projectname) = args.next() {
        directory.push(&projectname);
    }

    match super::config::create(directory) {
        Ok(_) => println!("Initialized project!"),
        Err(e) => println!("Initializing project failed: {}", e)
    }
}

pub fn build<I>(args: &mut I) -> Result<String, String> where I: Iterator<Item=String> {
    match args.next() {
        Some(ref module) if module.as_str() == "--module" => {
            match super::filesystem::get_current_module_root() {
                Some(dir) => super::build::build(dir),
                None => Err(String::from("not in a project (sub)directory."))
            }
        },
        Some(_) | None => {
            match super::filesystem::get_current_project_root() {
                Some(dir) => super::build::build_rec(dir),
                None => Err(String::from("not in a project (sub)directory."))
            }
        }
    }
}

pub fn exe<I>(args: &mut I) -> Result<String, String> where I: Iterator<Item=String> {
    let output_dir;
    let mut args = String::new();

    match super::filesystem::get_current_project_root() {
        Some(dir) => output_dir = dir,
        None => return Err(String::from("not in a project (sub)directory."))
    }

    match super::config::get_json(output_dir) {
        Ok(config) => {
            if cfg!(target_os = "linux") {
                match config["run"]["linux"].as_str() {
                    Some(string) => args = String::from(string),
                    None => return Err(String::from("beheer.json: 'run->linux' should be a string."))
                }
            }
            if cfg!(target_os = "macos") {
                match config["run"]["os-x"].as_str() {
                    Some(string) => args = String::from(string),
                    None => return Err(String::from("beheer.json: 'run->os-x' should be a string."))
                }
            }
            if cfg!(target_os = "windows") {
                match config["run"]["windows"].as_str() {
                    Some(string) => args = String::from(string),
                    None => return Err(String::from("beheer.json: 'run->windows' should be a string."))
                }
            }
        },
        Err(e) => return Err(e)
    }

    println!("Running project..");
    super::fetch::run(env::current_dir().unwrap(), args)
}

pub fn run<I>(args: &mut I) -> Result<String, String> where I: Iterator<Item=String> {
    println!("Building project..");

    match build(args) {
        Ok(output) => println!("{}", output),
        Err(e) => return Err(e)
    }

    exe(args)
}

pub fn delete<I>(path: &mut I) -> Result<String, String> where I: Iterator<Item=String> {
    let path = match path.next() {
        Some(arg) => arg,
        None => return Err(String::from("Missing path as argument."))
    };

    match env::current_dir() {
        Ok(mut dir) => {
            dir.push(path);
            super::delete::delete(dir)
        },
        Err(e) => Err(e.to_string())
    }
}

pub fn add(args: &Vec<String>) -> Result<String, String> {
    let mut path: String = match args.get(0) {
        Some(arg) => arg.clone(),
        None => return Err(String::from("Missing path as argument."))
    };

    match super::filesystem::get_current_module_root() {
        Some(dir) => {
            match super::git::ignore::add(&mut dir.clone(), &mut path) {
                Ok(_) => {},
                Err(e) => return Err(e)
            }

            match super::add::add(args) {
                Ok(msg) => println!("{}", msg),
                Err(e) => return Err(e)
            }

            match super::config::get_json(dir) {
                Ok(json) => {
                    super::dep::check(json)
                },
                Err(e) => return Err(e)
            }
        },
        None => Err(String::from("Not in a project (sub)directory."))
    }
}

pub fn ignore(args: &Vec<String>) -> Result<(), String> {
    let mut entry: String = args.get(0).unwrap().clone();

    match super::filesystem::get_current_dep_root() {
        Ok(mut dir) => {
            dir.push(&entry);
            super::git::ignore::add(&mut dir, &mut entry) // entry moet "beheer.json" worden.
        },
        Err(e) => Err(e.to_string())
    }
}

pub fn dep_tree<I>(args: &mut I) -> Result<deptree::Node, String> where I: Iterator<Item=String> {
    let path = match super::filesystem::get_current_module_root() {
        Some(p) => p,
        None => return Err(String::from("Not in a project/dependency directory."))
    };

    match args.next() {
        Some(ref os) if os.as_str() == "linux" => deptree::print(&super::system::OS::Linux, path),
        Some(ref os) if os.as_str() == "os-x" => deptree::print(&super::system::OS::MacOs, path),
        Some(ref os) if os.as_str() == "windows" => deptree::print(&super::system::OS::Windows, path),
        Some(ref os) if os.as_str() == "all" => deptree::print(&super::system::OS::All, path),
        Some(_) => Err(String::from("dep-tree: OS not found. Possible inputs: 'all', 'linux', 'os-x', 'windows'")),
        None => deptree::print(&super::system::OS::current(), path)
    }
}

pub fn help() {
    println!("Syntax:");
    println!("$ beheer [FLAG] [COMMAND [ARGUMENTS]]");
    println!("");

    println!("--help -h\t\t\t\tShow this message");
    println!("");

    println!("init [DIRECTORY]\t\t  Initialize new project in specified directory. Defaults to current directory.");
    println!("build [--module]\t\t  Build current project if module flag is not specified, otherwise only the module will be built.");
    println!("run [ARGUMENTS]\t\t\t  Build and run current project with ARGUMENTS to run project with.");
    println!("exe [ARGUMENTS]\t\t\t  Run current project with ARGUMENTS. The project won't be built.");
    println!("add NAME COMMAND [ARGUMENTS]\t  Add dependency with NAME to module and is built through COMMAND with ARGUMENTS.");
    println!("hide NAME COMMAND [ARGUMENTS]\t  Add dependency with NAME to module and is built through COMMAND with ARGUMENTS. Add configfile to '.gitignore'.");
    println!("delete PATH\t\t\t  Delete a dependency in PATH.");
    println!("dep-tree [all|linux|os-x|windows] Print a tree of all dependencies used (indirectly) by a project for specified OS. Defaults to 'all'.");
}

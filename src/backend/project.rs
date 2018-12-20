use std::env;
use std::result::Result;
use std::process::{Command, Stdio};
use super::deptree;

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

pub fn build(args: &mut env::Args) -> Result<String, String> {
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

pub fn exe(args: &mut env::Args) -> Result<String, String> {
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

    let mut arguments: Vec<&str> = args.split(' ').collect();
    let command = arguments.remove(0);
    let out = Command::new(command)
        .args(arguments)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("");

    match out.status.success() {
        true => Ok(String::from_utf8_lossy(&out.stdout).to_string()),
        false => Err(String::from_utf8_lossy(&out.stderr).to_string())
    }
}

pub fn run(args: &mut env::Args) -> Result<String, String> {
    println!("Building project..");

    match build(args) {
        Ok(output) => println!("{}", output),
        Err(e) => return Err(e)
    }

    exe(args)
}

pub fn delete(path: &mut env::Args) -> Result<String, String> {
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

pub fn dep_tree(args: &mut env::Args) -> Result<deptree::Node, String> {
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

    println!("--help -h\t\tShow this message");
    println!("");

    println!("init [DIRECTORY]\t\t  Initialize new project in specified directory. Defaults to current directory.");
    println!("build [--module]\t\t  Build current project if module flag is not specified, otherwise only the module will be built.");
    println!("run [ARGUMENTS]\t\t\t  Build and run current project with ARGUMENTS to run project with.");
    println!("exe [ARGUMENTS]\t\t\t  Run current project with ARGUMENTS. The project won't be built.");
    println!("add NAME COMMAND [ARGUMENTS]\t  Add dependency with NAME to module and is built through COMMAND with ARGUMENTS.");
    println!("delete PATH\t\t\t Delete a dependency in PATH.");
    println!("dep-tree [all|linux|os-x|windows] Print a tree of all dependencies used (indirectly) by a project for specified OS. Defaults to 'all'.");
}

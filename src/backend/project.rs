use std::thread;
use std::env;
use std::result::Result;
use super::deptree;

pub fn init<I>(args: &mut I) where I: Iterator<Item=String> {
    let mut directory = env::current_dir().unwrap();

    if let Some(projectname) = args.next() {
        directory.push(&projectname);
    }

    match super::git::ignore::init(&mut directory.clone()) {
        Ok(_) => {},
        Err(e) => println!("Add 'dep' folder to .gitignore failed: {}. Continuing with project initialization..", e)
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
                Some(dir) => {
                    let dep_name = String::from(dir.file_name().unwrap().to_str().unwrap());
                    super::build::build(dep_name)
                },
                None => Err(String::from("not in a project (sub)directory."))
            }
        },
        _ => {
            match super::filesystem::get_current_project_root() {
                Some(dir) => super::build::build_rec(dir),
                None => Err(String::from("not in a project (sub)directory."))
            }
        }
    }
}

pub fn exe<I>(args: &mut I) -> Result<String, String> where I: Iterator<Item=String> {
    match super::filesystem::get_current_project_root() {
        Some(dir) => super::run::run_sync(String::from(dir.file_name().unwrap().to_str().unwrap())),
        None => Err(String::from("not in a project (sub)directory."))
    }
}

pub fn run(args: &mut Vec<String>) -> Result<String, String> {
    println!("Building project..");

    let mut threads = super::internal::paralellism::Threadhandler::new();

    match build(&mut args.clone().into_iter()) {
        Ok(output) => println!("{}", output),
        Err(e) => return Err(e)
    }

    if args.len() > 1 {
        let mut modules = args.len() - 1;
        while modules > 0 {
            println!("running async module {}: {}", modules, args[modules]);
            match super::run::run_async(args[modules].clone(), &mut threads) {
                Ok(_) => {},
                Err(e) => return Err(e)
            }
            modules -= 1;
        }
    }

    if args.len() > 0 {
        let handler = thread::spawn(move || threads.start());

        println!("running sync module 0: {}", args[0]);
        let result = match super::run::run_sync(args[0].clone()) {
            Ok(_) => Ok(String::from("Run succeeded! Quitting..")),
            Err(e) => Err(e)
        };

        handler.join().unwrap();

        return result;
    }

    let config = match super::filesystem::get_current_project_root() {
        Some(path) => path,
        None => return Err(String::from("Not in a project (sub)directory."))
    };

    match super::run::run_sync(String::from(config.file_name().unwrap().to_str().unwrap())) {
        Ok(_) => Ok(String::from("Run succeeded! Quitting..")),
        Err(e) => Err(e)
    }
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
    let _dep: String = match args.get(0) {
        Some(arg) => arg.clone(),
        None => return Err(String::from("Missing dependency name as argument."))
    };

    match super::filesystem::get_current_module_root() {
        Some(dir) => {
            match super::add::add(args) {
                Ok(msg) => println!("{}", msg),
                Err(e) => return Err(e)
            }

            match super::config::get_json_from_dir(dir) {
                Ok(json) => {
                    super::dep::check(json)
                },
                Err(e) => return Err(e)
            }
        },
        None => Err(String::from("Not in a project (sub)directory."))
    }
}

pub fn hide(args: &Vec<String>) -> Result<String, String> {
    match add(args) {
        Ok(msg) => println!("{}", msg),
        Err(e) => return Err(e)
    }

    let dep: String = match args.get(0) {
        Some(arg) => arg.clone(),
        None => return Err(String::from("Missing dep name."))
    };

    match super::dep_config::init(dep.clone()) {
        Ok(_) => println!("Created '{}.json' in 'dep_config' folder.", dep),
        Err(e) => return Err(e)
    }

    Ok(String::from("dependency successfully hidden!"))
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
    println!("$ ambassade [FLAG] [COMMAND [ARGUMENTS]]");
    println!("");

    println!("--help -h\t\t\t\tShow this message");
    println!("");

    println!("init [DIRECTORY]\t\t  Initialize new project in specified directory. Defaults to current directory.");
    println!("build [--module]\t\t  Build current project if module flag is not specified, otherwise only the module will be built.");
    println!("run [MODULES]\t\t\t  Build current project and run MODULES. MODULES default to the project module.");
    println!("exe [ARGUMENTS]\t\t\t  Run current project with ARGUMENTS. The project won't be built.");
    println!("add NAME COMMAND [ARGUMENTS]\t  Add dependency with NAME to module and is built through COMMAND with ARGUMENTS.");
    println!("hide NAME COMMAND [ARGUMENTS]\t  Add dependency with NAME to module and is built through COMMAND with ARGUMENTS. Add configfile to '.gitignore'.");
    println!("delete PATH\t\t\t  Delete a dependency in PATH.");
    println!("dep-tree [all|linux|os-x|windows] Print a tree of all dependencies used (indirectly) by a project for specified OS. Defaults to 'all'.");
}

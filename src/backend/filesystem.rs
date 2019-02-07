use std::{path, env, fs};
use std::io::{Result, Error, ErrorKind};

fn get_root(mut path: path::PathBuf) -> Option<path::PathBuf> {
    loop {
        let mut config = path.clone();
        config.push("ambassade.json");

        if config.as_path().is_file() {
            return Some(path);
        }

        if !path.pop() {
            return None;
        }
    }
}

pub fn get_current_module_root() -> Option<path::PathBuf> {
    get_module_root(env::current_dir().unwrap())
}

pub fn get_module_root(from_dir: path::PathBuf) -> Option<path::PathBuf> {
    get_root(from_dir)
}

pub fn get_current_project_root() -> Option<path::PathBuf> {
    get_project_root(env::current_dir().unwrap())
}

pub fn get_project_root(from_dir: path::PathBuf) -> Option<path::PathBuf> {
    let mut path = get_root(from_dir);
    let mut parentdir = path.clone();

    loop {
        match parentdir {
            Some(mut p) => {
                path = Some(path::PathBuf::from(p.clone()));
                if !p.pop() {
                    return path;
                }
                parentdir = get_root(p);
            },
            None => return path
        }
    }
}

pub fn get_current_dep_root() -> Result<path::PathBuf> {
    get_dep_root(env::current_dir().unwrap())
}

pub fn get_dep_root(from_dir: path::PathBuf) -> Result<path::PathBuf> {
    match get_project_root(from_dir) {
        Some(mut path) => {
            path.push("dep");
            if !path.is_dir() {
                super::log("\tNo dep folder found. Creating folder..");
                match fs::create_dir(path.clone()) {
                    Ok(_) => super::log(format!("\tCreated dir {}.", path.clone().to_str().unwrap())),
                    Err(e) => return Err(e)
                }
            }
            Ok(path)
        },
        None => Err(Error::new(ErrorKind::NotFound, "No project file found. Aborted."))
    }
}

pub fn get_dep_config_root() -> Result<path::PathBuf> {
    match get_current_project_root() {
        Some(mut path) => {
            path.push("dep_config");
            if !path.exists() {
                super::log("\t No dep_config folder found. Creating folder..");
                match fs::create_dir(path.clone()) {
                    Ok(_) => super::log(format!("\tCreated dir {}.", path.clone().to_str().unwrap())),
                    Err(e) => return Err(e)
                }
            }
            Ok(path)
        },
        None => Err(Error::new(ErrorKind::NotFound, "No project file found. Aborted."))
    }
}

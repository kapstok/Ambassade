use std::{path, env, fs};
use std::io::{Result, Error, ErrorKind};

fn get_root (mut path: path::PathBuf) -> Option<path::PathBuf> {
    loop {
        let mut config = path.clone();
        config.push("beheer.json");

        if config.as_path().is_file() {
            return Some(path);
        }

        if !path.pop() {
            return None;
        }
    }
}

pub fn get_module_root() -> Option<path::PathBuf> {
    get_root(env::current_dir().unwrap())
}

pub fn get_project_root() -> Option<path::PathBuf> {
    let mut path = get_root(env::current_dir().unwrap());
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

pub fn get_dep_dir() -> Result<path::PathBuf> {
    match get_project_root() {
        Some(mut path) => {
            path.push("dep");
            if !path.is_dir() {
                println!("No dep folder found. Creating folder..");
                match fs::create_dir(path.clone()) {
                    Ok(_) => println!("Created dir {}.", path.clone().to_str().unwrap()),
                    Err(e) => return Err(e)
                }
            }
            Ok(path)
        },
        None => Err(Error::new(ErrorKind::NotFound, "No project file found. Aborted."))
    }
}

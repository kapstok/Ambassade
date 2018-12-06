use std::{path, env};

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

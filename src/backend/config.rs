extern crate serde_json;

use std::io;
use std::io::{Error, ErrorKind, Write, Read};
use std::fs::File;
use std::path::PathBuf;
use std::result::Result;

pub fn create(mut path: PathBuf) -> Result<(), Error> {
    path.push("ambassade.json");
    init(&path)
}

pub fn init(path: &PathBuf) -> Result<(), Error> {
    let content = json!({
        "build": {
            "windows": "",
            "os-x": "",
            "linux": ""
        },
        "run": {
            "windows": "",
            "os-x": "",
            "linux": ""
        }
    });

    match File::open(path.to_str().unwrap()) {
        Ok(_) => return Err(Error::new(ErrorKind::AlreadyExists, "Already found a 'ambassade.json' file.")),
        Err(_) => {
            match File::create(path) {
                Ok(mut file) => {
                    let content_str = serde_json::to_string_pretty(&content).unwrap();
                    file.write_all(content_str.as_bytes())?;
                },
                Err(e) => return Err(e)
            }
        }
    }
    Ok(())
}

pub fn update(path: &PathBuf, value: serde_json::Value) -> Result<(), String> {
    match File::create(path) {
        Ok(mut file) => {
            match file.write_all(serde_json::to_string_pretty(&value).unwrap().as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string())
            }
        },
        Err(e) => Err(e.to_string())
    }
}

fn read(path: &PathBuf) -> Result<String, Error> {
    let mut config = String::new();

    check(path);

    match File::open(path.to_str().unwrap()) {
        Ok(mut file) => {
            file.read_to_string(&mut config).unwrap();
        },
        Err(e) => return Err(e)
    }

    Ok(config)
}

pub fn get_json(path: &PathBuf) -> Result<serde_json::Value, String> {
    match read(path) {
        Ok(config) => super::dep::json(config),
        Err(e) => {
            super::log(format!("Error on config file: '{}'.", path.to_str().unwrap()));
            match e.kind() {
                ErrorKind::NotFound => Err(String::from("No config file found.")),
                ErrorKind::PermissionDenied => Err(String::from("Config file unreadable: permission denied.")),
                _ => Err(e.to_string())
            }
        }
    }
}

pub fn get_json_from_dir(mut path: PathBuf) -> Result<serde_json::Value, String> {
    path.push("ambassade.json");
    get_json(&path)
}

fn check(config: &PathBuf) {
    if !config.is_file() {
        let mut input = String::new();

        super::log(format!("'{}' not found. ", config.to_str().unwrap()));

        if config.is_dir() {
            super::normal("Create specific config file for module (using ambassade hide)? [y/N]?");

            match io::stdin().read_line(&mut input) {
                Ok(_) if input.as_str() == "y\n" => {
                    let dep_name = String::from(config.file_name().unwrap().to_str().unwrap());
                    match super::dep_config::init(dep_name) {
                        Ok(_) => {},
                        Err(e) => super::log(format!("Module initialization failed. Details: {}", e))
                    }
                },
                Ok(_) | Err(_) => {}
            }
        } else {
            panic!("Path does not exist!");
        }
    }
}

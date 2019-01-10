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

fn init(path: &PathBuf) -> Result<(), Error> {
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

pub fn update(mut path: PathBuf, value: serde_json::Value) -> Result<(), String> {
    path.push("ambassade.json");

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

fn read(path: &mut PathBuf) -> Result<String, Error> {
    let mut config = String::new();

    path.push("ambassade.json");
    check(&path);

    match File::open(path.to_str().unwrap()) {
        Ok(mut file) => {
            file.read_to_string(&mut config).unwrap();
        },
        Err(e) => return Err(e)
    }

    Ok(config)
}

pub fn get_json(mut path: PathBuf) -> Result<serde_json::Value, String> {
    match read(&mut path) {
        Ok(config) => super::dep::json(config),
        Err(e) => {
            println!("Error on config file: '{}'.", path.to_str().unwrap());
            match e.kind() {
                ErrorKind::NotFound => Err(String::from("No config file found.")),
                ErrorKind::PermissionDenied => Err(String::from("Config file unreadable: permission denied.")),
                _ => Err(e.to_string())
            }
        }
    }
}

fn check(config: &PathBuf) {
    if !config.is_file() {
        let mut input = String::new();

        println!("'{}' not found. ", config.to_str().unwrap());
        println!("Initialize module with new config file [y/N]?");

        match io::stdin().read_line(&mut input) {
            Ok(_) if input.as_str() == "y\n" => match init(config) {
                Ok(_) => {},
                Err(e) => println!("Module initialization failed. Details: {}", e)
            },
            Ok(_) | Err(_) => {}
        }
    }
}

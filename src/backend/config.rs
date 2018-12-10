extern crate serde_json;

use std::io::{Error, ErrorKind, Write, Read};
use std::fs::File;
use std::path::PathBuf;
use std::result::Result;

pub fn create(mut path: PathBuf) -> Result<(), Error> {
    let content = json!({
        "project-name": path.file_name().unwrap().to_str().unwrap(),
        "version": 0.1,
        "build": {
            "windows": "echo \"No build config set.\"",
            "os-x": "echo \"No build config set.\"",
            "linux": "echo \"No build config set.\""
        },
        "run": {
            "windows": "echo \"No run config set.\"",
            "os-x": "echo \"No run config set.\"",
            "linux": "echo \"No run config set.\""
        }
    });

    path.push("beheer.json");

    match File::open(path.to_str().unwrap()) {
        Ok(_) => return Err(Error::new(ErrorKind::AlreadyExists, "Already found a 'beheer.json' file.")),
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

fn read(mut path: PathBuf) -> Result<String, String> {
    let mut config = String::new();

    path.push("beheer.json");

    match File::open(path.to_str().unwrap()) {
        Ok(mut file) => {
            file.read_to_string(&mut config).unwrap();
        },
        Err(e) => return Err(e.to_string())
    }

    Ok(config)
}

pub fn get_json(path: PathBuf) -> Result<serde_json::Value, String> {
    match read(path) {
        Ok(config) => super::check::json(config),
        Err(e) => Err(e.to_string())
    }
}

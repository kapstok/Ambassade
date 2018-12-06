extern crate serde_json;

use std::io::{Result, Error, ErrorKind, Write};

pub fn dep(config: String) -> Result<String> {
    let config_json: serde_json::Value;

    println!("Checking dependencies..");

    match serde_json::from_str(&config) {
        Ok(json) => config_json = json,
        Err(e) => {
            let mut error = String::from("JSON ERROR! ");
            error.push_str(&e.line().to_string());
            error.push(':');
            error.push_str(&e.column().to_string());
            error.push(' ');

            match e.classify() {
                serde_json::error::Category::Io => {
                    error.push_str("Weird error....");
                    return Err(Error::new(ErrorKind::Other, error));
                },
                serde_json::error::Category::Syntax => {
                    error.push_str("Syntax error in 'beheer.json'");
                    return Err(Error::new(ErrorKind::InvalidInput, error));
                },
                serde_json::error::Category::Data => {
                    error.push_str("Semantic error in 'beheer.json'");
                    return Err(Error::new(ErrorKind::InvalidData, error));
                },
                serde_json::error::Category::Eof => {
                    error.push_str("Unexpected end-of-file in 'beheer.json'");
                    return Err(Error::new(ErrorKind::UnexpectedEof, error));
                }
            }
        }
    }

    pkg_check(config_json)
}

#[cfg(target_os="linux")]
fn pkg_check(config: serde_json::Value) -> Result<String> {
    match config["deps"]["linux"] {
        json!(null) => return Ok(String::from("No dependencies found!")),
        ref deps => {
            if !deps.is_object() {
                return Err(Error::new(ErrorKind::InvalidData, "beheer.json: 'deps->linux' should be an object."));
            }
            for dep in deps.as_object().unwrap().iter() {
                if !dep.1.is_string() {
                    return Err(Error::new(ErrorKind::InvalidData, "beheer.json: all deps should be strings!"));
                }
                println!("Checking for {}..\n\t{}", dep.0, dep.1);
                super::fetch::fetch(dep.0.to_string(), String::from(dep.1.as_str().unwrap()));
            }
        }
    }
    Ok(String::from("Dependencies OK!"))
}

#[cfg(target_os="macos")]
fn pkg_check(config: serde_json::Value) -> Result<String> {
    match config["deps"]["os-x"] {
        json!(null) => return Ok(String::from("No dependencies found!")),
        ref sys_deps => {
            if !sys_deps.is_object() {
                return Err(Error::new(ErrorKind::InvalidData, "beheer.json: 'deps->os-x' should be an object."));
            }
            for dep in sys_deps.as_object().unwrap().iter() {
                if !dep.1.is_string() {
                    return Err(Error::new(ErrorKind::InvalidData, "beheer.json: all deps should be strings!"));
                }
                println!("Checking for {} version {}..", dep.0, dep.1);
            }
        }
    }
    Ok(String::from("Dependencies OK!"))
}

#[cfg(target_os="windows")]
fn pkg_check(config: serde_json::Value) -> Result<String> {
    match config["deps"]["windows"] {
        json!(null) => return Ok(String::from("No dependencies found!")),
        ref sys_deps => {
            if !sys_deps.is_object() {
                return Err(Error::new(ErrorKind::InvalidData, "beheer.json: 'deps->windows' should be an object."));
            }
            for dep in sys_deps.as_object().unwrap().iter() {
                if !dep.1.is_string() {
                    return Err(Error::new(ErrorKind::InvalidData, "beheer.json: all deps should be strings!"));
                }
                println!("Checking for {} version {}..", dep.0, dep.1);
            }
        }
    }
    Ok(String::from("Dependencies OK!"))
}

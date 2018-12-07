extern crate serde_json;

use std::io::{Result, Error, ErrorKind};

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

    config_check(config_json)
}

#[cfg(target_os="linux")]
fn config_check(config: serde_json::Value) -> Result<String> {
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

                match dir_check(dep.0.to_string(), String::from(dep.1.as_str().unwrap())) {
                    Ok(_) => {},
                    Err(e) => return Err(e)
                }
            }
        }
    }
    Ok(String::from("Dependencies OK!"))
}

#[cfg(target_os="macos")]
fn config_check(config: serde_json::Value) -> Result<String> {
    match config["deps"]["os-x"] {
        json!(null) => return Ok(String::from("No dependencies found!")),
        ref sys_deps => {
            if !sys_deps.is_object() {
                return Err(Error::new(ErrorKind::InvalidData, "beheer.json: 'deps->os-x' should be an object."));
            }

            for dep in deps.as_object().unwrap().iter() {

                if !dep.1.is_string() {
                    return Err(Error::new(ErrorKind::InvalidData, "beheer.json: all deps should be strings!"));
                }

                println!("Checking for {}..\n\t{}", dep.0, dep.1);

                if dir_check(dep.0.to_string(), String::from(dep.1.as_str().unwrap())) == Err(e) {
                    return Err(e);
                }
            }
        }
    }
    Ok(String::from("Dependencies OK!"))
}

#[cfg(target_os="windows")]
fn config_check(config: serde_json::Value) -> Result<String> {
    match config["deps"]["windows"] {
        json!(null) => return Ok(String::from("No dependencies found!")),
        ref sys_deps => {
            if !sys_deps.is_object() {
                return Err(Error::new(ErrorKind::InvalidData, "beheer.json: 'deps->windows' should be an object."));
            }

            for dep in deps.as_object().unwrap().iter() {
                if !dep.1.is_string() {
                    return Err(Error::new(ErrorKind::InvalidData, "beheer.json: all deps should be strings!"));
                }

                println!("Checking for {}..\n\t{}", dep.0, dep.1);

                if dir_check(dep.0.to_string(), String::from(dep.1.as_str().unwrap())) == Err(e) {
                    return Err(e);
                }
            }
        }
    }
    Ok(String::from("Dependencies OK!"))
}

fn dir_check(dependency: String, command: String) -> Result<()> {
    let dir = super::filesystem::get_dep_dir();

    match dir {
        Ok(mut dep_dir) => {
            dep_dir.push(dependency);
            if !dep_dir.is_dir() {
                dep_dir.pop();
                super::fetch::fetch(dep_dir, command);
            }
            Ok(())
        },
        Err(e) => Err(e)
    }
}

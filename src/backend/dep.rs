extern crate serde_json;

use std::result::Result;
use super::system::OS;

pub fn json(config: String) -> Result<serde_json::Value, String> {
    let config_json: serde_json::Value;

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
                    return Err(error);
                },
                serde_json::error::Category::Syntax => {
                    error.push_str("Syntax error in 'beheer.json'");
                    return Err(error);
                },
                serde_json::error::Category::Data => {
                    error.push_str("Semantic error in 'beheer.json'");
                    return Err(error);
                },
                serde_json::error::Category::Eof => {
                    error.push_str("Unexpected end-of-file in 'beheer.json'");
                    return Err(error);
                }
            }
        }
    }

    Ok(config_json)
}

pub fn dep(config: serde_json::Value, os: &OS) -> Result<Vec<(String, String)>, String> {
    let mut output: Vec<(String, String)> = Vec::new();

    match os {
        OS::all => {
            match dep(config.clone(), &OS::linux) {
                Ok(mut result) => {
                    let mut r: Vec<(String,String)> = result;
                    output.append(&mut r)
                },
                Err(e) => return Err(e)
            }
            match dep(config.clone(), &OS::macos) {
                Ok(mut result) => {
                    let mut r: Vec<(String,String)> = result;
                    output.append(&mut r)
                },
                Err(e) => return Err(e)
            }
            match dep(config.clone(), &OS::windows) {
                Ok(mut result) => {
                    let mut r: Vec<(String,String)> = result;
                    output.append(&mut r)
                },
                Err(e) => return Err(e)
            }
        },
        OS::linux => match config["deps"]["linux"] {
            json!(null) => return Ok(output),
            ref deps => {
                match deps.as_object() {
                    Some(object) => {
                        for dep in object.iter() {
                            if !dep.1.is_string() {
                                return Err(String::from("beheer.json: all deps should be strings!"))
                            }
                            output.push((dep.0.to_string(), String::from(dep.1.as_str().unwrap())));
                        }
                    },
                    None => return Err(String::from("beheer.json: 'deps->linux' should be an object."))
                }
            }
        },
        OS::macos => match config["deps"]["os-x"] {
            json!(null) => return Ok(output),
            ref deps => {
                match deps.as_object() {
                    Some(object) => {
                        for dep in object.iter() {
                            if !dep.1.is_string() {
                                return Err(String::from("beheer.json: all deps should be strings!"))
                            }
                            output.push((dep.0.to_string(), String::from(dep.1.as_str().unwrap())));
                        }
                    },
                    None => return Err(String::from("beheer.json: 'deps->os-x' should be an object."))
                }
            }
        },
        OS::windows => match config["deps"]["windows"] {
            json!(null) => return Ok(output),
            ref deps => {
                match deps.as_object() {
                    Some(object) => {
                        for dep in object.iter() {
                            if !dep.1.is_string() {
                                return Err(String::from("beheer.json: all deps should be strings!"))
                            }
                            output.push((dep.0.to_string(), String::from(dep.1.as_str().unwrap())));
                        }
                    },
                    None => return Err(String::from("beheer.json: 'deps->windows' should be an object."))
                }
            }
        }
    }
    Ok(output)
}

#[cfg(target_os="linux")]
pub fn check(config: serde_json::Value) -> Result<String, String> {
    println!("Checking dependencies..");

    match dep(config, &OS::linux) {
        Ok(ref deps) if deps.is_empty() => return Ok(String::from("No dependencies found!")),
        Ok(ref deps) => {
            for dep in deps.iter() {
                let dep_clone = dep.clone();

                println!("Checking for {}..\n\t{}", dep_clone.0, dep_clone.1);

                match dir_check(dep_clone.0, dep_clone.1) {
                    Ok(_) => {},
                    Err(e) => return Err(e)
                }
            }
        },
        Err(e) => return Err(e)
    }
    Ok(String::from("Dependencies OK!"))
}

#[cfg(target_os="macos")]
pub fn check(config: serde_json::Value) -> Result<String, String> {
    println!("Checking dependencies..");

    match dep(config, &OS::macos) {
        Ok(ref deps) if deps.is_empty() => return Ok(String::from("No dependencies found!")),
        Ok(ref deps) => {
            for dep in deps.iter() {
                let dep_clone = dep.clone();

                println!("Checking for {}..\n\t{}", dep_clone.0, dep_clone.1);

                match dir_check(dep_clone.0, dep_clone.1) {
                    Ok(_) => {},
                    Err(e) => return Err(e)
                }
            }
        },
        Err(e) => return Err(e)
    }
    Ok(String::from("Dependencies OK!"))
}

#[cfg(target_os="windows")]
pub fn check(config: serde_json::Value) -> Result<String, String> {
    println!("Checking dependencies..");

    match dep(config, &OS::windows) {
        Ok(ref deps) if deps.is_empty() => return Ok(String::from("No dependencies found!")),
        Ok(ref deps) => {
            for dep in deps.iter() {
                let dep_clone = dep.clone();

                println!("Checking for {}..\n\t{}", dep_clone.0, dep_clone.1);

                match dir_check(dep_clone.0, dep_clone.1) {
                    Ok(_) => {},
                    Err(e) => return Err(e)
                }
            }
        },
        Err(e) => return Err(e)
    }
    Ok(String::from("Dependencies OK!"))
}

fn dir_check(dependency: String, command: String) -> Result<String, String> {
    let dir = super::filesystem::get_dep_root();

    match dir {
        Ok(mut dep_dir) => {
            dep_dir.push(dependency);
            if !dep_dir.exists() {
                dep_dir.pop();
                return super::fetch::fetch(dep_dir, command);
            }
            Ok(String::from("Dependency found."))
        },
        Err(e) => Err(e.to_string())
    }
}

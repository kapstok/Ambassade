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
                    error.push_str("Syntax error in 'ambassade.json'");
                    return Err(error);
                },
                serde_json::error::Category::Data => {
                    error.push_str("Semantic error in 'ambassade.json'");
                    return Err(error);
                },
                serde_json::error::Category::Eof => {
                    error.push_str("Unexpected end-of-file in 'ambassade.json'");
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
        OS::All => {
            match dep(config.clone(), &OS::Linux) {
                Ok(mut result) => {
                    let mut r: Vec<(String,String)> = result;
                    output.append(&mut r)
                },
                Err(e) => return Err(e)
            }
            match dep(config.clone(), &OS::MacOs) {
                Ok(mut result) => {
                    let mut r: Vec<(String,String)> = result;
                    output.append(&mut r)
                },
                Err(e) => return Err(e)
            }
            match dep(config.clone(), &OS::Windows) {
                Ok(mut result) => {
                    let mut r: Vec<(String,String)> = result;
                    output.append(&mut r)
                },
                Err(e) => return Err(e)
            }
        },
        OS::Linux => match config["deps"]["linux"] {
            json!(null) => return Ok(output),
            ref deps => {
                match deps.as_object() {
                    Some(object) => {
                        for dep in object.iter() {
                            if !dep.1.is_string() {
                                return Err(String::from("ambassade.json: all deps should be strings!"))
                            }
                            output.push((dep.0.to_string(), String::from(dep.1.as_str().unwrap())));
                        }
                    },
                    None => return Err(String::from("ambassade.json: 'deps->linux' should be an object."))
                }
            }
        },
        OS::MacOs => match config["deps"]["os-x"] {
            json!(null) => return Ok(output),
            ref deps => {
                match deps.as_object() {
                    Some(object) => {
                        for dep in object.iter() {
                            if !dep.1.is_string() {
                                return Err(String::from("ambassade.json: all deps should be strings!"))
                            }
                            output.push((dep.0.to_string(), String::from(dep.1.as_str().unwrap())));
                        }
                    },
                    None => return Err(String::from("ambassade.json: 'deps->os-x' should be an object."))
                }
            }
        },
        OS::Windows => match config["deps"]["windows"] {
            json!(null) => return Ok(output),
            ref deps => {
                match deps.as_object() {
                    Some(object) => {
                        for dep in object.iter() {
                            if !dep.1.is_string() {
                                return Err(String::from("ambassade.json: all deps should be strings!"))
                            }
                            output.push((dep.0.to_string(), String::from(dep.1.as_str().unwrap())));
                        }
                    },
                    None => return Err(String::from("ambassade.json: 'deps->windows' should be an object."))
                }
            }
        }
    }
    Ok(output)
}

#[cfg(target_os="linux")]
pub fn check(config: serde_json::Value) -> Result<String, String> {
    super::log("Checking dependencies..");

    match dep(config, &OS::Linux) {
        Ok(ref deps) if deps.is_empty() => return Ok(String::from("No dependencies found!")),
        Ok(ref deps) => {
            for dep in deps.iter() {
                let dep_clone = dep.clone();

                super::log(format!("Checking for {}..\n\t{}", dep_clone.0, dep_clone.1));

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
    super::log("Checking dependencies..");

    match dep(config, &OS::MacOs) {
        Ok(ref deps) if deps.is_empty() => return Ok(String::from("No dependencies found!")),
        Ok(ref deps) => {
            for dep in deps.iter() {
                let dep_clone = dep.clone();

                super::log(format!("Checking for {}..\n\t{}", dep_clone.0, dep_clone.1));

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
    super::log("Checking dependencies..");

    match dep(config, &OS::Windows) {
        Ok(ref deps) if deps.is_empty() => return Ok(String::from("No dependencies found!")),
        Ok(ref deps) => {
            for dep in deps.iter() {
                let dep_clone = dep.clone();

                super::log(format!("Checking for {}..\n\t{}", dep_clone.0, dep_clone.1));

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
    let dir = super::filesystem::get_current_dep_root();

    match dir {
        Ok(mut dep_dir) => {
            dep_dir.push(dependency.clone());
            if !dep_dir.exists() {
                dep_dir.pop();
                return super::fetch::build_from_path(dependency, command, dep_dir);
            }
            Ok(String::from("Dependency found."))
        },
        Err(e) => Err(e.to_string())
    }
}

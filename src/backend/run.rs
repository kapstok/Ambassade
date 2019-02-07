extern crate serde_json;

use std::result::Result;
use super::internal::paralellism::Threadhandler;

pub fn run_sync(dep_name: String) -> Result<String, String> {
    let config_path = match super::dep_config::scan(dep_name.clone()) {
        Ok(path) => path,
        Err(e) => return Err(e)
    };

    super::log(format!("Run config path: {:?}", config_path));

    let mut command = String::new();
    match super::config::get_json(&config_path) {
        Ok(config) => {
            if cfg!(target_os = "linux") {
                match config["run"]["linux"].as_str() {
                    Some(string) => command = String::from(string),
                    None => return Err(String::from("ambassade.json: 'run->linux' should be a string."))
                }
            }
            if cfg!(target_os = "macos") {
                match config["run"]["os-x"].as_str() {
                    Some(string) => command = String::from(string),
                    None => return Err(String::from("ambassade.json: 'run->os-x' should be a string."))
                }
            }
            if cfg!(target_os = "windows") {
                match config["run"]["windows"].as_str() {
                    Some(string) => command = String::from(string),
                    None => return Err(String::from("ambassade.json: 'run->windows' should be a string."))
                }
            }
        },
        Err(e) => return Err(e)
    }

    super::log("Running project..");

    let dep_dir = match super::filesystem::get_current_project_root() {
        Some(mut path) => {
            if path.file_name().unwrap().to_str().unwrap() != dep_name {
                path.push("dep");
                path.push(dep_name);
            }
            path
        },
        None => return Err(String::from("Not in a project (sub)directory."))
    };

    super::fetch::run(dep_dir, command)
}

pub fn run_async(dep_name: String, threads: &mut Threadhandler) -> Result<(), String> {
    let config_path = match super::dep_config::scan(dep_name.clone()) {
        Ok(path) => path,
        Err(e) => return Err(e)
    };

    super::log(format!("Run config path: {:?}", config_path));

    let config = match super::config::get_json(&config_path) {
        Ok(json) => json,
        Err(e) => return Err(e)
    };

    run_module(dep_name, config, threads)
}

#[cfg(target_os="linux")]
fn run_module(dep_name: String, config_value: serde_json::Value, threads: &mut Threadhandler) -> Result<(), String> {
    super::log(format!("Scheduling module '{}' as job..", &dep_name));

    let run_cmd = &config_value["run"]["linux"];

    if !run_cmd.is_string() {
        return Err(String::from("ambassade.json: 'run->linux' should be a string."));
    }

    match threads.add(dep_name, String::from(run_cmd.as_str().unwrap())) {
        true => Ok(()),
        false => Err(String::from("Scheduling job failed."))
    }
}

#[cfg(target_os="macos")]
fn run_module(dep_name: String, config_value: serde_json::Value, threads: &mut Threadhandler) -> Result<(), String> {
    super::log(format!("Scheduling module '{}' as job..", &dep_name));

    let run_cmd = &config_value["run"]["os-x"];

    if !run_cmd.is_string() {
        return Err(String::from("ambassade.json: 'run->os-x' should be a string."));
    }

    match threads.add(dep_name, String::from(run_cmd.as_str().unwrap())) {
        true => Ok(()),
        false => Err(String::from("Scheduling job failed."))
    }
}

#[cfg(target_os="windows")]
fn run_module(dep_name: String, config_value: serde_json::Value, threads: &mut Threadhandler) -> Result<(), String> {
    super::log(format!("Scheduling module '{}' as job..", &dep_name));

    let run_cmd = &config_value["run"]["windows"];

    if !run_cmd.is_string() {
        return Err(String::from("ambassade.json: 'run->windows' should be a string."));
    }

    match threads.add(dep_name, String::from(run_cmd.as_str().unwrap())) {
        true => Ok(()),
        false => Err(String::from("Scheduling job failed."))
    }
}

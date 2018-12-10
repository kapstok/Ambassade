extern crate serde_json;

use std::result::Result;
use std::path::PathBuf;
use std::env;

pub fn build(config_file: PathBuf) -> Result<String, String> {
    let config;

    match super::config::get_json(config_file) {
        Ok(result) => config = result,
        Err(e) => return Err(e)
    }

    match super::check::dep(config.clone()) {
        Ok(result) => println!("{}", result),
        Err(e) => return Err(e)
    }

    match build_module(config) {
        Ok(result) => println!("{}", result),
        Err(e) => return Err(e)
    }

    Ok(String::from("Build succeeded!"))
}

#[cfg(target_os="linux")]
fn build_module(config: serde_json::Value) -> Result<String, String> {
    println!("Building project..");

    let build_cmd = &config["build"]["linux"];

    if !build_cmd.is_string() {
        return Err(String::from("beheer.json: 'build->linux' should be a string."));
    }

    super::fetch::fetch(env::current_dir().unwrap(), String::from(build_cmd.as_str().unwrap()))
}

#[cfg(target_os="macos")]
fn build_module(config: serde_json::Value) -> Result<String, String> {
    println!("Building project..");

    let build_cmd = &config["build"]["os-x"];

    if !build_cmd.is_string() {
        return Err(String::from("beheer.json: 'build->os-x' should be a string."));
    }

    super::fetch::fetch(env::get_current_dir().unwrap(), String::from(build_cmd.as_str().unwrap()))
}

#[cfg(target_os="windows")]
fn build_module(config: serde_json::Value) -> Result<String, String> {
    println!("Building project..");

    let build_cmd = &config["build"]["windows"];

    if !build_cmd.is_string() {
        return Err(String::from("beheer.json: 'build->windows' should be a string."));
    }

    super::fetch::fetch(env::get_current_dir().unwrap(), String::from(build_cmd.as_str().unwrap()))
}

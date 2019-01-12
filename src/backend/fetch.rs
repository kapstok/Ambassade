extern crate rustyline;
extern crate serde_json;

use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::result::Result;

pub fn build(dep: PathBuf, mut command: String) -> Result<String, String> {
    let dep_name = String::from(dep.file_name().unwrap().to_str().unwrap());

    if command == String::new() {
        match set_command(&dep_name, true) {
            Some(cmd) => command = cmd,
            None => return Err(String::from("Fetching failed: no appropriate command set."))
        }
    }
    fetch(dep_name, &dep, command)
}

pub fn run(dep: PathBuf, mut command: String) -> Result<String, String> {
    let dep_name = String::from(dep.file_name().unwrap().to_str().unwrap());

    if command == String::new() {
        match set_command(&dep_name, false) {
            Some(cmd) => command = cmd,
            None => return Err(String::from("Fetching failed: no appropriate command set."))
        }
    }
    fetch(dep_name, &dep, command)
}

fn fetch(dep_name: String, path: &PathBuf, command: String) -> Result<String, String> {
    let mut args: Vec<&str> = command.split(' ').collect();
    let command = args.remove(0);
    let out = Command::new(command)
        .current_dir(path)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output();

    match out {
        Ok(response) => {
            match response.status.success() {
                true => Ok(String::from_utf8_lossy(&response.stdout).to_string()),
                false => Err(String::from_utf8_lossy(&response.stderr).to_string())
            }
        },
        Err(e) => {
            let config_path = match super::dep_config::scan(dep_name) {
                Ok(p) => p,
                Err(e) => return Err(e)
            };

            let mut error = String::from("Fetching failed: command '");
            error.push_str(command);
            error.push_str("' invalid. Details: ");
            error.push_str(&e.to_string());
            error.push_str("\n\nConsider changing the above command in the '");

            match config_path.to_str() {
                Some(p) => error.push_str(p),
                None => error.push_str("configuration")
            }

            error.push_str("' file.");
            Err(error)
        }
    }
}

fn set_command(dep: &String, build_cmd: bool) -> Option<String> {
    let mut editor = rustyline::Editor::<()>::new();

    let msg = match build_cmd {
        true => "No build command found. Please enter new command: ",
        false => "No run command found. Please enter new command: "
    };

    let cmd = match editor.readline(msg) {
        Ok(ref input) if input == &String::new() => return None,
        Err(_) => return None,
        Ok(input) => input,
    };

    let result = match build_cmd {
        true => update_module(dep, String::from("build"), cmd.clone()),
        false => update_module(dep, String::from("run"), cmd.clone())
    };

    match result {
        Ok(response) => {
            println!("{}", response);
            Some(cmd)
        },
        Err(e) => {
            println!("{}", e);
            None
        }
    }
}

fn update_module(dep_name: &String, key: String, value: String) -> Result<String, String> {
    let config: serde_json::Value;

    let path = match super::dep_config::scan(dep_name.clone()) {
        Ok(path) => path,
        Err(e) => return Err(e)
    };

    match super::config::get_json(&path) {
        Ok(mut json) => {
            json[key.clone()]["linux"] = json!(value);
            json[key.clone()]["os-x"] = json!(value);
            json[key]["windows"] = json!(value);
            config = json.clone();
        },
        Err(e) => return Err(e)
    }

    match super::config::update(&path, config) {
        Ok(_) => Ok(String::from("Command updated!")),
        Err(e) => Err(e)
    }
}

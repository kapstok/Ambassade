extern crate serde_json;

use std::result::Result;
use std::path::PathBuf;

pub fn add(args: &Vec<String>) -> Result<String, String> {
    let mut it = args.iter().peekable();
    let dep_dir: PathBuf;

    let dep_name: String = match it.next() {
        Some(arg) => (*arg).clone(),
        None => return Err(String::from("Missing dependency name!"))
    };

    match super::filesystem::get_current_dep_root() {
        Ok(mut dir) => {
            dir.push(dep_name.clone());
            dep_dir = dir;
        },
        Err(e) => return Err(e.to_string())
    }

    if dep_dir.exists() {
        return Err(String::from("Dependency already exists."))
    }

    let mut args_s: String = it.map(|arg| arg.clone() + " ").collect();

    match args_s.pop() {
        Some(_) => update_module(dep_name, args_s),
        None => update_module(dep_name, String::new())
    }
}

fn update_module(key: String, value: String) -> Result<String, String> {
    let mut path: PathBuf;
    let config: serde_json::Value;

    match super::filesystem::get_current_module_root() {
        Some(p) => path = p,
        None => return Err(String::from("No config file in module found."))
    }

    path.push("ambassade.json");

    match super::config::get_json(&path) {
        Ok(mut json) => {
            json["deps"]["linux"][key.clone()] = json!(value);
            json["deps"]["os-x"][key.clone()] = json!(value);
            json["deps"]["windows"][key] = json!(value);
            config = json.clone();
        },
        Err(e) => return Err(e)
    }

    match super::config::update(&path, config) {
        Ok(_) => Ok(String::from("Dependency added!")),
        Err(e) => Err(e)
    }
}

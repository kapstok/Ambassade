extern crate serde_json;

use std::result::Result;
use std::path::PathBuf;

pub fn build(dep_name: String) -> Result<String, String> {
    let dep_path = match super::filesystem::search_current_module_root(dep_name.clone()) {
        Ok(path) => path,
        Err(e) => return Err(e.to_string())
    };

    let config_path = match super::dep_config::scan(dep_name) {
        Ok(path) => path,
        Err(e) => return Err(e)
    };

    let config = match super::config::get_json(&config_path) {
        Ok(json) => json,
        Err(e) => return Err(e)
    };

    match super::dep::check(config.clone()) {
        Ok(result) => println!("{}", result),
        Err(e) => return Err(e)
    }

    match build_module(dep_path, config) {
        Ok(result) => println!("{}", result),
        Err(e) => return Err(e)
    }

    Ok(String::from("Build succeeded!"))
}

pub fn build_rec(config_file: PathBuf) -> Result<String, String> {
    let dep_tree = super::deptree::print(&super::system::OS::current(), config_file);

    // Inconventient, should be changed later
    match &dep_tree {
        Ok(result) => println!("{}", result),
        Err(e) => return Err(e.to_string())
    }

    let dep_tree = dep_tree.unwrap();

    for node in &dep_tree.depends_on {
        println!("Building module '{}'..", &node.name);
        match build_rec(node.path.clone()) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }
    }

    println!("Building current project '{}'..", &dep_tree.name);
    match build(dep_tree.name) {
        Ok(result) => println!("{}", result),
        Err(e) => return Err(e)
    }

    Ok(String::from("Project built!"))
}

#[cfg(target_os="linux")]
fn build_module(config_path: PathBuf, config_value: serde_json::Value) -> Result<String, String> {
    println!("Building module..");

    let build_cmd = &config_value["build"]["linux"];

    if !build_cmd.is_string() {
        return Err(String::from("ambassade.json: 'build->linux' should be a string."));
    }

    super::fetch::build(config_path, String::from(build_cmd.as_str().unwrap()))
}

#[cfg(target_os="macos")]
fn build_module(config_path: PathBuf, config_value: serde_json::Value) -> Result<String, String> {
    println!("Building module..");

    let build_cmd = &config_value["build"]["os-x"];

    if !build_cmd.is_string() {
        return Err(String::from("ambassade.json: 'build->os-x' should be a string."));
    }

    super::fetch::build(config_path, String::from(build_cmd.as_str().unwrap()))
}

#[cfg(target_os="windows")]
fn build_module(config_path: PathBuf, config_value: serde_json::Value) -> Result<String, String> {
    println!("Building module..");

    let build_cmd = &config_value["build"]["windows"];

    if !build_cmd.is_string() {
        return Err(String::from("ambassade.json: 'build->windows' should be a string."));
    }

    super::fetch::build(config_path, String::from(build_cmd.as_str().unwrap()))
}

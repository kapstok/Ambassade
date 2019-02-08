extern crate serde_json;

use std::result::Result;
use std::path::PathBuf;

pub fn build(dep_name: String) -> Result<(), String> {
    let config_path = match super::dep_config::scan(dep_name.clone()) {
        Ok(path) => path,
        Err(e) => return Err(e)
    };

    let config = match super::config::get_json(&config_path) {
        Ok(json) => json,
        Err(e) => return Err(e)
    };

    match super::dep::check(config.clone()) {
        Ok(result) => super::log(result),
        Err(e) => return Err(e)
    }

    match build_module(dep_name, config) {
        Ok(result) => super::log(result),
        Err(e) => return Err(e)
    }

    super::log("Build succeeded!");
    Ok(())
}

pub fn build_rec(config_file: PathBuf) -> Result<(), String> {
    let dep_tree = super::deptree::print(&super::system::OS::current(), config_file);

    // Inconventient, should be changed later
    match &dep_tree {
        Ok(result) => super::log(result),
        Err(e) => return Err(e.to_string())
    }

    let dep_tree = dep_tree.unwrap();

    for node in &dep_tree.depends_on {
        super::log(format!("Building module '{}'..", &node.name));
        match build_rec(node.path.clone()) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }
    }

    super::log(format!("Building current project '{}'..", &dep_tree.name));
    match build(dep_tree.name) {
        Ok(_) => {},
        Err(e) => return Err(e)
    }

    super::log("Project built!");
    Ok(())
}

#[cfg(target_os="linux")]
fn build_module(dep_name: String, config_value: serde_json::Value) -> Result<String, String> {
    super::log("Building module..");

    let build_cmd = &config_value["build"]["linux"];

    if !build_cmd.is_string() {
        return Err(String::from("ambassade.json: 'build->linux' should be a string."));
    }

    super::fetch::build(dep_name, String::from(build_cmd.as_str().unwrap()))
}

#[cfg(target_os="macos")]
fn build_module(dep_name: String, config_value: serde_json::Value) -> Result<String, String> {
    super::log("Building module..");

    let build_cmd = &config_value["build"]["os-x"];

    if !build_cmd.is_string() {
        return Err(String::from("ambassade.json: 'build->os-x' should be a string."));
    }

    super::fetch::build(dep_name, String::from(build_cmd.as_str().unwrap()))
}

#[cfg(target_os="windows")]
fn build_module(dep_name: String, config_value: serde_json::Value) -> Result<String, String> {
    super::log("Building module..");

    let build_cmd = &config_value["build"]["windows"];

    if !build_cmd.is_string() {
        return Err(String::from("ambassade.json: 'build->windows' should be a string."));
    }

    super::fetch::build(dep_name, String::from(build_cmd.as_str().unwrap()))
}

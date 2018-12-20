extern crate serde_json;

use super::system::OS;
use super::deptree::Node;
use std::result::Result;
use std::path::PathBuf;
use std::io;

pub fn delete(path: PathBuf) -> Result<String, String> {
    let module = match super::deptree::print(&OS::All, path) {
        Ok(node) => node,
        Err(e) => return Err(e)
    };

    match dep_check(&module) {
        Ok(_) => {},
        Err(e) => return Err(e)
    }

    Ok(module.name)
}

fn dep_check_rec(tree: &Node) -> Result<Vec<Node>, String> {
    let mut nodes: Vec<Node> = Vec::new();

    match super::deptree::dependency_of(&OS::current(), tree) {
        Ok(deps) => {
            for dep in deps {
                println!("Module '{}' depends on '{}'.", &dep.name, tree.name);
                match dep_check_rec(&dep) {
                    Ok(ref new_nodes) if new_nodes.is_empty() => nodes.push(dep.clone()),
                    Ok(new_nodes) => nodes = new_nodes,
                    Err(e) => return Err(e)
                }
            }
        },
        Err(e) => return Err(e)
    }

    Ok(nodes)
}

fn dep_check(node: &Node) -> Result<(), String> {
    let mut input = String::new();
    let mut config_path = PathBuf::new();

    match dep_check_rec(node) {
        Ok(nodes) => {
            for dep in nodes {
                config_path = dep.path;
            }
        },
        Err(e) => return Err(e)
    }

    println!("Continue [y/N]?");

    match io::stdin().read_line(&mut input) {
        Ok(_) if input.as_str() == "y\n" => {
        },
        Ok(_) => return Err(String::from("Aborted.")),
        Err(e) => return Err(e.to_string())
    }

    if config_path == PathBuf::new() {
        return Ok(());
    }

    rm_from_config(config_path, node.name.clone())
}

fn rm_from_config(super_module: PathBuf, dep_name: String) -> Result<(), String> {
    let mut json = match super::config::get_json(super_module.clone()) {
        Ok(config) => config,
        Err(e) => return Err(e)
    };

    match json["deps"]["linux"].as_object_mut() {
        Some(config) => {
            config.remove(&dep_name);
            println!("cfg: {:#?}", config);
        },
        None => { println!("Linux: dep not found."); }
    }

    match json["deps"]["os-x"].as_object_mut() {
        Some(config) => {
            config.remove(&dep_name);
            println!("cfg: {:#?}", config);
        },
        None => { println!("OS-X: dep not found."); }
    }

    match json["deps"]["windows"].as_object_mut() {
        Some(config) => {
            config.remove(&dep_name);
            println!("cfg: {:#?}", config);
        },
        None => { println!("Windows: dep not found."); }
    }

    println!("{} config:\n{}", super_module.file_name().unwrap().to_str().unwrap(), json);
    //super::config::update(super_module, json)
    Ok(())
}

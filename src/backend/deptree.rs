extern crate serde_json;

use super::system::OS;
use std::result::Result;
use std::path::PathBuf;
use std::cmp::PartialEq;
use std::clone::Clone;
use std::fmt;

#[derive(Clone)]
pub struct Node {
    pub name: String,
    pub path: PathBuf,
    pub depends_on: Vec<Node>
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dependency = self.clone();
        let mut string = String::from(self.name.clone());

        string.push('\n');

        for node in &dependency.depends_on {
            let node = node.to_string();

            for line in node.lines() {
                let mut line = String::from(line);

                line.insert(0, '\t');

                string.push_str(line.as_str());
                string.push('\n');
            }
        }

        try!(write!(f, "{}", string));
        Ok(())
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.name == other.name
    }
}

pub fn print(os: &OS, path: PathBuf) -> Result<Node, String> {
    let mut deps: Vec<String> = Vec::new();
    let mut nodes: Vec<Node> = Vec::new();
    let dep_name = String::from(path.file_name().unwrap().to_str().unwrap());

    let config_path = match super::dep_config::scan(dep_name.clone()) {
        Ok(path) => path,
        Err(e) => return Err(e)
    };

    let deps_json = match super::config::get_json(&config_path) {
        Ok(config) => config,
        Err(e) => return Err(e)
    };

    match super::dep::dep(deps_json, os) {
        Ok(vector) => vector.iter().for_each(|dep| deps.push(dep.0.clone())),
        Err(e) => return Err(e)
    }

    for dependency in deps {
        let node: Node;

        match super::filesystem::get_current_dep_root() {
            Ok(mut dir) => {
                dir.push(dependency);
                match print(&os, dir) {
                    Ok(dep) => node = dep,
                    Err(e) => return Err(e.to_string())
                }
            },
            Err(e) => return Err(e.to_string())
        };

        nodes.push(node);
    }

    let root = Node {
        name: dep_name,
        path: path, // What if backend::config::get_json_from_dir(root.path) gets called?
        depends_on: nodes
    };

    Ok(root)
}

pub fn dependency_of(os: &OS, dependency: &Node) -> Result<Vec<Node>, String> {
    match super::filesystem::get_project_root(dependency.path.clone()) {
        Some(path) => dependency_of_rec(os, dependency, path),
        None => Err(String::from("dependency_of: could not find project root."))
    }
}

fn dependency_of_rec(os: &OS, dependency: &Node, root: PathBuf) -> Result<Vec<Node>, String> {
    let mut nodes: Vec<Node> = Vec::new();

    match print(os, root) {
        Ok(tree) => {
            for node in &tree.depends_on {
                if node == dependency {
                    nodes.push(tree.clone());
                }
                nodes.append(&mut dependency_of_rec(os, dependency, node.path.clone()).unwrap());
            }
            Ok(nodes)
        },
        Err(e) => Err(e)
    }
}

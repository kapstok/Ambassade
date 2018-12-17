extern crate serde_json;

use super::system::OS;
use std::result::Result;
use std::path::PathBuf;
use std::fmt;

pub struct Node {
    dep_name: String,
    path: PathBuf,
    depends_on: Vec<Node>
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dependency = self.clone();
        let mut string = String::from(self.dep_name.clone());

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

        dependency = self.clone();

        try!(write!(f, "{}", string));
        Ok(())
    }
}

pub fn print(os: &OS, path: PathBuf) -> Result<Node, String> {
    let mut deps: Vec<String> = Vec::new();
    let mut nodes: Vec<Node> = Vec::new();

    let deps_json = match super::config::get_json(path.clone()) {
        Ok(config) => config,
        Err(e) => return Err(e)
    };

    match super::dep::dep(deps_json, os) {
        Ok(vector) => vector.iter().for_each(|dep| deps.push(dep.0.clone())),
        Err(e) => return Err(e)
    }

    for dependency in deps {
        let node: Node;

        match super::filesystem::get_dep_root() {
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
        dep_name: String::from(path.file_name().unwrap().to_str().unwrap()),
        path: super::filesystem::get_module_root().unwrap(),
        depends_on: nodes
    };

    Ok(root)
}

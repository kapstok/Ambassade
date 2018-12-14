extern crate serde_json;

use super::dep as backend;
use std::result::Result;
use std::option::Option;
use std::path::PathBuf;
use std::rc::Rc;
use std::fmt;

pub struct Node {
    dep_name: String,
    path: PathBuf,
    depends_on: Option<Rc<Node>>
}

// Incomplete
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Incomplete"));
        Ok(())
    }
}

pub fn print(os: backend::OS) -> Result<Node, String> {
    let mut deps: Vec<String> = Vec::new();
    let path: PathBuf;

    match super::filesystem::get_module_root() {
        Some(p) => path = p,
        None => return Err(String::from("Not in a project/dependency directory."))
    }

    let deps_json = match super::config::get_json(path) {
        Ok(config) => config,
        Err(e) => return Err(e)
    };

    match backend::dep(deps_json, os) {
        Ok(vector) => vector.iter().for_each(|tuple| deps.push(tuple.0.clone())),
        Err(e) => return Err(e)
    }

    // Incomplete
    let root = Node {
        dep_name: String::from("root"),
        path: super::filesystem::get_module_root().unwrap(),
        depends_on: None
    };

    Ok(root)
}

use backend;
use backend::system::OS;
use backend::deptree::Node;
use std::process::{Command, Stdio};
use std::path::PathBuf;

pub fn exe_rec(args: Vec<String>) -> Result<(), String> {
    let tree: Node;
    let mut dependency: &Node;

    match backend::filesystem::get_current_project_root() {
        Some(path) => {
            match backend::deptree::print(&OS::current(), path) {
                Ok(node) => {
                    tree = node;
                    dependency = &tree;
                },
                Err(e) => return Err(e)
            }
        }
        None => return Err(String::from("Not in a project (sub)directory."))
    }

    loop {
        match to_shell(&dependency.path, args.clone()) {
            Ok(msg) => {
                backend::normal(msg);
                backend::log(format!("Module {}: {}\n", dependency.name, "Git command successfully executed!"));
            },
            Err(e) => return Err(e)
        }

        match dependency.depends_on.iter().next() {
            Some(dep) => dependency = dep,
            None => break
        }
    }
    Ok(())
}

pub fn exe(path: &PathBuf, args: Vec<String>) -> Result<(), String> {
    match to_shell(path, args) {
        Ok(msg) => {
            backend::normal(msg);
            backend::log("Git command successfully executed!");
            Ok(())
        },
        Err(e) => Err(e)
    }
}

fn to_shell<I>(path: &PathBuf, args: I) -> Result<String, String> where I: IntoIterator<Item=String> {
    let out = Command::new("git")
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
            let mut error = String::from("Git (sub)command failed: command invalid. Details: ");
            error.push_str(&e.to_string());
            Err(error)
        }
    }
}

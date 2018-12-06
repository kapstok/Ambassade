use std::process::Command;

pub fn fetch(dep: String, command: String) {
    let out = Command::new(command).output().expect("");

    println!("{:#?}", out.stderr);
}

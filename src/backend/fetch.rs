use std::process::Command;
use std::path;

pub fn fetch(dep: path::PathBuf, command: String) {
    let mut args: Vec<&str> = command.split(' ').collect();
    let command = args.remove(0);
    let out = Command::new(command).current_dir(dep).args(args).output().expect("");

    println!("{:#?}", out.stderr);
}

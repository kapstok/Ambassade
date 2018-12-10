use std::process::Command;
use std::path;
use std::result::Result;

pub fn fetch(dep: path::PathBuf, command: String) -> Result<String, String> {
    let mut args: Vec<&str> = command.split(' ').collect();
    let command = args.remove(0);
    let out = Command::new(command).current_dir(dep).args(args).output().expect("");

    match out.status.success() {
        true => Ok(String::from_utf8_lossy(&out.stdout).to_string()),
        false => Err(String::from_utf8_lossy(&out.stderr).to_string())
    }
}

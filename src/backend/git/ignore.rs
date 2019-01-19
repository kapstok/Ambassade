use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::fs::OpenOptions;

#[allow(dead_code)]
pub fn add(path: &mut PathBuf, item: &mut String) -> Result<(), String> {
    path.push(".gitignore");
    item.push('\n');

    let file_options = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path);

    match file_options {
        Ok(mut file) => {
                match file.write(item.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string())
                }
        },
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    let mut error = String::from(path.to_str().unwrap());
                    error.push_str(" not found");
                    Err(error)
                },
                ErrorKind::PermissionDenied => Err(String::from("gitignore file unreadable: permission denied.")),
                _ => Err(e.to_string())
            }
        }
    }
}

pub fn init(path: &mut PathBuf) -> Result<(), String> {
    path.push(".gitignore");

    let file_options = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path);

    match file_options {
        Ok(mut file) => {
                match file.write(b"dep\n") {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string())
                }
        },
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    let mut error = String::from(path.to_str().unwrap());
                    error.push_str(" not found");
                    Err(error)
                },
                ErrorKind::PermissionDenied => Err(String::from("gitignore file unreadable: permission denied.")),
                _ => Err(e.to_string())
            }
        }
    }
}

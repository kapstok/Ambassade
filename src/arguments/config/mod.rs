use std::io::{Result, Error, ErrorKind, Write};
use std::fs::File;
use std::path::PathBuf;

pub fn create(mut path: PathBuf) -> Result<()> {
    let content = json!({
        "project-name": path.file_name().unwrap().to_str().unwrap(),
        "version": 0.1
    });

    path.push("beheer.json");

    match File::open(path.to_str().unwrap()) {
        Ok(_) => return Err(Error::new(ErrorKind::AlreadyExists, "Already found a 'beheer.json' file.")),
        Err(_) => {
            match File::create(path) {
                Ok(mut file) => file.write_all(content.to_string().as_bytes())?,
                Err(e) => return Err(e)
            }
        }
    }
    Ok(())
}

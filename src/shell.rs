extern crate rustyline;

use shell::rustyline::error::ReadlineError as Error;
use std::process::{Command, Stdio};
use std::env;
use parser;
use backend;

pub fn shell() {
    let mut dir = env::current_dir().unwrap();
    let mut editor = rustyline::Editor::<()>::new();

    loop {
        backend::output::clear(); // editor.readline() flushes stdout.
        let status = dir.canonicalize().unwrap();
        let mut status = String::from(status.file_name().unwrap().to_str().unwrap());
        status.push_str("> ");

        let input = match editor.readline(status.as_str()) {
            Ok(line) => line,
            Err(Error::Interrupted) => break,
            Err(_) => continue
        };

        let mut input = input.split_whitespace();

        if parser::parse_command(&mut input.clone()) {
            continue;
        }

        match input.next() {
            Some("cd") => match input.next() {
                Some(subdir) => dir.push(subdir),
                None => backend::normal("Missing subdirectory.")
            },
            Some("exit") | Some("quit") => break,
            Some(cmd) => {
                let input: Vec<&str> = input.collect();

                let outp = Command::new(cmd)
                                    .args(input)
                                    .current_dir(&dir)
                                    .stdin(Stdio::inherit())
                                    .stdout(Stdio::inherit())
                                    .output();

                if outp.is_err() {
                    backend::normal("Could not execute command.");
                }
            },
            None => continue
        }
    }
}

extern crate rustyline;

use shell::rustyline::error::ReadlineError as Error;
use std::process::{Command, Stdio};
use std::env;
use parser;

pub fn shell() {
    let mut dir = env::current_dir().unwrap();
    let mut editor = rustyline::Editor::<()>::new();

    loop {
        let input = match editor.readline("ambassade> ") {
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
                None => println!("Missing subdirectory.")
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
                    println!("Could not execute command.");
                }
            },
            None => continue
        }
    }
}

extern crate termcolor;

use std::fmt::Display;
use std::marker::Sized;
use std::io::Write;
use self::termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[allow(unused_must_use)]
pub fn clear() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(&ColorSpec::new());
}

pub fn normal<T: ?Sized>(s: T) where T: Display + Sized {
    clear();
    println!("{}", &s);
}

pub fn log<T: ?Sized>(s: T) where T: Display + Sized {
    let string = Box::new(&s);
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(244,185,66)))).unwrap();
    writeln!(&mut stdout, ":: {}", string).unwrap();
}

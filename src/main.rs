extern crate clap;
use clap::{App, Arg};
use std::io::{self, Read};
use std::process::{Command, Stdio};

fn main() {
    let matches = App::new("dragontail")
        .author("wataru-script")
        .about("Dress up stdin")
        .arg(
            Arg::with_name("keyword")
                .value_name("KEYWORD")
                .help("Keyword")
                .multiple(true)
                .required(false),
        )
        .get_matches();
    let stdin = get_stdin().unwrap();
    println!("{}", stdin);
}

fn get_stdin() -> io::Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;
    Ok(buf)
}

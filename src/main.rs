extern crate clap;
use clap::{App, Arg};
use std::io::{self, Read};
use std::process::{Command, Stdio};

fn main() {
    let stdin = get_stdin().unwrap();
    println!("{}", stdin);
}

fn get_stdin() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

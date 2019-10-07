extern crate clap;
use atty::Stream;
use clap::{App, Arg};
use std::io;
use std::io::prelude::*;
use std::process;
extern crate memmap;

fn main() {
    let matches = App::new("dresscode")
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
    if atty::is(Stream::Stdin) {
        println!("{}", "Error: there is no pipe!");
        process::exit(1);
    }
    let keywords = match matches.values_of_lossy("keyword") {
        Some(k) => k,
        None => Vec::new(),
    };
    get_stdin();
}

fn get_stdin() {
    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        println!("{}", l.unwrap());
    }
}

extern crate clap;
use atty::Stream;
use clap::{App, Arg};
use std::io::{self, Read};
use std::process;

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

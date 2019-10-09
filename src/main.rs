extern crate clap;
use atty::Stream;
use clap::{App, Arg};
use std::io;
use std::io::prelude::*;
use std::process;
use itertools::Itertools;

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
        Some(k) => get_correct_keywords(&k),
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

fn get_correct_keywords(keywords: &Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for (index, value) in keywords.iter().enumerate() {
        let mut flag = false;
        for i in 0..(keywords.len()) {
            if keywords[i].find(value).is_some() && index != i {
                res.push(keywords[i].to_string());
                flag = true;
                break;
            }
            if value.find(&keywords[i]).is_some() && index != i {
                res.push(value.to_string());
                flag = true;
                break;
            }
        }
        if !flag {
            res.push(value.to_string());
        }
    }
    res.into_iter().unique().collect()
}

fn split_line_by_keywords<'a>(line: &'a String, keywords: &Vec<String>) -> Vec<&'a str> {
    let mut matches: Vec<(usize, &str)> = vec![];
    for kw in keywords {
        if kw == "" {
            continue;
        }
        let mut m: Vec<_> = line.match_indices(kw).collect();
        if m.len() > 0 {
            matches.append(&mut m);
        }
    }
    matches.sort_by_key(|k| k.0);
    let mut result: Vec<&str> = vec![];
    let mut count: usize = 0;
    for m in matches {
        if m.0 != 0 {
            result.push(&line[count..m.0]);
        }
        result.push(&line[m.0..(m.0 + m.1.len())]);
        count = m.0 + m.1.len();
    }
    if count != line.len() {
        result.push(&line[count..]);
    }
    result
}

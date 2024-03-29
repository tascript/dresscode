extern crate clap;
use atty::Stream;
use clap::{App, Arg};
use itertools::Itertools;
use std::io;
use std::io::prelude::*;
use std::process;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
    get_stdin(&keywords);
}

fn get_stdin(keywords: &Vec<String>) {
    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let splited_line = split_line_by_keywords(&line, keywords);
        print_colored_line(splited_line, keywords);
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

fn get_colors(index: usize) -> Option<termcolor::Color> {
    let color_val: usize = 6;
    match index % color_val {
        0 => return Some(Color::Magenta),
        1 => return Some(Color::Cyan),
        2 => return Some(Color::Green),
        3 => return Some(Color::Red),
        4 => return Some(Color::Yellow),
        5 => return Some(Color::Blue),
        _ => return None,
    }
}

fn print_colored_line(splited_line: Vec<&str>, keywords: &Vec<String>) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let len = splited_line.len();
    if len == 0 {
        writeln!(&mut stdout, "{}", "").unwrap();
    }
    for i in 0..len {
        for (ki, kw) in keywords.iter().enumerate() {
            if splited_line[i] == kw {
                let color = get_colors(ki);
                stdout.set_color(ColorSpec::new().set_fg(color)).unwrap();
                break;
            } else {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::White)))
                    .unwrap();
            }
        }
        if i != (len - 1) {
            write!(&mut stdout, "{}", splited_line[i]).unwrap();
        } else {
            writeln!(&mut stdout, "{}", splited_line[i]).unwrap();
        }
    }
}

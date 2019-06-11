use clap::{App, Arg};
use ldiff::compare;
use std::{fs, process};

fn main() {
    let matches = App::new("ldiff")
        .version("0.1.0")
        .author("Martin Hagström")
        .about("Compare two files line by line")
        .arg(Arg::with_name("left").required(true))
        .arg(Arg::with_name("right").required(true))
        .get_matches();

    let left_content = read_file(matches.value_of("left").unwrap());
    let right_content = read_file(matches.value_of("right").unwrap());

    let result = compare(left_content, right_content);
    print!("{}", result);
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Could not read \"{}\":\n{}", filename, err);
        process::exit(1)
    })
}

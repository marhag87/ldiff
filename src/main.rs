use clap::{App, Arg};
use ldiff::compare;
use std::fs;

fn main() {
    let matches = App::new("ldiff")
        .version("0.1.0")
        .author("Martin Hagström")
        .about("Compare two files line by line")
        .arg(Arg::with_name("left").required(true))
        .arg(Arg::with_name("right").required(true))
        .get_matches();

    let file_left = matches.value_of("left").unwrap();
    let file_right = matches.value_of("right").unwrap();
    let left_content =
        fs::read_to_string(file_left).expect(&format!("Could not read file: {}", file_left));
    let right_content =
        fs::read_to_string(file_right).expect(&format!("Could not read file: {}", file_right));

    let result = compare(left_content, right_content);
    print!("{}", result);
}

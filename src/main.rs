use ldiff::compare;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_left = &args[1];
    let file_right = &args[2];

    let left_content =
        fs::read_to_string(file_left).expect(&format!("Could not read file: {}", file_left));
    let right_content =
        fs::read_to_string(file_right).expect(&format!("Could not read file: {}", file_right));

    let result = compare(left_content, right_content);
    print!("{}", result);
}

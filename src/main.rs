use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_left = &args[1];
    let file_right = &args[2];

    let left_content =
        fs::read_to_string(file_left).expect(&format!("Could not read file: {}", file_left));
    let right_content =
        fs::read_to_string(file_right).expect(&format!("Could not read file: {}", file_right));

    let left_lines: Vec<&str> = left_content.trim().split('\n').collect();
    let right_lines: Vec<&str> = right_content.trim().split('\n').collect();

    let max = left_lines
        .iter()
        .chain(right_lines.iter())
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .len();

    left_lines
        .iter()
        .zip(right_lines.iter())
        .for_each(|(left, right)| {
            if left == right {
                print!("{:width$} = {}", left, right, width = max);
                println!();
            } else {
                print!("{:width$} | {}", left, right, width = max);
                println!();
            }
        });
}

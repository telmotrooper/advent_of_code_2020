use std::{fs, num::ParseIntError};

fn main() {
    let file_content: String = fs::read_to_string("src/input.txt")
        .expect("Unable to read file.");

    let lines: Vec<&str> = file_content.split("\n").collect();

    let x: Vec<Result<i32, ParseIntError>> = lines.iter().map(|&line| line.parse::<i32>()).collect();

    println!("{:?}", x);
}

use std::fs;

fn main() {
    let file_content: String = fs::read_to_string("src/input.txt")
        .expect("Unable to read file.");

    let lines: Vec<&str> = file_content.split("\n").collect();

    let numbers: Vec<i32> = lines.iter().map(|&line| line.parse::<i32>())
        .filter_map(Result::ok).collect();

    println!("{:?}", numbers);
}

use std::fs;

fn main() {
    let file_content: String = fs::read_to_string("src/input.txt")
        .expect("Unable to read file.");

    let lines: Vec<&str> = file_content.lines().collect();

    let numbers: Vec<i32> = lines.iter().filter_map(|&line| line.parse().ok()).collect();

    println!("{:?}", numbers);
}

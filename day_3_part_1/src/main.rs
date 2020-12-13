use std::fs;

fn main() {
    let file_content: String = fs::read_to_string("src/input.txt")
        .expect("Unable to read file.");

    let lines: Vec<&str> = file_content.lines().collect();

    let mut map: Vec<Vec<char>> = Vec::new();

    // Build map.
    for line in lines.iter() {
        map.push(line.chars().collect());       
    }

    println!("{:?}\n", map);
}

use std::fs;

fn main() {
    let file_content: String = fs::read_to_string("src/input.txt")
        .expect("Unable to read file.");

    let lines: Vec<&str> = file_content.lines().collect();

    println!("{:?}\n", lines);

    // for x in numbers.iter() {
    //
    // }
}

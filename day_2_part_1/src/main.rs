use std::fs;

fn main() {
    let file_content: String = fs::read_to_string("src/input.txt")
        .expect("Unable to read file.");

    let lines: Vec<&str> = file_content.lines().collect();

    for entry in lines.iter() {
        let mut split = entry.splitn(2, ": ");
        let tuple: (&str, &str) = (split.next().unwrap(), split.next().unwrap());
        println!("{:?}", tuple);
    }
}

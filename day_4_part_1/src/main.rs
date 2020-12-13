use std::fs;

fn main() {
    let file_content: String = fs::read_to_string("src/input.txt")
        .expect("Unable to read file.");

    let lines: Vec<&str> = file_content.lines().filter(|x| *x != "").collect();

    for line in lines {
        let vec: Vec<&str> = line
            .split(" ")
            .collect(); // E.g. ["ecl:gry", "pid:860033327", "eyr:2020", "hcl:#fffffd"]

        println!("{:?}\n", vec);
    }
}

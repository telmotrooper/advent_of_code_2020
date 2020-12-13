use std::fs;

#[derive(Debug)]
struct Entry<'e> {
    position1: usize,
    position2: usize,
    character: char,
    password: &'e str
}

fn main() {
    let file_content: String = fs::read_to_string("src/input.txt")
        .expect("Unable to read file.");

    let lines: Vec<&str> = file_content.lines().collect();

    let mut valid_password_counter = 0;

    for line in lines.iter() {
        let entry_vector: Vec<&str> = line
            .splitn(2, ": ")
            .flat_map(|y: &str| y.splitn(2, "-"))
            .flat_map(|z: &str| z.splitn(2, " "))
            .collect(); // E.g. ["8", "9", "n", "nnnnnnnnn"]
        
        let entry = Entry {
            position1: entry_vector[0].parse().unwrap(),
            position2: entry_vector[1].parse().unwrap(),
            character: entry_vector[2].parse().unwrap(),
            password: entry_vector[3]
        };

        let x = entry.password.chars().nth(entry.position1 - 1).unwrap_or_default() == entry.character;
        let y = entry.password.chars().nth(entry.position2 - 1).unwrap_or_default() == entry.character;
        
        if !(x && y) && (x || y) {
            valid_password_counter += 1;
            println!("{:?} is a valid password for policy \"{:?}-{:?} {:?}\".",
                entry.password, entry.position1, entry.position2, entry.character);
        }
    }

    println!("{:?} passwords are valid.", valid_password_counter);
}

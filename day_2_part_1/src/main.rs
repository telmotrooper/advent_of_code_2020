use std::fs;

#[derive(Debug)]
struct Entry<'e> {
    minimum: u8,
    maximum: u8,
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
            minimum: entry_vector[0].parse().unwrap(),
            maximum: entry_vector[1].parse().unwrap(),
            character: entry_vector[2].parse().unwrap(),
            password: entry_vector[3]
        };

        let mut char_counter = 0;

        for c in entry.password.chars() {
            if c == entry.character {
                char_counter += 1;
            }
        }

        if char_counter >= entry.minimum && char_counter <= entry.maximum {
            valid_password_counter += 1;
            println!("{:?} is a valid password.", entry.password);
        }
    }

    println!("{:?} passwords are valid.", valid_password_counter);
}

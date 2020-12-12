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

    for entry in lines.iter() {
        let x: Vec<&str> = entry
        .splitn(2, ": ")
        .flat_map(|y: &str| y.split("-"))
        .collect();
        
        println!("{:?}", x);
        // let mut split = entry.splitn(2, ": "); // E.g. "8-9 n", "nnnnnnnnn"

        // let policy: &str = split.next().unwrap();

        // let mut policy_split = policy.splitn(2, "-"); // E.g. "8", "9 n"
        // let min = policy_split.next().unwrap();
        // // let min_max: (&str, &str) = 
        // let password: &str = split.next().unwrap();
        // println!("{:?} / {:?}", policy, password);
    }
}

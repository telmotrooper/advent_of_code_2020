use std::fs;

#[derive(Debug)]
struct Passport<'p> {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<&'p str>,
    hcl: Option<&'p str>,
    ecl: Option<&'p str>,
    pid: Option<&'p str>,
    cid: Option<u16>
}

fn main() {
    let file_content: String = fs::read_to_string("src/input.txt")
        .expect("Unable to read file.");

    let lines: Vec<&str> = file_content.lines().filter(|x| *x != "").collect();

    for line in lines {
        let vec: Vec<Vec<&str>> = line.split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.split(":").collect())
            .collect(); // E.g. [["hcl", "#cfa07d"], ["byr", "1929"]]

        println!("{:?}\n", vec);
    }
}

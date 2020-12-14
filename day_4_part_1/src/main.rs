use std::collections::HashMap;
use std::fs;

fn main() {
    let file_content: String = fs::read_to_string("src/input.txt")
        .expect("Unable to read file.");

    let lines: Vec<&str> = file_content.lines().filter(|x| *x != "").collect();

    for line in lines {
        let entry: Vec<Vec<&str>> = line.split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.split(":").collect())
            .collect(); // E.g. [["hcl", "#cfa07d"], ["byr", "1929"]]


        let mut passport: HashMap<&str, &str> = HashMap::new();;

        for field in entry {
            passport.insert(
                field.iter().nth(0).unwrap(), 
                field.iter().nth(1).unwrap()
            );
        }

        // println!("{:?}\n", passport);

        if passport.contains_key("byr") && passport.contains_key("iyr")
        && passport.contains_key("eyr") && passport.contains_key("hgt")
        && passport.contains_key("hcl") && passport.contains_key("ecl")
        && passport.contains_key("pid") /*&& passport.contains_key("cid")*/ {
            println!("{:?} is a valid passport.\n", passport);
        }


    }
}

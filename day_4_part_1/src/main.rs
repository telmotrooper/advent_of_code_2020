use std::collections::HashMap;
use std::fs;

fn main() {
    let file_content: String = fs::read_to_string("src/input.txt")
        .expect("Unable to read file.");

    let lines: Vec<&str> = file_content.lines().collect();

    let mut vec: Vec<Vec<&str>> = Vec::new();

    for line in lines {
        let mut data: Vec<Vec<&str>> = line.split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.split(":").collect())
            .collect(); // E.g. [["hcl", "#cfa07d"], ["byr", "1929"]]
        
        vec.append(&mut data);
    }

    let mut passports: Vec<HashMap<&str, &str>> = Vec::new();

    let mut current_passport: usize = 0;

    passports.push(HashMap::new());

    for field in vec {
        if field.iter().nth(0).unwrap() != &"" {
            let passport = passports.get_mut(current_passport).unwrap();

            passport.insert(
                field.iter().nth(0).unwrap(), 
                field.iter().nth(1).unwrap()
            );

        } else { // If an empty string was found, it's time to advance to the next passport.
            current_passport += 1;
            passports.push(HashMap::new());
        }
    }

    let mut valid_passport_counter = 0;

    for passport in passports.iter() {
        if passport.contains_key("byr") && passport.contains_key("iyr")
        && passport.contains_key("eyr") && passport.contains_key("hgt")
        && passport.contains_key("hcl") && passport.contains_key("ecl")
        && passport.contains_key("pid") {
            valid_passport_counter += 1;
            println!("{:?} is a valid passport.\n", passport);
        }
    }

    println!("{:?} passports are valid.", valid_passport_counter)
}

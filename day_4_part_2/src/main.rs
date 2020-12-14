use regex::Regex;

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

    let re_digits = Regex::new(r"\d+").unwrap();
    let re_non_digits = Regex::new(r"\D+").unwrap();
    let re_hexadecimal = Regex::new(r"[a-f0-9]+").unwrap();

    for passport in passports.iter() {
        if passport.contains_key("byr") && passport.contains_key("iyr")
        && passport.contains_key("eyr") && passport.contains_key("hgt")
        && passport.contains_key("hcl") && passport.contains_key("ecl")
        && passport.contains_key("pid") {
            // Birth Year
            let byr = passport.get("byr").unwrap();
            let parsed_byr: u16 = byr.parse().unwrap_or_default();

            if parsed_byr < 1920 || parsed_byr > 2002 {
                continue;
            }

            // Issue Year
            let iyr = passport.get("iyr").unwrap();
            let parsed_iyr: u16 = iyr.parse().unwrap_or_default();

            if parsed_iyr < 2010 || parsed_iyr > 2020 {
                continue;
            }

            // Expiration Year
            let eyr = passport.get("eyr").unwrap();
            let parsed_eyr: u16 = eyr.parse().unwrap_or_default();

            if parsed_eyr < 2020 || parsed_eyr > 2030 {
                continue;
            }

            // Height
            let hgt = passport.get("hgt").unwrap();

            let digits = re_digits.captures(hgt);
            let non_digits = re_non_digits.captures(hgt);

            if digits.is_none() || non_digits.is_none() {
                continue;
            }

            let unit = non_digits.unwrap().get(0).unwrap().as_str();

            if unit != "cm" && unit != "in" {
                continue;
            }

            let digits: u8 = digits.unwrap().get(0).unwrap().as_str().parse().unwrap();

            if unit == "cm" && (digits < 150 || digits > 193) {
                continue;
            }

            if unit == "in" && (digits < 59 || digits > 76) {
                continue;
            }

            // Hair Color
            let hcl = passport.get("hcl").unwrap().to_string();

            if hcl.len() != 7 || &hcl[0..1] != "#" {
                continue;
            }

            let hexadecimal = re_hexadecimal.captures(hcl.as_str());

            if hexadecimal.is_some() && hexadecimal.unwrap().get(0).unwrap().as_str().len() != 6 {
                continue;
            }

            valid_passport_counter += 1;
            // println!("{:?} is a valid passport.\n", passport);
        }
    }

    println!("{:?} passports are valid.", valid_passport_counter)
}

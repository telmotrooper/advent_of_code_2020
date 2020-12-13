use std::fs;

fn main() {
    let file_content: String = fs::read_to_string("src/input.txt")
        .expect("Unable to read file.");

    let lines: Vec<&str> = file_content.lines().collect();

    let mut map: Vec<Vec<char>> = Vec::new();

    // Build map.
    for line in lines.iter() {
        map.push(line.chars().collect());       
    }

    let x_limit = map.iter().nth(0).unwrap().len();
    let y_limit = map.len();

    println!("The map is {:?}x{:?} in size.", x_limit, y_limit);

    let mut trees_found = 0;
    let mut current_pos: (usize, usize) = (0,0);

    let mut i = 0;

    while i < map.len() {
        // println!("Going further.");
        i += 1;
    }

    println!("{:?}\n", map);
}

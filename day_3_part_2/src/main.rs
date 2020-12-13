use std::fs;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize
}

fn traverse_slopes(map: &Vec<Vec<char>>, steps_right: usize, steps_down: usize) -> u64 {
    let x_limit = map.iter().nth(0).unwrap().len();
    let y_limit = map.len();

    // println!("The map is {:?}x{:?} in size.", x_limit, y_limit);

    let mut trees_found: u64 = 0;
    let mut current_pos = Position { x: 0, y: 0 };

    while current_pos.y < y_limit - 1 {
        current_pos = Position {
            x: (current_pos.x + steps_right) % x_limit,
            y: current_pos.y + steps_down
        };

        let location = map
            .iter().nth(current_pos.y).unwrap()
            .iter().nth(current_pos.x).unwrap();

        if *location == '#' {
            trees_found += 1;
            // println!("Tree found at position ({:?},{:?}).", current_pos.x, current_pos.y);
        }
    }

    return trees_found;
}

fn main() {
    let file_content: String = fs::read_to_string("src/input.txt")
        .expect("Unable to read file.");

    let lines: Vec<&str> = file_content.lines().collect();

    let mut map: Vec<Vec<char>> = Vec::new();

    // Build map.
    for line in lines.iter() {
        map.push(line.chars().collect());       
    }

    let trees_found_1_1 = traverse_slopes(&map, 1, 1);
    let trees_found_3_1 = traverse_slopes(&map, 3, 1);
    let trees_found_5_1 = traverse_slopes(&map, 5, 1);
    let trees_found_7_1 = traverse_slopes(&map, 7, 1);
    let trees_found_1_2 = traverse_slopes(&map, 1, 2);

    println!("{:?} tree(s) found.\n", trees_found_1_1);
    println!("{:?} tree(s) found.\n", trees_found_3_1);
    println!("{:?} tree(s) found.\n", trees_found_5_1);
    println!("{:?} tree(s) found.\n", trees_found_7_1);
    println!("{:?} tree(s) found.\n", trees_found_1_2);

    let multiplied: u64 = trees_found_1_1 * trees_found_3_1 * trees_found_5_1 * trees_found_7_1 * trees_found_1_2;

    println!("{:?} * {:?} * {:?} * {:?} * {:?} = {:?}",
        trees_found_1_1, trees_found_3_1, trees_found_5_1,
        trees_found_7_1, trees_found_1_2, multiplied);
}

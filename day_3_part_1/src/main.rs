use std::fs;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize
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

    let x_limit = map.iter().nth(0).unwrap().len();
    let y_limit = map.len();

    println!("The map is {:?}x{:?} in size.", x_limit, y_limit);

    let mut trees_found = 0;
    let mut current_pos = Position { x: 0, y: 0 };

    while current_pos.y < y_limit - 1 {
        current_pos = Position {
            x: (current_pos.x + 3) % x_limit,
            y: current_pos.y + 1
        };

        let location = map
            .iter().nth(current_pos.y).unwrap()
            .iter().nth(current_pos.x).unwrap();

        if *location == '#' {
            trees_found += 1;
            println!("Tree found at position ({:?},{:?}).", current_pos.x, current_pos.y);
        }
    }

    println!("{:?} tree(s) found.\n", trees_found);
}

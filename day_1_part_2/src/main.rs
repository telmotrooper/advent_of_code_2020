use std::fs;

fn main() {
    let file_content: String = fs::read_to_string("src/input.txt")
        .expect("Unable to read file.");

    let lines: Vec<&str> = file_content.lines().collect();

    let numbers: Vec<i32> = lines.iter().filter_map(|&line| line.parse().ok()).collect();

    println!("{:?}\n", numbers);

    let expected_sum = 2020;

    for x in numbers.iter() { // I should probably ignore the pairs that were already checked, but right now this is O(n²).
        for y in numbers.iter() {
            for z in numbers.iter() {
                if x + y + z == expected_sum {
                    println!("{:?} + {:?} + {:?} = {:?}", x, y, z, expected_sum);
                    println!("{:?} * {:?} * {:?} = {:?}\n", x, y, z, x * y * z);
                }
            }
        }
    }
}

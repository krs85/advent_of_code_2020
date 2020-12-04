use std::fs;

fn main() {
    let mut valid_count_part_one = 0;
    let mut valid_count_part_two = 0;

    for line in fs::read_to_string("input.txt")
        .expect("failed to read")
        .lines()
    {
        let split_line: Vec<&str> = line.split(' ').collect();
        let nums: Vec<u64> = split_line[0]
            .split('-')
            .map(|x| x.parse().expect("failed to parse"))
            .collect();
        let (first, second) = (nums[0], nums[1]);
        let character = split_line[1].chars().next().expect("no char");
        let password = split_line[2];

        if is_valid_part_one(first, second, character, password) {
            valid_count_part_one += 1;
        }

        if is_valid_part_two(first, second, character, password) {
            valid_count_part_two += 1;
        }
    }

    println!("valid_count_part_one: {}", valid_count_part_one);
    println!("valid_count_part_two: {}", valid_count_part_two);
}

// Part 1:
fn is_valid_part_one(first: u64, second: u64, letter: char, password: &str) -> bool {
    let total = password.chars().filter(|c| *c == letter).count();
    total >= first as usize && total <= second as usize
}

// Part 2:
fn is_valid_part_two(first: u64, second: u64, letter: char, password: &str) -> bool {
    // Their index doesn't start at zero.
    let first_pos = first - 1;
    let second_pos = second - 1;

    let first_char = password
        .chars()
        .nth(first_pos as usize)
        .expect("couldn't get char");
    let second_char = password
        .chars()
        .nth(second_pos as usize)
        .expect("couldn't get char");

    match (first_char == letter, second_char == letter) {
        (false, false) => false,
        (true, true) => false,
        (_, _) => true,
    }
}

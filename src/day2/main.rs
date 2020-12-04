use std::fs;

fn main() {
    let mut valid_count = 0;
    for line in fs::read_to_string("input.txt").expect("failed to read").lines() {
        let split_line: Vec<&str> = line.split(' ').collect();
        let range: Vec<u64> = split_line[0].split('-').map(|x| x.parse().expect("failed to parse")).collect();
        let (min, max) = (range[0], range[1]);
        let character = split_line[1].chars().next().expect("no char");
        let password = split_line[2];

        if is_valid(max, min, character, String::from(password)) {
            valid_count += 1;
        }
    }

    println!("valid_count: {}", valid_count);

}

// examples:
// 1-3 a: abcde -- valid
// 1-3 b: cdefg -- not valid
// 2-9 c: ccccccccc -- valid
fn is_valid(max: u64, min: u64, letter: char, password: String) -> bool {
    let total = password.chars().filter(|c| *c == letter).count();
    total >= min as usize && total <= max as usize 
}
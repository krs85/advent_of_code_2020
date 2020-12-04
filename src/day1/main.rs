use std::fs;

fn main() {
    let contents: Vec<u64> = fs::read_to_string("input.txt")
        .expect("failed to read")
        .lines()
        .map(|l| l.parse().expect("failed to parse"))
        .collect();

    // Part 1:
    // let (first, second) = find_two(contents);
    // println!("first: {}, second: {}", first, second);
    // let product = first * second;
    // println!("product: {}", product);

    // Part 2:
    let (first, second, third) = find_three(contents);
    println!("first: {}, second: {}, third: {}", first, second, third);
    let product = first * second * third;
    println!("product: {}", product);
}

// Part 1:
// Find two numbers that sum to 2020
fn find_two(numbers: Vec<u64>) -> (u64, u64) {
    let length = numbers.len();

    for i in 0..length {
        for j in 0..length {
            if i != j {
                let x = numbers[i];
                let y = numbers[j];

                let sum = x + y;
                if sum == 2020 {
                    return (x, y);
                }
            }
        }
    }
    (0, 0)
}

// Part 2:
// Find three numbers that sum to 2020
fn find_three(numbers: Vec<u64>) -> (u64, u64, u64) {
    let length = numbers.len();

    for i in 0..length {
        for j in 0..length {
            if i != j {
                for k in 0..length {
                    if k != i && k != j {
                        let x = numbers[i];
                        let y = numbers[j];
                        let z = numbers[k];
                        let sum = x + y + z;
                        if sum == 2020 {
                            return (x, y, z);
                        }
                    }
                }
            }
        }
    }
    (0, 0, 0)
}

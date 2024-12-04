use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let re = Regex::new(r"mul\(([-\d]+),([-\d]+)\)|do\(\)|don't\(\)").unwrap();
    let file = read_to_string("input.txt").expect("Failed to read file");

    let mut total = 0;
    let mut enabled = true;
    for capt in re.captures_iter(&file) {
        if capt[0] == *"do()" {
            enabled = true;
        } else if capt[0] == *"don't()" {
            enabled = false;
        } else if enabled {
            let a: i32 = capt[1].parse().expect("Failed to interpret a as number");
            let b: i32 = capt[2].parse().expect("Failed to interpret b as number");
            total += a*b;
        }
    }

    println!("{total}");
}

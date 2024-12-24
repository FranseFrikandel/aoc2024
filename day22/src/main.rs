use std::time::Instant;
use std::fs::read_to_string;
use std::vec::Vec;

fn main() {
    let timer = Instant::now();
    let mut results: Vec<usize> = Vec::new();
    let mut p1_sum: usize = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let mut secret: usize = line.parse().expect("Failed to parse to integer");
        for _ in 0..2000 {
            // *64 same as bitshift left 6?
            secret = (secret ^ (secret*64)) % 16777216;
            secret = (secret ^ (secret/32)) % 16777216;
            secret = (secret ^ (secret * 2048)) % 16777216;
        }
        p1_sum += secret;
    }
    println!("Sum of random numbers: {}", p1_sum);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

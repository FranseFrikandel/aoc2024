use std::fs::{read_to_string};
use std::time::Instant;

fn main() {
    let timer = Instant::now();
    let mut corr_sum = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let line_arr: Vec<&str> = line.split(": ").collect();
        let res: usize = line_arr[0].parse().expect("Cant parse to int");
        let parameters: Vec<usize> = line_arr[1].split(" ").map(|str| str.parse().expect("Cant parse to int")).collect();
        for i in 0..2_usize.pow((parameters.len()-1) as u32) {
            if calculate(&parameters, i) == res {
                corr_sum += res;
                break;
            }
        }
    }
    println!("Total amount of correct formula's: {}", corr_sum);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn calculate(params: &Vec<usize>, operators: usize) -> usize {
    let mut sum = params[0];
    for i in 1..params.len() {
        if operators & (1 << i-1) != 0 {
            sum = sum + params[i];
        } else {
            sum = sum * params[i];
        }
    }
    return sum;
}
use std::fs::{read_to_string};
use std::time::Instant;

fn main() {
    let timer = Instant::now();
    let mut corr_sum = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        println!("{}", line);
        let line_arr: Vec<&str> = line.split(": ").collect();
        let res: usize = line_arr[0].parse().expect("Cant parse to int");
        let parameters: Vec<usize> = line_arr[1].split(" ").map(|str| str.parse().expect("Cant parse to int")).collect();
        for operators in 0..4_usize.pow((parameters.len()-1) as u32) {
            for j in 0..std::mem::size_of::<usize>()/2 {
                let operator_pair = operators & (0b11 << j*2);
                if operator_pair & operator_pair >> 1 != 0 {
                    // There's a case where both operators are set, skip this.
                    continue
                }
            }
            if calculate(&parameters, operators) == res {
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
    for i in 0..params.len()-1 {
        if operators & (0b01 << i*2) != 0 {
            let sum_str = sum.to_string();
            let param_str = params[i+1].to_string();
            sum = (sum_str + &param_str).parse().expect("Concat failed");
        } else if operators & (0b10 << i*2) != 0 {
            sum = sum * params[i+1];
        } else {
            sum = sum + params[i+1];
        }
    }
    return sum;
}
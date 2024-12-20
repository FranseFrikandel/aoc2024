use std::time::Instant;
use std::thread;
use std::fs::read_to_string;
// use itertools::Itertools;

fn main() {
    let timer = Instant::now();

    let filestr = read_to_string("input.txt").unwrap();
    let mut lines = filestr.lines();
    let mut towels: Vec<&str> = lines.next().unwrap().split(", ").collect();
    // Sorting from large to small seems to speed up the example input, but not the actual input
    // towels.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut p1_sum = 0;
    let mut p2_sum = 0;
    // for line_chunk in lines.chunks(50) {

    // }
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let sum = try_finish_line(line, &towels);
        p2_sum += sum;
        if sum > 0 {
            p1_sum += 1;
        }
    }

    println!("Amount of completed lines: {}", p1_sum);
    println!("Total possible designs: {}", p2_sum);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn try_finish_line(line: &str, towels: &Vec<&str>) -> usize {
    let mut sum = 0;
    if line.len() == 0 {
        return 1;
    }
    for towel in towels.iter() {
        if line.starts_with(towel) {
            let stripped_line = &line[towel.len()..];
            sum += try_finish_line(stripped_line, towels);
            if sum == 1 {
                break
            }
        };
    }
    return sum;
}
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use std::fs::read_to_string;
use std::cmp;
// use itertools::Itertools;
const NUM_HASHSETS: usize = 10;

fn main() {
    let timer = Instant::now();

    let filestr = read_to_string("input.txt").unwrap();
    let mut lines = filestr.lines();
    // Split into seperate hashset for each length. Need to make sure there's enough hashmaps for all lengths.
    // Edit: This did not help at all with performance.
    let mut towels: [HashSet<&str>; NUM_HASHSETS] = Default::default();
    let mut max_towel_length = 0;

    for towel in lines.next().unwrap().split(", ") {
        towels[towel.len()-1].insert(towel);
        max_towel_length = cmp::max(max_towel_length, towel.len());
    }

    let mut p1_sum = 0;
    let mut p2_sum = 0;

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let mut cache: HashMap<&str, usize> = HashMap::new();
        let sum = try_finish_line(line, &towels, max_towel_length, &mut cache);
        p2_sum += sum;
        if sum > 0 {
            p1_sum += 1;
        }
    }

    println!("Amount of completed lines: {}", p1_sum);
    println!("Total possible designs: {}", p2_sum);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn try_finish_line<'a>(
    line: &'a str, 
    towels: &[HashSet<&str>; NUM_HASHSETS], 
    max_len: usize, 
    cache: &mut HashMap<&'a str, usize>) -> usize 
{
    let mut sum = 0;
    if line.len() == 0 {return 1}
    if let Some(v) = cache.get(line) {return *v}
    for i in 0..cmp::min(max_len, line.len()) {
        let lineseg = &line[0..i+1];
        let towel_group = &towels[i];
        if towel_group.contains(lineseg) {
            let stripped_line = &line[i+1..];
            sum += try_finish_line(stripped_line, towels, max_len, cache);
        }
    }
    cache.insert(line, sum);
    return sum;
}
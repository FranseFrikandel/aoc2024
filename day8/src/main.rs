use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn main() {
    let timer = Instant::now();

    let mut antennas: HashMap<char, Vec<[usize; 2]>> = HashMap::new();
    let mut y_size = 0;
    let mut x_size = 0;
    for (i, line) in read_to_string("input.txt").unwrap().lines().enumerate() {
        y_size = i;
        for (j, ch) in line.chars().enumerate() {
            if ch != '.' {
                match antennas.entry(ch) {
                    Vacant(ant_vec) => {ant_vec.insert(vec![[i, j]]);},
                    Occupied(ant_vec) => {
                        let entry = ant_vec.into_mut();
                        entry.push([i, j]);
                    }
                }
            }
            if j > x_size {
                x_size = j;
            }
        }
    }

    let antinode_w: Vec<bool> = vec![false; x_size+1];
    let mut antinodes: Vec<Vec<bool>> = vec![antinode_w; y_size+1];
    for (_freq, freq_antennas) in &mut antennas {
        while freq_antennas.len() >= 2 {
            let last_antenna = freq_antennas.pop().expect("Could not pop antenna");
            for antenna in &freq_antennas[0..freq_antennas.len()] {
                let anti_a;
                let anti_b;
                (anti_a, anti_b) = calculate_antinodes(antenna, &last_antenna);
                for anti in [anti_a, anti_b] {
                    if !anti.contains(&None) {
                        match antinodes.get_mut(anti[0].expect("")) {
                            Some(line) => {
                                match line.get_mut(anti[1].expect("")) {
                                    Some(node) => *node = true,
                                    None => {}
                                }
                            },
                            None => {}
                        }
                    }
                }
            }
        }
    }

    let mut sum: usize = 0;
    for line in antinodes {
        for el in line {
            if el == true {
                sum += 1;
            }
        };
    }
    println!("Total sum: {}", sum);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn calculate_antinodes(a: &[usize; 2], b: &[usize; 2]) -> ([Option<usize>; 2], [Option<usize>; 2]) {
    let antinode_a = [(2*a[0]).checked_sub(b[0]), (2*a[1]).checked_sub(b[1])];
    let antinode_b = [(2*b[0]).checked_sub(a[0]), (2*b[1]).checked_sub(a[1])];
    return (antinode_a, antinode_b);
}

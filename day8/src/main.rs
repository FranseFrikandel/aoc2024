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

    x_size += 1;
    y_size += 1;

    let antinode_w: Vec<bool> = vec![false; x_size];
    let mut antinodes: Vec<Vec<bool>> = vec![antinode_w; y_size];
    for (_freq, freq_antennas) in &mut antennas {
        while freq_antennas.len() >= 2 {
            let last_antenna = freq_antennas.pop().expect("Could not pop antenna");
            for antenna in &freq_antennas[0..freq_antennas.len()] {
                let antinode_pair = calculate_antinodes(antenna, &last_antenna, y_size, x_size);
                for anti in antinode_pair {
                    antinodes[anti[0]][anti[1]] = true;
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

fn calculate_antinodes(a: &[usize; 2], b: &[usize; 2], max_y: usize, max_x: usize) -> Vec<[usize; 2]> {
    let mut antinodes: Vec<[usize; 2]> = Vec::new();
    let diff: [isize; 2] = [b[0] as isize - a[0] as isize, b[1] as isize - a[1] as isize];

    antinodes.push(*a);
    let mut cur_pos = [a[0] as isize, a[1] as isize];
    // Loop through all a+diff options
    loop {
        cur_pos[0] = cur_pos[0] + diff[0];
        cur_pos[1] = cur_pos[1] + diff[1];
        if cur_pos[0] < 0 || cur_pos[1] < 0 {
            break;
        }
        if cur_pos[0] as usize >= max_y || cur_pos[1] as usize >= max_x {
            break;
        }
        antinodes.push([cur_pos[0] as usize, cur_pos[1] as usize]);
    };
    
    let mut cur_pos = [a[0] as isize, a[1] as isize];
    loop {
        cur_pos[0] = cur_pos[0] - diff[0];
        cur_pos[1] = cur_pos[1] - diff[1];
        if cur_pos[0] < 0 || cur_pos[1] < 0 {
            break;
        }
        if cur_pos[0] as usize >= max_y || cur_pos[1] as usize >= max_x {
            break;
        }
        antinodes.push([cur_pos[0] as usize, cur_pos[1] as usize]);
    };
    return antinodes;
}

use std::fs::{read, read_to_string};

fn main() {
    // Read file
    let mut position;
    let mut travel_dir;
    let mut obstructions;
    (position, travel_dir, obstructions) = read_file();

    // run
    let mut is_loop: bool;
    let travelled_path: Vec<Vec<bool>>;
    (is_loop, travelled_path) = run_path(&obstructions, position, travel_dir);

    let mut base_sum: usize = 0;
    for i in travelled_path {
        for j in i {
            if j {base_sum += 1};
        }
    }
    println!("The length of base path is: {base_sum}");

    // run w/ obstructions
    let mut loop_sum: usize = 0;
    for i in 0..obstructions.len() {
        for j in 0..obstructions[i].len() {
            if !obstructions[i][j] {
                // println!("{}, {}", j, i);

                obstructions[i][j] = true;
                (is_loop, _) = run_path(&obstructions, position, travel_dir);
                obstructions[i][j] = false;
                if is_loop {
                    loop_sum += 1
                }
            }
        }
    }
    println!("The amount of possible loops is: {loop_sum}");
}

fn read_file() -> ([isize; 2], usize, Vec<Vec<bool>>) {
    let mut obstructions: Vec<Vec<bool>> = Vec::new();
    let mut travel_dir: usize = 4;
    let mut position: [isize; 2] = [0, 0];
    for (j, line) in read_to_string("input.txt").unwrap().lines().enumerate() {
        let mut line_arr: Vec<bool> = Vec::new();
        for (i, char) in line.chars().enumerate() {
            if char == '#' {
                line_arr.push(true);
                continue;
            } else if char == '^' {
                travel_dir = 0;
                position = [j as isize, i as isize];
            } else if char == '>' {
                travel_dir = 1;
                position = [j as isize, i as isize];
            } else if char == 'v' {
                travel_dir = 2;
                position = [j as isize, i as isize];
            } else if char == '<' {
                travel_dir = 3;
                position = [j as isize, i as isize];
            }
            line_arr.push(false);
        }
        obstructions.push(line_arr);
    }

    if travel_dir == 4 {
        panic!("Error! No direction set");
    }

    return (position, travel_dir, obstructions)
}

fn run_path(obstructions: &Vec<Vec<bool>>, start_position: [isize; 2], start_travel_dir: usize) -> (bool, Vec<Vec<bool>>){
    let dirs: [[isize; 2];4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    let mut travel_dir = start_travel_dir;
    let mut position = start_position;

    // Construct travelled_points and travelled_dir vector
    let mut travelled_points: Vec<Vec<bool>> = Vec::new();
    let mut travelled_dir: Vec<Vec<u8>> = Vec::new();
    let mut travelled_points_w: Vec<bool> = Vec::new();
    let mut travelled_dir_w: Vec<u8> = Vec::new();
    for _ in 0..obstructions[0].len() {
        travelled_points_w.push(false);
        travelled_dir_w.push(0);
    }
    for _ in 0..obstructions.len() {
        travelled_points.push(travelled_points_w.clone());
        travelled_dir.push(travelled_dir_w.clone());
    }

    let mut next_pos: [usize; 2] = [0, 0];
    loop {
        travelled_points[position[0] as usize][position[1] as usize] = true;
        travelled_dir[position[0] as usize][position[1] as usize] = travelled_dir[position[0] as usize][position[1] as usize] | (1 << travel_dir);

        // If out of bounds (negative index), we've gone off the map
        next_pos[0] = match usize::try_from(position[0] + dirs[travel_dir][0]) {Err(_) => return (false, travelled_points), Ok(val) => val};
        next_pos[1] = match usize::try_from(position[1] + dirs[travel_dir][1]) {Err(_) => return (false, travelled_points), Ok(val) => val};
        
        // println!("{}, {}", next_pos[1], next_pos[0]);

        // If out of bounds (idx larger than obstruction map), break from the loop
        if obstructions.get(next_pos[0]) == None || 
           obstructions.get(next_pos[0]).expect("Error out of bounds").get(next_pos[1]) == None {
            return (false, travelled_points);
        }

        // We're back in a place and direction we've already been, we're in a loop.
        // (since we're only tracking the last direction we've travelled over a place, I think there's an edge case
        // where a loop isn't detected, but that probably won't be an issue)
        // In case it does: should use 4 seperate bits to denote travelled directions instead of a 0-3 index.
        if travelled_points[next_pos[0]][next_pos[1]] && (travelled_dir[next_pos[0]][next_pos[1]] & (1 << travel_dir)) != 0 {
            return (true, travelled_points);
        }

        // We've come across an obstruction, turn right.
        if obstructions.get(next_pos[0]).expect("Error out of bounds").get(next_pos[1]) == Some(&true) {
            travel_dir += 1;
            if travel_dir >= dirs.len() {
                travel_dir = 0;
            }
            continue;
        }

        position = [next_pos[0] as isize, next_pos[1] as isize];
    }
}
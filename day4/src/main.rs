use std::fs::read_to_string;

const search_arr: [char; 4] = ['X', 'M', 'A', 'S'];

fn main() {
    let mut file_vec: Vec<Vec<char>> = Vec::new();
    let mut matches: usize = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        file_vec.push(line.trim().chars().collect());
    }
    
    for j in 0..file_vec.len() {
        for i in 0..file_vec[j].len() {
            if file_vec[j][i] == search_arr[0] {
                matches += search(i as isize, j as isize, &file_vec);
            }
        }
    }
    println!("{matches}");
}

fn search(x: isize, y: isize, arr: &Vec<Vec<char>>) -> usize {
    let dirs: [[isize;2]; 8] = [[-1, -1], [-1, 0], [-1, 1], [0, 1], [0, -1], [1, -1], [1, 0], [1, 1]]; // [y, x]
    let mut matches: usize = 0;
    for dir in dirs {
        if search_dir(x, y, arr, dir[0], dir[1]) {
            matches += 1;
        }
    }
    return matches;
}

fn search_dir(x: isize, y: isize, arr: &Vec<Vec<char>>, i: isize, j: isize) -> bool {
    let mut cur_x = x;
    let mut cur_y = y;

    // Skip checking for first character since we already know its correct.
    for char in &search_arr[1..] {
        cur_x += i;
        cur_y += j;

        if cur_x < 0 {
            return false;
        }
        if cur_y < 0 {
            return false;
        }
        if cur_y >= arr.len() as isize {
            return false
        }
        if cur_x >= arr[cur_y as usize].len() as isize {
            return false;
        }
        if arr[cur_y as usize][cur_x as usize] != *char {
            return false;
        }
    }
    return true;
}
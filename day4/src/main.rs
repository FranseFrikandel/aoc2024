use std::fs::read_to_string;

fn main() {
    let mut file_vec: Vec<Vec<char>> = Vec::new();
    let mut matches: usize = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        file_vec.push(line.trim().chars().collect());
    }
    
    // We don't need to search the edges, since there can't be an X shape there.
    for j in 1..file_vec.len()-1 {
        for i in 1..file_vec[j].len()-1 {
            if file_vec[j][i] == 'A' {
                matches += search(i as isize, j as isize, &file_vec);
            }
        }
    }
    println!("{matches}");
}

fn search(x: isize, y: isize, arr: &Vec<Vec<char>>) -> usize {
    let dirs: [[isize;2]; 4] = [[-1, -1], [-1, 1], [1, -1], [1, 1]]; // [y, x]
    let mut matches: usize = 0;
    for dir in dirs {
        if arr[(y+dir[0]) as usize][(x+dir[1]) as usize] == 'M' && arr[(y-dir[0]) as usize][(x-dir[1]) as usize] == 'S' {
            matches += 1;
        }
    }
    if matches == 2 {
        return 1
    }
    return 0
}

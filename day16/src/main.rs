use std::time::Instant;
use std::fs::read_to_string;
use std::vec::Vec;
use std::collections::{VecDeque, HashMap, HashSet};

fn main() {
    let timer = Instant::now();
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut start_pos: [usize; 2] = [0, 0];

    // Position, dir, cost
    // 0 = up, 1 = right, 2 = down, 3 = left
    let mut paths: VecDeque<([usize; 2], u8, usize, Vec<[usize; 2]>)> = VecDeque::new();
    let mut final_paths: Vec<([usize; 2], u8, usize, Vec<[usize; 2]>)> = Vec::new();
    let mut min_score: Option<usize> = None;
    let mut visited: HashMap<usize, usize> = HashMap::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        // Add to the map
        // 0 = free, 1 = wall, 2 = start, 3 = end
        let mut map_line: Vec<u8> = Vec::new();
        for (i, ch) in line.chars().enumerate() {
            match ch {
                '.' => map_line.push(0),
                '#' => map_line.push(1),
                'E' => map_line.push(2),
                'S' => {map_line.push(3); start_pos = [map.len(), i];},
                _ => todo!()
            }
        }
        map.push(map_line);
    }

    for i in 0..3 {
        let to_pos = get_pos_w_dir(start_pos, i);
        if map[to_pos[0]][to_pos[1]] == 0 {
            if i == 1 {
                paths.push_back((to_pos, i, 1, Vec::from([to_pos])));
            } else {
                paths.push_back((to_pos, i, 1001, Vec::from([to_pos])));
            }
        }
    }

    while let Some(cur_path) = paths.pop_front() {
        if let Some(score) = min_score {
            // Cull path if the best path so far is better
            if cur_path.2 > score {
                continue;
            }
        }

        let dirs: [u8; 3];
        if cur_path.1 == 0 {
            dirs = [3, 0, 1];
        } else if cur_path.1 == 3 {
            dirs = [2, 3, 0];
        } else {
            dirs = [cur_path.1 - 1, cur_path.1, cur_path.1 + 1];
        }

        for dir in dirs {
            let pos = get_pos_w_dir(cur_path.0, dir);
            let key = pos_to_key(pos, dir);
            let cost;
            if cur_path.1 == dir {
                cost = cur_path.2 + 1;
            } else {
                cost = cur_path.2 + 1001;
            }

            // Already visited the node with lower cost
            if visited.get(&key).unwrap_or(&usize::MAX) < &cost {
                continue;
            }

            if map[pos[0]][pos[1]] == 0 {
                let mut pos_hist = cur_path.3.clone();
                pos_hist.push(pos);
                paths.push_back((pos, dir, cost, pos_hist));
                visited.insert(key, cost);
            } else if map[pos[0]][pos[1]] == 2 {
                if min_score == None || min_score >= Some(cost) {
                    let pos_hist = cur_path.3.clone();
                    min_score = Some(cost);
                    final_paths.push((pos, dir, cost, pos_hist));
                }
            }
        }

        paths.make_contiguous().sort_by(|a, b| a.2.cmp(&b.2));
    }

    let mut travelled_points = HashSet::new();
    for path in final_paths {
        if path.2 != min_score.expect("") {
            continue;
        }
        for point in path.3 {
            travelled_points.insert(point);
        }
    }

    if let Some(score) = min_score {
        println!("Minimum score: {}", score);
    }
    // Add 2 to add start and end point
    println!("Total travelled places: {}", travelled_points.len() + 2);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn get_pos_w_dir(start: [usize; 2], dir: u8) -> [usize; 2] {
    match dir {
        0 => return [start[0]-1, start[1]],
        1 => return [start[0], start[1]+1],
        2 => return [start[0]+1, start[1]],
        3 => return [start[0], start[1]-1],
        4_u8..=u8::MAX => panic!()
    }
}

fn pos_to_key(pos: [usize; 2], dir:u8) -> usize {
    return (pos[0] << 32) + (pos[1] << 4) + (dir as usize);
}

fn print_map(map: &Vec<Vec<u8>>) {
    for line in map {
        for item in line {
            match *item {
                0 => print!("."),
                1 => print!("#"),
                2 => print!("E"),
                3 => print!("S"),
                4 => print!("*"),
                5_u8..=u8::MAX => todo!()
            }
        }
        print!("\n");
    }
    print!("\n");
}
use std::time::Instant;
use std::fs::read_to_string;
use std::vec::Vec;
use std::collections::{VecDeque, HashMap};

fn main() {
    let timer = Instant::now();
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut start_pos: [usize; 2] = [0, 0];
    let mut end_pos: [usize; 2] = [0, 0];

    for line in read_to_string("input.txt").unwrap().lines() {
        // Add to the map
        // 0 = free, 1 = wall, 2 = end, 3 = start
        let mut map_line: Vec<u8> = Vec::new();
        for (i, ch) in line.chars().enumerate() {
            match ch {
                '.' => map_line.push(0),
                '#' => map_line.push(1),
                'E' => {map_line.push(2); end_pos = [map.len(), i];},
                'S' => {map_line.push(3); start_pos = [map.len(), i];},
                _ => todo!()
            }
        }
        map.push(map_line);
    }

    let init_path = find_path(&map, start_pos, end_pos).unwrap();
    let mut skip_lengths: HashMap<usize, usize> = Default::default();

    for j in 1..map.len()-1 {
        for i in 1..map[0].len()-1 {
            if map[i][j] != 1 {
                continue;
            }
            let mut map_w_skip = map.clone();
            map_w_skip[i][j] = 0;
            let new_path = match find_path(&map_w_skip, start_pos, end_pos) {
                Some(val) => val,
                None => {
                    print_map(&map_w_skip);
                    panic!();
                }
            };
            let skip_dist = init_path.1 - new_path.1;
            if skip_dist > 0 {
                match skip_lengths.get_mut(&skip_dist) {
                    Some(val) => *val += 1,
                    None => {skip_lengths.insert(skip_dist, 1);}
                }
            }
        }
    }

    let mut p1_sum = 0;
    for (key, value) in skip_lengths {
        if key >= 100 {
            p1_sum += value;
        }
    }

    // println!("Skips: {:?}", skip_lengths);
    println!("Amount of skips saving 100ps or more: {}", p1_sum);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn find_path(map: &Vec<Vec<u8>>, start_pos: [usize; 2], end_pos: [usize; 2]) -> Option<([usize; 2], usize, Vec<[usize; 2]>)>{
    let mut paths: VecDeque<([usize; 2], usize, Vec<[usize; 2]>)> = VecDeque::new();
    let mut visited: HashMap<usize, usize> = HashMap::new();
    let mut min_score: Option<usize> = None;
    let mut final_path: Option<([usize; 2], usize, Vec<[usize; 2]>)> = None;

    paths.push_back((start_pos, 0, Vec::new()));

    while let Some(cur_path) = paths.pop_front() {
        if let Some(score) = min_score {
            // Cull path if the best path so far is better
            if cur_path.1 > score {
                continue;
            }
        }

        for dir in 0..4 {
            let pos = match get_pos_w_dir(cur_path.0, dir) {
                Some(pos) => pos,
                None => continue,
            };
            let key = (pos[0] << 32) + pos[1];
            let cost = cur_path.1 + 1;

            // Already visited the node with lower cost
            if visited.get(&key).unwrap_or(&usize::MAX) <= &cost {
                continue;
            }

            if map[pos[0]][pos[1]] == 0 {
                let mut pos_hist = cur_path.2.clone();
                pos_hist.push(pos);
                paths.push_back((pos, cost, pos_hist));
                visited.insert(key, cost);
            } else if map[pos[0]][pos[1]] == 2 {
                if min_score == None || min_score > Some(cost) {
                    let pos_hist = cur_path.2.clone();
                    min_score = Some(cost);
                    final_path = Some((pos, cost, pos_hist));
                    // let mut map_copy = map.clone();
                    // for point in cur_path.2.iter() {
                    //     map_copy[point[0]][point[1]] = 2;
                    // }
                    // print_map(&map_copy);
                }
            }
        }

        paths.make_contiguous().sort_by(|a, b| {
            // let a_dist = (end_pos[0] as isize - a.0[0] as isize).abs() + (end_pos[1] as isize - a.0[1] as isize).abs();
            // let b_dist = (end_pos[0] as isize - b.0[0] as isize).abs() + (end_pos[1] as isize - b.0[1] as isize).abs();
            let score_a = a.1;
            let score_b = b.1;
            return score_a.cmp(&score_b)
        });
    }

    return final_path;
}

fn get_pos_w_dir(start: [usize; 2], dir: u8) -> Option<[usize; 2]> {
    let x: Option<usize>;
    let y: Option<usize>;
    match dir {
        0 => {
            y = start[0].checked_sub(1);
            x = Some(start[1]);
        },
        1 => {
            y = Some(start[0]);
            x = Some(start[1] + 1);
        },
        2 => {
            y = Some(start[0] + 1);
            x = Some(start[1]);
        },
        3 => {
            y = Some(start[0]);
            x = start[1].checked_sub(1);
        },
        4_u8..=u8::MAX => panic!()
    }
    if (x == None) || (y == None) {
        return None;
    } else {
        return Some([y?, x?]);
    }
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
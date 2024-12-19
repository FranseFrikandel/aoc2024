use std::time::Instant;
use std::fs::read_to_string;
use std::vec::Vec;
use std::collections::VecDeque;

fn main() {
    let timer = Instant::now();
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut start_pos: [usize; 2] = [0, 0];

    // Position, dir, cost
    // 0 = up, 1 = right, 2 = down, 3 = left
    let mut paths: VecDeque<([usize; 2], u8, usize, Vec<[usize; 2]>)> = VecDeque::new();
    let mut final_path: Option<([usize; 2], u8, usize, Vec<[usize; 2]>)> = None;

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
        if let Some(final_p) = &final_path {
            // Cull path if the best path so far is better
            if cur_path.2 > final_p.2 {
                // println!("Culling path");
                // let mut final_route_map = map.clone();
                // for point in cur_path.3.iter() {
                //     final_route_map[point[0]][point[1]] = 4
                // }
                // print_map(&final_route_map);
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
            if cur_path.3.contains(&pos) {
                // Loop
                continue;
            }
            if map[pos[0]][pos[1]] == 0 {
                let mut pos_hist = cur_path.3.clone();
                pos_hist.push(pos);
                if cur_path.1 == dir {
                    paths.push_back((pos, dir, cur_path.2 + 1, pos_hist));
                } else {
                    paths.push_back((pos, dir, cur_path.2 + 1001, pos_hist));
                }
            } else if map[pos[0]][pos[1]] == 2 {
                if cur_path.1 == dir {
                    if final_path == None || final_path.clone().unwrap().2 > cur_path.2 + 1 {
                        final_path = Some((pos, dir, cur_path.2 + 1, cur_path.3.clone()));
                    }
                } else {
                    if final_path == None || final_path.clone().unwrap().2 > cur_path.2 + 1001 {
                        final_path = Some((pos, dir, cur_path.2 + 1001, cur_path.3.clone()));
                    }
                }
            }
        }
    }

    if let Some(final_p) = &final_path {
        let mut final_route_map = map.clone();
        for point in final_p.3.iter() {
            final_route_map[point[0]][point[1]] = 4
        }
        print_map(&final_route_map);
        println!("Total cost: {}", final_p.2);
    }
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
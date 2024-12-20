use std::time::Instant;
use std::fs::read_to_string;
use std::collections::{VecDeque, HashMap};

const SIZE: usize = 71;

fn main() {
    let timer = Instant::now();
    let mut map: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    let filestr = read_to_string("input.txt").unwrap();
    let mut lines = filestr.lines();
    let mut i = 0;
    for _ in 0..12 {
        let line = lines.next().unwrap();
        i += 1;
        let mut spl = line.split(",");
        let x: usize = spl.next().unwrap().parse().expect("Failed to parse x-coordinate");
        let y: usize = spl.next().unwrap().parse().expect("Failed to parse y-coordinate");
        map[y][x] = 1;
    }

    let mut first_run = true;
    loop {
        let result = match find_path(&map) {
            Some(res) => res,
            None => break
        };
        if first_run {
            println!("P1 result: {}", result.1);
            first_run = false;
        }
        loop {
            let line = lines.next().unwrap();
            i += 1;
            let mut spl = line.split(",");
            let x: usize = spl.next().unwrap().parse().expect("Failed to parse x-coordinate");
            let y: usize = spl.next().unwrap().parse().expect("Failed to parse y-coordinate");
            map[y][x] = 1;
            if result.2.contains(&[y, x]) {
                break;
            }
        }
    }
    println!("No more path after {} bytes, blocking {:?}", i, filestr.lines().nth(i-1).unwrap());
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn find_path(map: &[[u8; SIZE]; SIZE]) -> Option<([usize; 2], usize, Vec<[usize; 2]>)>{
    let mut paths: VecDeque<([usize; 2], usize, Vec<[usize; 2]>)> = VecDeque::new();
    let mut visited: HashMap<usize, usize> = HashMap::new();
    let mut min_score: Option<usize> = None;
    let mut final_path: Option<([usize; 2], usize, Vec<[usize; 2]>)> = None;

    paths.push_back(([0, 0], 0, Vec::new()));

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

            if pos == [SIZE-1, SIZE-1] {
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
            } else if map[pos[0]][pos[1]] == 0 {
                let mut pos_hist = cur_path.2.clone();
                pos_hist.push(pos);
                paths.push_back((pos, cost, pos_hist));
                visited.insert(key, cost);
            }
        }

        paths.make_contiguous().sort_by(|a, b| {
            let score_a = a.1 + (SIZE - a.0[0]) +  (SIZE - a.0[1]);
            let score_b = b.1 + (SIZE - b.0[0]) +  (SIZE - b.0[1]);
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
    } else if (x >= Some(SIZE)) || (y >= Some(SIZE)) {
        return None;
    } else {
        return Some([y?, x?]);
    }
}

fn print_map(map: &[[u8; SIZE]; SIZE]) {
    for line in map {
        for item in line {
            match *item {
                0 => print!("."),
                1 => print!("#"),
                2 => print!("O"),
                3_u8..=u8::MAX => todo!()
            }
        }
        print!("\n");
    }
    print!("\n");
}
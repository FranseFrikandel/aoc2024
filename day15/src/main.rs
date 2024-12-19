use std::time::Instant;
use std::fs::read_to_string;
use std::vec::Vec;

fn main() {
    let timer = Instant::now();
    let mut movements: Vec<u8> = Vec::new();
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut robot_pos: [usize; 2] = [0, 0];

    // Toggle for p2
    let double_width = true;

    for line in read_to_string("input.txt").unwrap().lines() {
        if line.len() < 1 {
            continue;
        }
        if line.chars().next().unwrap() == '#' {
            // Add to the map
            // 0 = free, 1 = wall, 2 = box, 3 = robot, 4 = left of double box, 5 = right of double box.
            let mut map_line: Vec<u8> = Vec::new();
            for (i, ch) in line.chars().enumerate() {
                if double_width {
                    match ch {
                        '.' => {map_line.push(0); map_line.push(0)},
                        '#' => {map_line.push(1); map_line.push(1)},
                        'O' => {map_line.push(4); map_line.push(5)},
                        '@' => {map_line.push(3); map_line.push(0); robot_pos = [map.len(), i*2];},
                        _ => todo!()
                    }
                } else {
                    match ch {
                        '.' => map_line.push(0),
                        '#' => map_line.push(1),
                        'O' => map_line.push(2),
                        '@' => {map_line.push(3); robot_pos = [map.len(), i];},
                        _ => todo!()
                    }
                }
            }
            map.push(map_line);
        } else {
            // Add to planned movements
            // 0 = up, 1 = right, 2 = down, 3 = left
            for ch in line.chars() {
                match ch {
                    '^' => movements.push(0),
                    '>' => movements.push(1),
                    'v' => movements.push(2),
                    '<' => movements.push(3),
                    _ => todo!()
                }
            }
        }
    }
    
    // print_map(&map);
    for movement in movements {
        // Since with double width setup, sometimes boxes are moved even though it turns out to be impossible,
        // so we keep a copy.
        let mut map_copy = map.clone();
        if move_item(&mut map_copy, robot_pos, movement, true) {
            // println!("Moved");
            match movement {
                0 => robot_pos = [robot_pos[0]-1, robot_pos[1]],
                1 => robot_pos = [robot_pos[0], robot_pos[1]+1],
                2 => robot_pos = [robot_pos[0]+1, robot_pos[1]],
                3 => robot_pos = [robot_pos[0], robot_pos[1]-1],
                4_u8..=u8::MAX => todo!()
            }
            map = map_copy;
        }
        // print_map(&map)
    }
    print_map(&map);
    println!("Total cost: {}", calculate_score(&map));
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn move_item(map: &mut Vec<Vec<u8>>, pos: [usize; 2], direction: u8, check_wide_box: bool) -> bool {
    if map[pos[0]][pos[1]] == 0 {
        // println!("Air at {:?}", pos);
        return true;
    }
    if map[pos[0]][pos[1]] == 1 {
        // println!("Wall at {:?}", pos);
        return false;
    }
    if check_wide_box && (direction == 0 || direction == 2) {
        // Move the other half of the box too
        if map[pos[0]][pos[1]] == 4 {
            // println!("Also push right side");
            if !move_item(map, [pos[0], pos[1]+1], direction, false) {
                return false
            }
        }
        if map[pos[0]][pos[1]] == 5 {
            // println!("Also push left side");
            if !move_item(map, [pos[0], pos[1]-1], direction, false) {
                return false
            }
        }
    }
    let new_pos: [usize; 2];
    match direction {
        0 => new_pos = [pos[0]-1, pos[1]],
        1 => new_pos = [pos[0], pos[1]+1],
        2 => new_pos = [pos[0]+1, pos[1]],
        3 => new_pos = [pos[0], pos[1]-1],
        4_u8..=u8::MAX => todo!()
    }
    if move_item(map, new_pos, direction, true) {
        let cur_item = map[pos[0]][pos[1]];
        map[pos[0]][pos[1]] = 0;
        map[new_pos[0]][new_pos[1]] = cur_item;
        return true;
    } else {
        return false;
    }
}

fn print_map(map: &Vec<Vec<u8>>) {
    for line in map {
        for item in line {
            match *item {
                0 => print!("."),
                1 => print!("#"),
                2 => print!("O"),
                3 => print!("@"),
                4 => print!("["),
                5 => print!("]"),
                6_u8..=u8::MAX => todo!()
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn calculate_score(map: &Vec<Vec<u8>>) -> usize {
    let mut score = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, point) in line.iter().enumerate() {
            if *point == 2 {
                score += (y*100)+x;
            }
            if *point == 4 {
                score += (y*100)+x;
            }
        }
    }
    return score;
}
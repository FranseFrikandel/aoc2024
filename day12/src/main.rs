use std::time::Instant;
use std::fs::read_to_string;

fn main() {
    let timer = Instant::now();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in read_to_string("input.txt").unwrap().lines() {
        let mut line_vec: Vec<char> = Vec::new();
        for plant in line.chars() {
            line_vec.push(plant);
        }
        map.push(line_vec);
    }
    // Region: Plant, Coordinates
    let original_map = map.clone();
    let mut regions: Vec<(char, Vec<[usize; 2]>)> = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let character = map[i][j];
            if character != '-' {
                let region = gather_region(i, j, character, &mut map);
                regions.push((character, region));
            }
        }
    }

    // Calculate the border and area for each region
    let mut p1_sum = 0;
    let mut p2_sum = 0;
    for region in regions {
        let area = region.1.len();
        let mut border = 0;
        // Border pieces sorted by coordinate and side
        let mut border_pieces: Vec<[usize; 3]> = Vec::new();
        for point in &region.1 {
            match get_2d(point[0], point[1]+1, &original_map) {
                Some(plant) => if plant != region.0 {
                    border += 1;
                    border_pieces.push([point[0], point[1], 0]);
                }
                None => {
                    border += 1;
                    border_pieces.push([point[0], point[1], 0]);
                }
            }
            match get_2d(point[0]+1, point[1], &original_map) {
                Some(plant) => if plant != region.0 {
                    border += 1;
                    border_pieces.push([point[0], point[1], 1]);
                }
                None => {
                    border += 1;
                    border_pieces.push([point[0], point[1], 1]);
                }
            }
            if point[1] > 0 {
                match get_2d(point[0], point[1]-1, &original_map) {
                    Some(plant) => if plant != region.0 {
                        border += 1;
                        border_pieces.push([point[0], point[1], 2]);
                    }
                    None => {
                        border += 1;
                        border_pieces.push([point[0], point[1], 2]);
                    }
                }
            } else {
                border += 1;
                border_pieces.push([point[0], point[1], 2]);
            }
            if point[0] > 0 {
                match get_2d(point[0]-1, point[1], &original_map) {
                    Some(plant) => if plant != region.0 {
                        border += 1;
                        border_pieces.push([point[0], point[1], 3]);
                    }
                    None => {
                        border += 1;
                        border_pieces.push([point[0], point[1], 3]);
                    }
                }
            } else {
                border += 1;
                border_pieces.push([point[0], point[1], 3]);
            }
        }

        let border_pieces_cpy = border_pieces.clone();
        for piece in border_pieces_cpy {
            // If we've already removed this piece from the non-copied array, we've already added this piece as a perimeter.
            if !border_pieces.contains(&piece) {continue}
            if piece[2] == 1 || piece[2] == 3 {
                // Oriented along the X-axis (2nd index)
                let mut i = 1;
                loop {
                    let Some(pos) = border_pieces.iter().position(|&p| p == [piece[0], piece[1]+i, piece[2]]) else {break};
                    border_pieces.remove(pos);
                    i += 1;
                }
                i = 1;
                loop {
                    if i > piece[1] {break}
                    let Some(pos) = border_pieces.iter().position(|&p| p == [piece[0], piece[1]-i, piece[2]]) else {break};
                    border_pieces.remove(pos);
                    i += 1;
                }
            } else {
                // Oriented along the Y-axis (1st index)
                let mut i = 1;
                loop{
                    let Some(pos) = border_pieces.iter().position(|&p| p == [piece[0]+i, piece[1], piece[2]]) else {break};
                    border_pieces.remove(pos);
                    i += 1;
                }
                i = 1;
                loop {
                    if i > piece[0] {break}
                    let Some(pos) = border_pieces.iter().position(|&p| p == [piece[0]-i, piece[1], piece[2]]) else {break};
                    border_pieces.remove(pos);
                    i += 1;
                }
            }
        }
        println!("{:?}", border_pieces);
        println!("{}: {} * {}", region.0, area, border_pieces.len());
        p1_sum += area*border;
        p2_sum += area*border_pieces.len();
    }

    println!("Total part 1 cost: {}", p1_sum);
    println!("Total part 2 cost: {}", p2_sum);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn gather_region(y: usize, x: usize, plant_type: char, map: &mut Vec<Vec<char>>) -> Vec<[usize; 2]> {
    let mut final_coords: Vec<[usize; 2]> = Vec::new();
    map[y][x] = '-';
    final_coords.push([y, x]);
    match get_2d(y, x+1, map) {
        Some(plant) => 
        if plant == plant_type {
            map[y][x+1] = '-';
            let result = gather_region(y, x+1, plant_type, map);
            for res in result {
                final_coords.push(res);
            }
        }
        None => {}
    }
    match get_2d(y+1, x, map) {
        Some(plant) => 
        if plant == plant_type {
            map[y+1][x] = '-';
            let result = gather_region(y+1, x, plant_type, map);
            for res in result {
                final_coords.push(res);
            }
        }
        None => {}
    }
    if x > 0 {
        match get_2d(y, x-1, map) {
            Some(plant) => 
            if plant == plant_type {
                map[y][x-1] = '-';
                let result = gather_region(y, x-1, plant_type, map);
                for res in result {
                    final_coords.push(res);
                }
            }
            None => {}
        }
    }
    if y > 0 {
        match get_2d(y-1, x, map) {
            Some(plant) => 
            if plant == plant_type {
                map[y-1][x] = '-';
                let result = gather_region(y-1, x, plant_type, map);
                for res in result {
                    final_coords.push(res);
                }
            }
            None => {}
        }
    }
    return final_coords;
}

fn get_2d(y: usize, x:usize, vector: &Vec<Vec<char>>) -> Option<char> {
    match vector.get(y) {
        Some(vector2) => {
            match vector2.get(x) {
                Some(character) => return Some(*character),
                None => return None
            }
        }
        None => return None
    }
}
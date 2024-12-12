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
    let mut sum = 0;
    for region in regions {
        let area = region.1.len();
        let mut border = 0;
        for point in &region.1 {
            match get_2d(point[0], point[1]+1, &original_map) {
                Some(plant) => if plant != region.0 {border += 1}
                None => {border += 1}
            }
            match get_2d(point[0]+1, point[1], &original_map) {
                Some(plant) => if plant != region.0 {border += 1}
                None => {border += 1}
            }
            if point[1] > 0 {
                match get_2d(point[0], point[1]-1, &original_map) {
                    Some(plant) => if plant != region.0 {border += 1}
                    None => {border += 1}
                }
            } else {
                border += 1;
            }
            if point[0] > 0 {
                match get_2d(point[0]-1, point[1], &original_map) {
                    Some(plant) => if plant != region.0 {border += 1}
                    None => {border += 1}
                }
            } else {
                border += 1;
            }
        }
        // println!("{}: {:?}", region.0, region.1);
        // println!("{}: {} * {}", region.0, area, border);
        sum += area*border;
    }

    println!("Total cost: {}", sum);
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
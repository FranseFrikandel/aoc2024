use std::time::Instant;
use std::fs::read_to_string;

fn main() {
    let timer = Instant::now();

    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in read_to_string("input.txt").unwrap().lines() {
        let mut line_vec: Vec<u8> = Vec::new();
        for ch in line.chars() {
            line_vec.push(ch.to_digit(10).expect("Failed to parse to integer") as u8);
        }
        map.push(line_vec);
    }

    let mut sum_points: usize = 0;
    let mut sum_rated: usize = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                let mut final_points = search_points(y, x, 0, &map);
                final_points.sort();
                final_points.dedup();
                sum_points += final_points.len();
                sum_rated += search_rated(y, x, 0, &map);
            }
        }
    }

    println!("Total part 1 sum is: {}", sum_points);
    println!("Total part 2 sum is: {}", sum_rated);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn search_points(y: usize, x: usize, cur_height: u8, map: &Vec<Vec<u8>>) -> Vec<[usize; 2]> {
    if cur_height == 9 {
        return vec![[y, x]];
    }

    let mut final_points: Vec<[usize; 2]> = Vec::new();

    match get_2d(y, x+1, &map) {
        Some(height) => if height == (cur_height + 1) {
            let result = search_points(y, x+1, cur_height + 1, &map);
            for res in result {
                final_points.push(res);
            }
        }
        None => {}
    }
    match get_2d(y+1, x, &map) {
        Some(height) => if height == (cur_height + 1) {
            let result = search_points(y+1, x, cur_height + 1, &map);
            for res in result {
                final_points.push(res);
            }
        }
        None => {}
    }
    if x > 0 {
        match get_2d(y, x-1, &map) {
            Some(height) => if height == (cur_height + 1) {
                let result = search_points(y, x-1, cur_height + 1, &map);
                for res in result {
                    final_points.push(res);
                }
            }
            None => {}
        }
    }
    if y > 0 {
        match get_2d(y-1, x, &map) {
            Some(height) => if height == (cur_height + 1) {
                let result = search_points(y-1, x, cur_height + 1, &map);
                for res in result {
                    final_points.push(res);
                }
            }
            None => {}
        }
    }
    return final_points;
}

fn search_rated(y: usize, x: usize, cur_height: u8, map: &Vec<Vec<u8>>) -> usize {
    if cur_height == 9 {
        return 1;
    }

    let mut rating = 0;

    match get_2d(y, x+1, &map) {
        Some(height) => if height == (cur_height + 1) {
            rating += search_rated(y, x+1, cur_height + 1, &map);
        }
        None => {}
    }
    match get_2d(y+1, x, &map) {
        Some(height) => if height == (cur_height + 1) {
            rating += search_rated(y+1, x, cur_height + 1, &map);
        }
        None => {}
    }
    if x > 0 {
        match get_2d(y, x-1, &map) {
            Some(height) => if height == (cur_height + 1) {
                rating += search_rated(y, x-1, cur_height + 1, &map);
            }
            None => {}
        }
    }
    if y > 0 {
        match get_2d(y-1, x, &map) {
            Some(height) => if height == (cur_height + 1) {
                rating += search_rated(y-1, x, cur_height + 1, &map);
            }
            None => {}
        }
    }
    return rating;
}

fn get_2d(y: usize, x:usize, vector: &Vec<Vec<u8>>) -> Option<u8> {
    match vector.get(y) {
        Some(vector2) => {
            match vector2.get(x) {
                Some(int) => return Some(*int),
                None => return None
            }
        }
        None => return None
    }
}
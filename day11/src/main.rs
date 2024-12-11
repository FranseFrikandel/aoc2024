use std::time::Instant;
use std::fs::read_to_string;
use intmap::IntMap;

fn main() {
    let timer = Instant::now();

    // Number on stone, amount of stones.
    let mut stones: IntMap<usize, usize> = IntMap::new();
    // Cache saves after how many iterations some initial stone splits into some other set of stones.
    // Cache data format: [iterations, a, b]
    for stone in read_to_string("input.txt").unwrap().split(" ") {
        let stone_int = stone.parse().expect("Failed to parse int");
        add_stones(&mut stones, stone_int, 1);
    }

    for i in 0..75 {
        let mut new_stones: IntMap<usize, usize> = IntMap::new();
        for (stone, amount) in stones.iter_mut() {
            if stone == 0 {
                add_stones(&mut new_stones, 1, *amount);
                continue;
            }

            let stone_str = stone.to_string();
            let length = stone_str.len();
            if length % 2 == 0 {
                let a = stone_str[0..length/2].parse().expect("Failed to parse int");
                let b = stone_str[length/2..length].parse().expect("Failed to parse int");
                add_stones(&mut new_stones, a, *amount);
                add_stones(&mut new_stones, b, *amount);
                continue;
            }

            add_stones(&mut new_stones, stone*2024, *amount);
        }
        // println!("{}", i);
        // println!("{}: {:?}", i, new_stones);
        stones = new_stones;
    }

    let mut sum = 0;
    for val in stones.values() {
        sum += val;
    }

    println!("Amount of stones is: {}", sum);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn add_stones(stonemap: &mut IntMap<usize, usize>, stone_id: usize, stone_amount: usize) {
    match stonemap.get_mut(stone_id) {
        Some(val) => {*val += stone_amount;},
        None => {stonemap.insert(stone_id, stone_amount);}
    }
}
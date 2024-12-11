use std::time::Instant;
use std::fs::read_to_string;
use std::collections::HashMap;
use intmap::IntMap;

fn main() {
    let timer = Instant::now();

    let mut stones: Vec<usize> = Vec::new();
    // Cache saves after how many iterations some initial stone splits into some other set of stones.
    // Cache data format: [iterations, a, b]
    let mut cache = IntMap::<usize, [usize; 3]>::default();
    // let mut cache = HashMap::<usize, [usize; 3]>::default();
    for stone in read_to_string("input.txt").unwrap().split(" ") {
        stones.push(stone.parse().expect("Failed to parse to int"));
    }

    let mut sum = 0;
    for stone in stones {
        sum += process_stone(stone, 26, &mut cache);
        println!("Processed a stone: {}", sum);
        println!("Runtime: {:.3?}", timer.elapsed());
    }
    println!("Amount of stones is: {}", sum);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn process_stone(init_stone: usize, init_iterations: usize, cache: &mut IntMap<usize, [usize; 3]>) -> usize {
    let mut stone = init_stone;
    let mut iterations = init_iterations;
    loop {
        iterations -= 1;
        match cache.get(stone) {
            Some(&cache_hit) => if cache_hit[0] < iterations {
                // We've found a longer chain, cache it too.
                if stone != init_stone {
                    cache.insert(init_stone, [init_iterations - iterations + cache_hit[0], cache_hit[1], cache_hit[2]]);
                }
                // println!("Cache hit");
                let sum_a = process_stone(cache_hit[1], iterations - cache_hit[0] + 1, cache);
                let sum_b = process_stone(cache_hit[2], iterations - cache_hit[0] + 1, cache);
                return sum_a + sum_b;
            },
            None => {}
        }
        if iterations == 0 {
            return 1;
        }
        if stone == 0 {
            stone = 1;
            continue;
        }
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            let a = stone_str[0..stone_str.len()/2].parse().expect("Failed to parse to int");
            let b = stone_str[stone_str.len()/2..stone_str.len()].parse().expect("Failed to parse to int");
            // Cache the result
            cache.insert(init_stone, [init_iterations - iterations, a, b]);
            let sum_a = process_stone(a, iterations, cache);
            let sum_b = process_stone(b, iterations, cache);
            return sum_a+sum_b;
        }
        stone = stone*2024;
    }
}
use std::time::Instant;
use std::fs::read_to_string;

fn main() {
    let timer = Instant::now();

    let mut diskmap: Vec<i32> = Vec::new();
    let mut is_file = true;
    let mut file_id: i32 = 0;
    // Read in the disk map
    for ch in read_to_string("input.txt").unwrap().chars() {
        let len = ch.to_digit(10).expect("Error, can't convert to int");
        if is_file {
            for _ in 0..len {
                diskmap.push(file_id);
            }
            file_id += 1;
        } else {
            for _ in 0..len {
                diskmap.push(-1);
            }
        }
        is_file = !is_file;
    }

    // Remove empty space off the end of the diskmap.
    while *diskmap.last().expect("Diskmap is empty") == -1 {
        diskmap.pop();
    }

    let mut i: usize = 0;

    while i < diskmap.len() {
        if diskmap[i] == -1 {
            // Edgecase: While popping empty space we could end up popping index i?
            while *diskmap.last().expect("Diskmap is empty") == -1 {
                diskmap.pop();
            }
            // If popping the empty space above removed ith element too, we're already at the end.
            if i >= diskmap.len() {break}
            diskmap[i] = diskmap.pop().expect("Diskmap is empty");
        }
        i += 1;
    }

    let mut sum: usize = 0;
    for (i, el) in diskmap.into_iter().enumerate() {
        sum += el as usize*i;
    }

    println!("Total sum is: {}", sum);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

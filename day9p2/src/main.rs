use std::time::Instant;
use std::fs::read_to_string;

fn main() {
    let timer = Instant::now();

    // Files are saved as [start, length]
    let mut files: Vec<[usize; 3]> = Vec::new();
    let mut freespaces: Vec<[usize; 2]> = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;
    let mut pos = 0;
    // Read in the disk map
    for ch in read_to_string("input.txt").unwrap().chars() {
        let len = ch.to_digit(10).expect("Error, can't convert to int") as usize;
        if is_file {
            files.push([pos, len, file_id]);
            file_id += 1;
        } else {
            freespaces.push([pos, len]);
        }
        is_file = !is_file;
        pos += len;
    }

    for file in files.iter_mut().rev() {
        for freespace in &mut freespaces {
            if file[0] < freespace[0] {
                // This freespace sits past the actual file itself, stop attempting to move
                break;
            } else if file[1] <= freespace[1] {
                file[0] = freespace[0];
                freespace[0] = freespace[0] + file[1];
                freespace[1] = freespace[1] - file[1];
            }
        }
    }

    let mut sum = 0;
    for file in files {
        for i in file[0]..file[0]+file[1] {
            sum += file[2] * i;
        }
    }

    println!("Total sum is: {}", sum);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

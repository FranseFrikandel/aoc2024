use std::iter::zip;
use std::fs::read_to_string;

fn main() {
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        for (i, value) in line.split("   ").enumerate() {
            if i==0 {
                a.push(value.parse().expect("Failed to read integer"));
            } else {
                b.push(value.parse().expect("Failed to read integer"));
            }
        }
    }

    a.sort();
    b.sort();

    let mut sum: i32 = 0;
    for (ela, elb) in zip(&a, &b) {
        sum += (ela-elb).abs();
    }
    println!("The total distance is: {sum}");

    let mut similarity: i32 = 0;
    for ela in &a {
        let mut hits = 0;
        for elb in &b {
            if ela == elb {
                hits += 1;
            } else if elb > ela {
                // Since a and b are sorted, we've already gone past the same results and b is now always bigger
                continue;
            }
        }
        similarity += ela * hits;
    }
    println!("The similarity score is: {similarity}");
}

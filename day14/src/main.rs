use std::fs::read_to_string;
use std::time::Instant;
use regex::Regex;


fn main() {
    let timer = Instant::now();
    let width: isize = 101;
    let height: isize = 103;
    let time: isize = 100;

    let half_width = width/2;
    let half_height = height/2;
    let reg = Regex::new(r"p=([-\d]+),([-\d]+) v=([-\d]+),([-\d]+)").unwrap();

    let mut robots: Vec<[isize;2]> = Vec::new();
    for line in read_to_string("input.txt").unwrap().lines() {
        let robot_reg = reg.captures(line).expect("Failed to parse regex");
        let x: isize = robot_reg[1].parse().expect("Failed to parse to int");
        let y: isize = robot_reg[2].parse().expect("Failed to parse to int");
        let vx: isize = robot_reg[3].parse().expect("Failed to parse to int");
        let vy: isize = robot_reg[4].parse().expect("Failed to parse to int");

        let mut final_x = (x + vx*time)%width;
        if final_x < 0 {final_x += width};
    
        let mut final_y = (y + vy*time)%height;
        if final_y < 0 {final_y += height };

        robots.push([final_x, final_y]);
    }
    let mut q1=0;
    let mut q2=0;
    let mut q3=0;
    let mut q4=0;
    for robot in robots {
        if robot[0] < half_width && robot[1] < half_height { q1 += 1; }
        if robot[0] > half_width && robot[1] < half_height { q2 += 1; }
        if robot[0] < half_width && robot[1] > half_height { q3 += 1; }
        if robot[0] > half_width && robot[1] > half_height { q4 += 1; }
    }
    println!("Result: {}", q1*q2*q3*q4);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

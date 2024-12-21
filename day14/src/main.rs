use std::fs::read_to_string;
use std::time::Instant;
use regex::Regex;
use std::io::stdin;

fn main() {
    let timer = Instant::now();
    let width: isize = 101;
    let height: isize = 103;
    let time: isize = 100;

    let half_width = width/2;
    let half_height = height/2;
    let reg = Regex::new(r"p=([-\d]+),([-\d]+) v=([-\d]+),([-\d]+)").unwrap();

    let mut robots: Vec<[isize;4]> = Vec::new();
    for line in read_to_string("input.txt").unwrap().lines() {
        let robot_reg = reg.captures(line).expect("Failed to parse regex");
        let x: isize = robot_reg[1].parse().expect("Failed to parse to int");
        let y: isize = robot_reg[2].parse().expect("Failed to parse to int");
        let vx: isize = robot_reg[3].parse().expect("Failed to parse to int");
        let vy: isize = robot_reg[4].parse().expect("Failed to parse to int");

        robots.push([x, y, vx, vy]);
    }

    let p1_result = get_positions(&robots, time, width, height);

    let mut q1=0;
    let mut q2=0;
    let mut q3=0;
    let mut q4=0;
    for robot in p1_result {
        if robot[0] < half_width && robot[1] < half_height { q1 += 1; }
        if robot[0] > half_width && robot[1] < half_height { q2 += 1; }
        if robot[0] < half_width && robot[1] > half_height { q3 += 1; }
        if robot[0] > half_width && robot[1] > half_height { q4 += 1; }
    }
    println!("Result: {}", q1*q2*q3*q4);
    println!("Total runtime: {:.3?}", timer.elapsed());

    let mut i = 0;
    let mut _l = String::new();
    loop {
        let mut img;
        loop {
            i += 1;
            let result = get_positions(&robots, i, width, height);
            img = generate_picture(&result, width, height);
            if img.contains("++++++++") {break};
        }
        println!("Input {}", i);
        println!("{}", img);
        stdin().read_line(&mut _l).unwrap();
        i += 1;
    }
}

fn get_positions(robots: &Vec<[isize; 4]>, time:isize, width: isize, height: isize) -> Vec<[isize; 2]> {
    let mut result: Vec<[isize; 2]> = Vec::new();
    for robot in robots {
        let mut final_x = (robot[0] + robot[2]*time)%width;
        if final_x < 0 {final_x += width};

        let mut final_y = (robot[1] + robot[3]*time)%height;
        if final_y < 0 {final_y += height };
        result.push([final_x, final_y]);
    }
    return result;
}

fn generate_picture(robots: &Vec<[isize;2]>, width: isize, height: isize) -> String {
    let w_buffer = vec![false; width as usize];
    let mut buffer = vec![w_buffer.clone(); height as usize];
    let mut res_str = String::new();

    for robot in robots {
        buffer[robot[1] as usize][robot[0] as usize] = true;
    }

    for buffer_line in buffer {
        let mut line_str = String::new();
        for var in buffer_line {
            if var {line_str.push('+')}
            else {line_str.push('.')}
        }
        res_str += &line_str;
        res_str += "\n";
    }
    return res_str;
}
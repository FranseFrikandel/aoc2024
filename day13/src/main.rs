use std::time::Instant;
use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let reg_button = Regex::new(r"X\+([\d]+), Y\+([\d]+)").unwrap();
    let reg_prize = Regex::new(r"X=([\d]+), Y=([\d]+)").unwrap();
    let timer = Instant::now();

    // A offset, B offset, price location
    let mut machines: Vec<[[isize;2]; 3]> = Vec::new();
    let filestr = read_to_string("input.txt").unwrap();
    let mut lines = filestr.lines();
    while let(Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let a_button_reg = reg_button.captures(line1).expect("Failed to parse regex");
        let b_button_reg = reg_button.captures(line2).expect("Failed to parse regex");
        let prize_reg = reg_prize.captures(line3).expect("Failed to parse regex");

        let a_button: [isize; 2] = [a_button_reg[1].parse().expect("Failed to parse int"),
                                    a_button_reg[2].parse().expect("Failed to parse int")];
        let b_button: [isize; 2] = [b_button_reg[1].parse().expect("Failed to parse int"),
                                    b_button_reg[2].parse().expect("Failed to parse int")];
        // let prize: [isize; 2] = [prize_reg[1].parse::<isize>().expect("Failed to parse int"),
        //                          prize_reg[2].parse::<isize>().expect("Failed to parse int")];
        let prize = [prize_reg[1].parse::<isize>().expect("Failed to parse int") + 10000000000000,
                     prize_reg[2].parse::<isize>().expect("Failed to parse int") + 10000000000000];

        machines.push([a_button, b_button, prize]);
        lines.next();
    }

    let mut p1_sum = 0;
    for machine in machines {
        let a = (machine[2][0]*machine[1][1] - machine[1][0]*machine[2][1]) / 
                (machine[0][0]*machine[1][1] - machine[1][0]*machine[0][1]);
        let a_remainder = (machine[2][0]*machine[1][1] - machine[1][0]*machine[2][1]) %
                          (machine[0][0]*machine[1][1] - machine[1][0]*machine[0][1]);
        if a_remainder != 0 {
            continue;
        }
        let b = (machine[2][1] - a*machine[0][1]) / machine[1][1];
        let b_remainder = (machine[2][1] - a*machine[0][1]) % machine[1][1];
        if b_remainder != 0 {
            continue;
        }

        // let x = a*machine[0][0] + b*machine[1][0];
        // let y = a*machine[0][1] + b*machine[1][1];

        // if x == machine[2][0] && y == machine[2][1] {
        p1_sum += get_cost(a, b);
        // }
    }

    println!("Total part 1 cost: {}", p1_sum);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn get_cost(a: isize, b:isize) -> isize {
    a*3 + b
}
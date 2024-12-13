use std::time::Instant;
use std::fs::read_to_string;
use regex::Regex;
use std::cmp;

fn main() {
    let reg_button = Regex::new(r"X\+([\d]+), Y\+([\d]+)").unwrap();
    let reg_prize = Regex::new(r"X=([\d]+), Y=([\d]+)").unwrap();
    let timer = Instant::now();

    // A offset, B offset, price location
    let mut machines: Vec<[[usize;2]; 3]> = Vec::new();
    let filestr = read_to_string("input.txt").unwrap();
    let mut lines = filestr.lines();
    while let(Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let a_button_reg = reg_button.captures(line1).expect("Failed to parse regex");
        let b_button_reg = reg_button.captures(line2).expect("Failed to parse regex");
        let prize_reg = reg_prize.captures(line3).expect("Failed to parse regex");

        let a_button = [a_button_reg[1].parse().expect("Failed to parse int"),
                        a_button_reg[2].parse().expect("Failed to parse int")];
        let b_button = [b_button_reg[1].parse().expect("Failed to parse int"),
                        b_button_reg[2].parse().expect("Failed to parse int")];
        let prize = [prize_reg[1].parse::<usize>().expect("Failed to parse int"),
                     prize_reg[2].parse::<usize>().expect("Failed to parse int")];
        // let prize = [prize_reg[1].parse::<usize>().expect("Failed to parse int") + 10000000000000,
        //              prize_reg[2].parse::<usize>().expect("Failed to parse int") + 10000000000000];

        machines.push([a_button, b_button, prize]);
        lines.next();
    }
    
    let mut p1_sum = 0;
    for machine in machines {
        // Searchspace: presses on A, presses on B
        // Current best match: presses on A, presses on B, cost
        let mut cur_match: Option<[usize; 3]> = None;
        let max_a = cmp::min(machine[2][0] / machine[0][0], machine[2][1] / machine[0][1]);

        // println!("{}", max_a);

        for a in 0..max_a+1 {
            let x = a * machine[0][0];
            let y = a * machine[0][1];
            // There is some amount of B button presses that gets us to the prize on X-axis
            if (machine[2][0] - x) % machine[1][0] == 0 {
                let b = (machine[2][0] - x) / machine[1][0];
                // That same amount also aligns us on the Y-axis
                if machine[2][1] == y + b * machine[1][1] {
                    let cost = get_cost(a, b);
                    if !cur_match.is_none() && cur_match.unwrap()[2] < cost {
                        continue
                    }
                    cur_match = Some([a, b, cost]);
                }
            }
        }
        if cur_match != None {
            p1_sum += cur_match.unwrap()[2];
        }
    }

    println!("Total part 1 cost: {}", p1_sum);
    // println!("Total part 2 cost: {}", p2_sum);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn get_cost(a: usize, b:usize) -> usize {
    a*3 + b
}
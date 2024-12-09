use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let timer = Instant::now();
    let mut corr_sum = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let line_arr: Vec<&str> = line.split(": ").collect();
        let res: usize = line_arr[0].parse().expect("Cant parse to int");
        let parameters: Vec<usize> = line_arr[1].split(" ").map(|str| str.parse().expect("Cant parse to int")).collect();
        let poss_res = calculate(&parameters, res);
        if poss_res.contains(&res) {
            corr_sum += res;
        }
    }
    // assert_eq!(corr_sum, 426214131924213);
    println!("Total amount of correct formula's: {}", corr_sum);
    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn calculate(params: &[usize], res: usize) -> Vec<usize> {
    let mut possible_res = Vec::new();
    if params.len() <= 2 {
        possible_res.push(params[0] + params[1]);
        possible_res.push(params[0] * params[1]);
        let concat = params[0].to_string() + &params[1].to_string();
        possible_res.push(concat.parse().expect("Failed to concat"));
    } else {
        let param = params.last().expect("Failed to retrieve last item");
        let results = calculate(&params[0..params.len()-1], res);
        for result in results {
            let sum = param + result;
            if sum <= res {
                possible_res.push(sum);
            }
            let mult = param * result;
            if mult <= res {
                possible_res.push(mult);
            }
            let concat = (result.to_string() + &param.to_string()).parse().expect("Failed to concat");
            if concat <= res {
                possible_res.push(concat);
            }
        }
    }
    return possible_res;
}
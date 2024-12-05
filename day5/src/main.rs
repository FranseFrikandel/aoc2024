use std::fs::read_to_string;

fn main() {
    let mut rules: Vec<Vec<usize>> = Vec::new();
    let mut ordered_mid_pages: Vec<usize> = Vec::new();
    let mut unordered_mid_pages: Vec<usize> = Vec::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        if line.contains("|") {
            let mut vec: Vec<usize> = Vec::new();
            for value in line.split("|") {
                vec.push(value.parse().expect("Failed to convert rule to integer"));
            }
            rules.push(vec);
        }

        else if line.len() > 1 {
            let mut line_arr = line_to_array(&line);
            if check_line_rule(&line_arr, &rules) {
                // Valid ordering
                ordered_mid_pages.push(line_arr[line_arr.len()/2]);
            } else {
                // Invalid ordering
                fix_line(&mut line_arr, &rules);
                while !check_line_rule(&line_arr, &rules) {
                    fix_line(&mut line_arr, &rules);
                }
                unordered_mid_pages.push(line_arr[line_arr.len()/2]);
            }
        }
    }
    let ordered_sum: usize = ordered_mid_pages.into_iter().sum();
    let unordered_sum: usize = unordered_mid_pages.into_iter().sum();
    println!("Ordered pages sum to: {}", ordered_sum);
    println!("Unordered pages sum to: {}", unordered_sum);
}

fn check_line_rule(vector: &Vec<usize>, rules: &Vec<Vec<usize>>) -> bool {
    let mut seen_vars: Vec<usize> = Vec::new();
    for value in vector {
        for rule in rules {
            if rule[0] == *value && seen_vars.contains(&rule[1]) {
                return false;
            }
        }
        seen_vars.push(*value);
    }
    return true;
}

fn line_to_array(line: &str) -> Vec<usize> {
    let mut vec: Vec<usize> = Vec::new(); 
    for value in line.split(",") {
        let value_int = value.parse().expect("Failed to convert page to integer");
        vec.push(value_int);
    }
    return vec;
}

fn fix_line(vector: &mut Vec<usize>, rules: &Vec<Vec<usize>>) {
    // let mut vector = vector.clone();
    for i in 0..vector.len() {
        for rule in rules {
            if rule[1] == vector[i] && vector[i..].contains(&rule[0]) {
                let index = vector.iter().position(|&r| r == rule[0]).unwrap();
                let val = vector.remove(index);
                vector.insert(i, val);
            }
        }
    }
}
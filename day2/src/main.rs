use std::fs::read_to_string;

fn main() {
    let mut safe_reports = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let mut values: Vec<i32> = Vec::new();
        for var in line.split(" ") {
            values.push(var.parse().expect("Could not parse into a number"));
        }

        // Generate a vector of all possible reports where a single value is removed
        let mut poss_values: Vec<Vec<i32>> = Vec::new();
        poss_values.push(values.clone());
        for i in 0..values.len() {
            let mut mod_values = values.clone();
            mod_values.remove(i);
            poss_values.push(mod_values);
        }

        let mut poss_diff: Vec<Vec<i32>> = Vec::new();
        for value in poss_values {
            let mut diff: Vec<i32> = Vec::new();
            for i in 0..value.len()-1 {
                diff.push(value[i+1] - value[i]);
            }
            poss_diff.push(diff);
        }

        for idiff in poss_diff {
            if check_if_safe(idiff) {
                safe_reports += 1;
                break;
            }
        }
        // if check_if_safe(poss_diff[0].clone()) {
        //     safe_reports += 1;
        // }
    }
    println!("{safe_reports}");
}

fn check_if_safe(diff: Vec<i32>) -> bool {
    if diff[0] > 0 {
        for el in diff {
            if el > 3 {
                return false;
            } else if el < 1 {
                return false;
            }
        }
    } else if diff[0] < 0 {
        for el in diff {
            if el < -3 {
                return false;
            } else if el > -1 {
                return false;
            }
        }
    } else {
        return false;
    }
    return true;
}
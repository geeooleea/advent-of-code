use std::{cmp::min, fs, time::Instant};

fn flip_characters(current_indicator: &String, positions: &Vec<usize>) -> String {
    let mut current_indicator_new = current_indicator.clone();
    for key in positions {
        if current_indicator.chars().nth(*key).unwrap() == '#' {
            current_indicator_new.replace_range(*key..(*key+1), ".");
        } else {
            current_indicator_new.replace_range(*key..(*key+1), "#");
        }
    }
    return current_indicator_new;
}

fn min_button_pushes(
    target_indicator: &String,
    current_indicator: &String,
    buttons: &Vec<Vec<usize>>,
    idx: usize
) -> i32 {
    if *current_indicator == *target_indicator {
        return 0;
    } else if idx >= buttons.len() {
        return i32::MAX - 1;
    }
    // press button
    let new_indicator = flip_characters(current_indicator, buttons.iter().nth(idx).unwrap());
    let push = 1+min_button_pushes(target_indicator, &new_indicator, buttons, idx+1);
    let no_push = min_button_pushes(target_indicator, current_indicator, buttons, idx+1);
    return min(push, no_push);
}

fn satisfy_joltage(current_indicator: &Vec<i32>, positions: &Vec<usize>, n_applications: i32) -> Vec<i32> {
    let mut new_vec = current_indicator.clone();
    for idx in positions {
        new_vec[*idx] -= n_applications;
    }
    return new_vec;
}

fn min_joltage_button_pushes(
    joltage: &Vec<i32>,
    buttons: &Vec<Vec<usize>>,
    idx: usize,
    buttons_remaining: &Vec<i32>
) -> i32 {

    let button_switch = buttons.iter().nth(idx).unwrap();

    if idx == buttons.len() {
        return i32::MAX - 1;
    }
    // Optimization: no need to recurse on last button
    if idx == buttons.len() - 1 {
        for i in 0..button_switch.len()-1 {
            if joltage[button_switch[i]] != joltage[button_switch[i+1]] {
                return i32::MAX - 1;
            }
        }  // If we get here, they are all the same
        let new_joltage = satisfy_joltage(joltage, &button_switch, joltage[buttons[idx][0]]);
        let total_joltage: i32 = new_joltage.iter().sum();
        if total_joltage == 0 {
            return joltage[buttons[idx][0]];
        } else {
            return i32::MAX - 1;
        }
    }

    for i in 0..joltage.len()-1 {
        for j in i+1..joltage.len() {
            let (higher_idx, lower_idx);
            if joltage[i] > joltage[j] {
                higher_idx = i;
                lower_idx = j;
            } else if joltage[j] > joltage[i] {
                higher_idx = j;
                lower_idx = i;
            } else { continue; }
            let mut button_exists = false;
            for k in idx..buttons.len() {
                if buttons[k].contains(&higher_idx) && !buttons[k].contains(&lower_idx) {
                    button_exists = true;
                    break;
                }
            }
            if !button_exists {
                return i32::MAX - 1;
            }
        }
    }

    // Invalid configuration (too many button pushes for one indicator)
    let any_negative: Vec<&i32> = joltage.iter().filter(|x| **x < 0).collect();
    if any_negative.len() > 0 {
        return i32::MAX - 1;
    }
    
    // Invalid configuation (can't reach a valid configuration from here w/ the remaining buttons)
    for (j, b) in joltage.iter().zip(buttons_remaining) {
        if *j > 0 && *b == 0 {
            return i32::MAX - 1;
        }
    }

    // Exit condition: valid configuration
    let total_joltage: i32 = joltage.iter().sum();
    if total_joltage == 0 {
        return 0;
    }

    let new_indicator = satisfy_joltage(joltage, &button_switch, 1);
    // press button again
    let push_again = 1+min_joltage_button_pushes(&new_indicator, buttons, idx, buttons_remaining);
    // no press
    
    let new_buttons = satisfy_joltage(buttons_remaining, &button_switch, 1);
    let no_push = min_joltage_button_pushes(joltage, buttons, idx+1, &new_buttons);
    return min(no_push, push_again);
}

pub fn day10(part: i32) {
    let file_path = "/Users/giuliacarocari/aoc/inputs/day10.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut min_operations_sum = 0;
    for line in lines {
        let mut buttons: Vec<Vec<usize>> =  vec![];

        let mut split_line = line.split(" ");

        let indicators_with_terms = split_line.next().unwrap().to_string().to_string();
        let target_indicators: String = indicators_with_terms[1..indicators_with_terms.len()-1].to_string();
        let mut target_joltage: Vec<i32> = vec![0];
        let mut item: Option<&str> = split_line.next();
        while item != None {
            let unwrapped_item = item.unwrap().to_string();
            let first_char= unwrapped_item.chars().nth(0).unwrap();
            let vec_no_parentheses = unwrapped_item[1..unwrapped_item.len()-1].to_string();
            if first_char == '(' {
                let button_as_vec: Vec<usize> = vec_no_parentheses.split(',').map(|num| num.parse::<usize>().unwrap()).collect();
                buttons.push(button_as_vec);
            } else if first_char == '{' {
                target_joltage = vec_no_parentheses.split(',').map(|num| num.parse::<i32>().unwrap()).collect();
            }
            item = split_line.next();
        }
        if part == 1 {
            let current_indicator = std::iter::repeat(".").take(target_indicators.len()).collect::<String>();
            min_operations_sum += min_button_pushes(&target_indicators, &current_indicator, &buttons, 0); 
        } else {
            // (3) (1,3) (2) (2,3) (0,2) (0,1)          {3,5,4,7}
            // (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4)  {7,5,12,7,2}
            // (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2)    {10,11,11,5,10,5}
            println!("Starting {:?}", target_joltage);

            // Need to compute button-wise GCD: this would tell us how many times each button can be pressed?
            // Would still affect total result though, because pressing GCD[button] times might affect other button combinations

            // let mut gcd = target_joltage[0];
            // for i in 0..target_joltage.len() {
            //     gcd = num_integer::gcd(gcd, target_joltage[i]);
            // }
            // println!("GCD: {gcd}");

            // target_joltage = target_joltage.iter().map(|x| (*x)/gcd).collect();

            // Heuristic 1: burn more switches early by using buttons with more switches first
            buttons.sort_by(|a, b| b.len().cmp(&a.len()));

            let mut buttons_remaining: Vec<i32> = vec![0; target_joltage.len()];
            for i in 0..buttons.len() {
                for j in 0..buttons[i].len() {
                    buttons_remaining[buttons[i][j]] += 1;
                }
            }

            let now = Instant::now();
            let min_ops= min_joltage_button_pushes(
                &target_joltage,
                &buttons,
                0,
                &buttons_remaining
            );
            min_operations_sum += min_ops;
            let elapsed = now.elapsed();
            println!("Time taken: {:.2?}", elapsed);
        }
    }
    println!("Minimum number of button pushes: {min_operations_sum}")
}

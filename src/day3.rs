use std::{cmp::max, fs};

fn get_max_joltage(bank: &str) -> i64 {
    let mut max_joltage_in_bank: i64 = 0;
    for (i, first_char) in bank.chars().enumerate() {
        for second_char in bank.split_at(i+1).1.chars() {
            let first_char_numeric: i64 = first_char.to_digit(10).expect("LIESSS") as i64 * 10;
            max_joltage_in_bank = max(max_joltage_in_bank, first_char_numeric + second_char.to_digit(10).expect("More lies???") as i64);
        }
    }
    return max_joltage_in_bank;
}

fn get_supermax_joltage(memo: &mut Vec<[i64; 12]>, bank: &String, position: usize, significant_digit: i32) -> i64 {
    if bank.len() - position <= significant_digit as usize {
        return 0;
    } else if significant_digit == -1 || position >= bank.len() {
        return 0;
    }

    if memo[position][significant_digit as usize] < 0 {
        let char_at_position = bank[position..position+1].chars().next().expect("Surely this exists");
        let digit_at_position = char_at_position.to_digit(10).expect("Surely it must be a number") as i64;
        let base: i64 = 10;
        let multiplicator: i64 = base.pow(significant_digit.try_into().expect("Shit, how did we get here?"));
        let value = multiplicator * digit_at_position;
        let take = value + get_supermax_joltage(memo, bank, position+1, significant_digit-1);
        let no_take = get_supermax_joltage(memo, bank, position+1, significant_digit);
        memo[position][significant_digit as usize] = max( take, no_take);
    }
    return memo[position][significant_digit as usize];
}


pub fn day3(part: i32) {
    
    let file_path = "/Users/giuliacarocari/aoc/inputs/day3.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let banks = contents.split("\n");

    let mut max_joltage: i64 = 0;

    for bank in banks {
        if part == 1 {
            max_joltage += get_max_joltage(bank);
        } else {
            let mut memo: Vec<[i64; 12]> =  vec![[-1; 12]; bank.len()];
            let joltage= get_supermax_joltage(&mut memo, &bank.to_string(), 0, 11);
            max_joltage += joltage;
        }
    }
    println!("Maximum power at: {max_joltage} joltage")
}

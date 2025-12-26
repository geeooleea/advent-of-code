use std::fs;


pub fn day1() {
    let file_path = "/Users/giuliacarocari/aoc/input.txt";

    let contents = fs::read_to_string(file_path).unwrap();
    let lines = contents.split("\n");
    
    let mut position: i32 = 50;
    let mut num_zeros: i32 = 0;

    for line in lines {
        let number: String = line.chars().skip(1).collect();
        let steps: i32 = number.parse::<i32>().unwrap();
        if line.starts_with("L") {
            position -= steps;
        } else if line.starts_with("R") {
            position += steps;
        } else {
            panic!("Oh no!")
        }
        position %= 100;
        if position == 0 {
            num_zeros += 1;
        }
    }

    println!("Password: {num_zeros}");
}

pub fn day1_part2() {
    let file_path = "/Users/giuliacarocari/aoc/input.txt";
    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).unwrap();

    let lines = contents.split("\n");

    let mut position: i32 = 50;
    let mut num_zeros: i32 = 0;

    for line in lines {
        let number: String = line.chars().skip(1).collect();
        let mut steps: i32 = number.parse::<i32>().unwrap();
        num_zeros += steps / 100;
        steps %= 100;
        if line.starts_with("L") {
            if position != 0 && position - steps <= 0 {
                num_zeros += 1;
            }
            position -= steps;
        } else if line.starts_with("R") {
            if position != 0 && position + steps >= 100 {
                num_zeros += 1;
            }
            position += steps;
        } else {
            panic!("Oh no!")
        }
        position = (position + 100) % 100;
    }

    println!("Actual password: {num_zeros}");
}
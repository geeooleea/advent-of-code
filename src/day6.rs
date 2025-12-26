use std::{fs};


pub fn day6(part: i32) {
    let file_path = "/Users/giuliacarocari/aoc/inputs/day6.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lines = contents.split("\n");

    let mut problems: Vec<Vec<i64>> = vec![];
    let mut operations: Vec<char> = vec![];

    if part == 1 {
        loop {
            let line = lines.next();
            if line.is_none() {
                break;
            }
            let line_as_str = line.expect("huhhhh").to_string();
            let nums_as_strs = line_as_str.split_whitespace();
            let mut i = 0;
            for num in nums_as_strs {
                if num.len() == 0 { continue; }
                if num == "*" || num == "+" {
                    operations.push(num.chars().next().expect(""));
                } else {
                    let number = num.parse::<i64>().unwrap();
                    if problems.len() <= i {
                        problems.push(vec![number]);
                    } else {
                        problems[i].push(number);
                    }
                }
                i+=1;
            }
        }
    } else {
        let max_column_width = lines.clone().map(|x| x.len()).max().unwrap();

        let mut problem_id = 0;

        for j in 0..max_column_width {
            let mut number_as_str_from_left_to_right = String::new();
            let copied_lines = lines.clone();
            for line in copied_lines {
                let c = line.chars().nth(j).unwrap();
                if c != ' ' && c != '*' && c != '+' {
                    number_as_str_from_left_to_right.push(c);
                } else if c == '*' || c == '+' {
                    operations.push(c);
                }
            }
            if number_as_str_from_left_to_right == "" {
                problem_id += 1
            } else {
                let number_from_str = number_as_str_from_left_to_right.parse::<i64>().unwrap();
                if problem_id >= problems.len() {
                    problems.push(vec![number_from_str]);
                } else {
                    problems[problem_id].push(number_from_str);
                }
            }
        }
    }
    
    let mut big_total_result: i64 = 0;
    for (i, operation) in operations.iter().enumerate() {
        let numbers = &problems[i];
        if *operation == '+' {
            big_total_result += (*numbers).iter().sum::<i64>();
        } else {
            big_total_result += (*numbers).iter().product::<i64>();
        }
    }
    println!("Big total result (as computed in part {part}) is {big_total_result}");
}

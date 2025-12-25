use std::fs;

fn is_invalid(n: i64) -> bool {
    let n_as_str = n.to_string();
    if n_as_str.len() % 2 != 0 {
        return false;
    } else {
        // str is a number -> ascii only?
        let (half1, half2) = n_as_str.split_at(n_as_str.len()/2);
        return half1 == half2;
    }
}


fn is_invalid_multiple_reps(n: i64) -> bool {
    let n_as_str = n.to_string();
    let len = n_as_str.len();
    for i in 1..=(len/2) {
        if len % i == 0 {
            let number_of_segments = len / i;
            let mut segments: Vec<String> = vec![String::new(); number_of_segments];

            let mut str_to_split = n_as_str.clone();

            for j in 0..number_of_segments {
                let (half1, half2) = str_to_split.split_at(i);
                segments[j] = half1.to_string();
                str_to_split = half2.to_string();
            }
            let first_segment = segments[0].clone();

            let mut valid = true;
            for segment in segments {
                if segment != first_segment {
                    valid = false;
                    break;
                }
            }
            if valid {return true;}
        }
    }
    return false;
}


pub fn day2(part: i32) {
    
    let file_path = "/Users/giuliacarocari/aoc/inputs/day2.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let ranges_str = contents.split(",");

    let mut invalid_sum: i64 = 0; 

    for range_str in ranges_str {
        let mut range = range_str.split("-");
        let left = range.next().expect("Y NO NEXT??").parse::<i64>().unwrap();
        let right = range.next().expect("Y NO NEXT??").parse::<i64>().unwrap();

        for n in left..=right {
            if part ==1 {
                if is_invalid(n) {
                    invalid_sum += n;
                }
            } else if part == 2 {
                if is_invalid_multiple_reps(n) {
                    invalid_sum += n;
                }
            } else {
                panic!("There's only two parts to this day \\(”˚☐˚)/")
            }
            
        }
    }
    println!("All invalid IDs sum up to: {invalid_sum}")
}

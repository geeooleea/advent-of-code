use std::fs;

fn is_invalid(n: i64) -> bool {
    let n_digits = n.ilog10() + 1;
    if n_digits % 2 != 0 {
        return false;
    } else {
        let exp = (10 as i64).pow(n_digits/2);
        return n % exp == n / exp;
    }
}


fn is_invalid_multiple_reps(n: i64) -> bool {
    let n_as_str = n.to_string();
    let n_str_bytes = n_as_str.as_bytes();
    let len = n_as_str.len();
    for i in 1..=(len/2) {
        if len % i == 0 {
            let segments = n_str_bytes.chunks(i)
                .map(|buf| unsafe { str::from_utf8_unchecked(buf) })
                .collect::<Vec<&str>>();

            let first_segment = segments[0];
            let valid = segments.iter().all(|x| *x == first_segment);
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
        let left = range.next().unwrap().parse::<i64>().unwrap();
        let right = range.next().unwrap().parse::<i64>().unwrap();

        if part ==1 {
            for n in left..=right {
                if is_invalid(n) {
                    invalid_sum += n;
                }
            }
        } else if part == 2 {
            for n in left..=right {
                // Todo: there has to be a way to skip checking many numbers in the range.
                if is_invalid_multiple_reps(n) {
                    invalid_sum += n;
                }
            }
        } else {
            panic!("There's only two parts to this day \\(”˚☐˚)/");
        }

    }
    println!("All invalid IDs sum up to: {invalid_sum}")
}

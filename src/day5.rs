use std::fs;


pub fn day5(part: i32) {
    let file_path = "/Users/giuliacarocari/aoc/inputs/day5.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lines = contents.split("\n");

    let mut ranges: Vec<(i64, i64)> = vec![];
    let mut product_ids: Vec<i64> = vec![];
    let mut reading_ranges = true;

    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let line_str = line.expect("huhhhh").to_string();
        if line_str == "" {
            reading_ranges = false;
        } else if reading_ranges {
            let mut range_strs = line_str.split("-");
            let lower_bound = range_strs.next().expect("").parse::<i64>().unwrap();
            let upper_bound = range_strs.next().expect("").parse::<i64>().unwrap();
            ranges.push((lower_bound, upper_bound)); 
        } else {
            let product_id = line_str.parse::<i64>().unwrap();
            product_ids.push(product_id);
        }
    }

    if part == 1 {
        let mut fresh_products: i32 = 0;

        for product_id in product_ids {
            for (lower, upper) in &ranges {
                if product_id >= *lower && product_id <= *upper {
                    fresh_products += 1;
                    break;
                }
            }
        }
        println!("Numer of fresh products: {fresh_products}");
    } else {
        ranges.sort();
        let mut merged_ranges: Vec<(i64, i64)> = vec![];
        for range in &ranges {
            if merged_ranges.len() == 0 {
                merged_ranges.push(*range);
                continue;
            }
            let (lower, upper) = *range;
            let previous_range = merged_ranges.pop().expect("Stack should not be empty");
            if lower > previous_range.1 {
                merged_ranges.push(previous_range);
                merged_ranges.push(*range);
            } else if upper > previous_range.1 {
                merged_ranges.push((previous_range.0, upper));
            } else {
                merged_ranges.push(previous_range);  // just undo pop
            }
        }
        let mut total_fresh_ingredients: i64 = 0;
        for (lower, upper) in merged_ranges {
            total_fresh_ingredients += upper - lower + 1;
        }
        println!("All fresh ingredient ranges contain {total_fresh_ingredients} products.")
    }
    
}

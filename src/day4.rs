use std::fs;


fn find_removable(floor_plan: & mut Vec<Vec<char>>, remove_as_we_go: bool) -> i32 {
    let n_rows = floor_plan.len();
    let n_columns = floor_plan[0].len();

    let n_rows_as_int = floor_plan.len() as i32;
    let n_columns_as_int = floor_plan[0].len() as i32;
    let mut n_free = 0;

    for i in 0..n_rows {
        for j in 0..n_columns {
            let (i_as_i32, j_as_i32) = (i as i32, j as i32);
            if floor_plan[i][j] == '@' {
                let all_indices_to_check: [(i32, i32); 8] = [
                    (i_as_i32-1, j_as_i32-1),
                    (i_as_i32-1, j_as_i32),
                    (i_as_i32-1, j_as_i32+1),
                    (i_as_i32, j_as_i32-1),
                    (i_as_i32, j_as_i32+1),
                    (i_as_i32+1, j_as_i32-1),
                    (i_as_i32+1, j_as_i32),
                    (i_as_i32+1, j_as_i32+1)
                ];
                let mut n_adjacent_rolls = 0;
                for (i_, j_) in all_indices_to_check {
                    if i_ >= 0 && j_ >= 0 && i_ < n_rows_as_int && j_ < n_columns_as_int {
                        let i_idx = i_ as usize;
                        let j_idx = j_ as usize;
                        n_adjacent_rolls += if floor_plan[i_idx][j_idx] == '@' {1} else {0};
                    }
                }
                if n_adjacent_rolls < 4 {
                    n_free += 1;
                    if remove_as_we_go {
                        floor_plan[i][j] = '.';
                    }
                }
            }
        }
    }
    return n_free;
}


pub fn day4(part: i32) {
    let file_path = "/Users/giuliacarocari/aoc/inputs/day4.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut floor_plan: Vec<Vec<char>> =  vec![];
    for line in lines {
        let mut line_vec: Vec<char> = vec![];
        for char in line.chars() {
            line_vec.push(char);
        }
        floor_plan.push(line_vec);
    }

    if part == 1 {
        let accessible = find_removable(& mut floor_plan, false);
        println!("Number of accessible rolls: {accessible}");
    } else {
        let mut still_removing = true;
        let mut all_removable = 0;
        while still_removing {
            let removed_now = find_removable(& mut floor_plan, true);
            all_removable += removed_now;
            if removed_now == 0 {
                still_removing = false;
            }
        }
        println!("Number of accessible rolls after removals: {all_removable}");
    }
   
}

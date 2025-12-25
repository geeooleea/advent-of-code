use std::fs;


pub fn day7(part: i32) {
    let file_path = "/Users/giuliacarocari/aoc/inputs/day7.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut beam_path: Vec<Vec<char>> =  vec![];
    for line in lines {
        let mut line_vec: Vec<char> = vec![];
        for char in line.chars() {
            line_vec.push(char);
        }
        beam_path.push(line_vec);
    }

    let start_idx = beam_path[0].iter().enumerate().filter(|(_, c)| **c == 'S').map(|(i, _)| i).next().unwrap();
    beam_path[0][start_idx] = '|';

    if part == 1 {
        let width = beam_path[0].len();
        let mut n_splits = 0;
        for i in 0..(beam_path.len()-1) {
            for j in 0..width {
                if beam_path[i][j] == '|' {
                    if beam_path[i+1][j] == '^' {
                        n_splits += 1;
                        if (j as i32)-1 >= 0 {
                            beam_path[i+1][j-1] = '|';
                        }
                        if j+1 < width {
                            beam_path[i+1][j+1] = '|';
                        }
                    } else {
                        beam_path[i+1][j] = '|';
                    }
                }
            }
        }
        println!("Number of splits at the end {n_splits}");
    } else {
        let width = beam_path[0].len();
        let mut n_worlds_in_position: Vec<Vec<i64>> = vec![vec![0; width]; beam_path.len()];
        n_worlds_in_position[0][start_idx] = 1;
        for i in 0..(beam_path.len()-1) {
            for j in 0..width {
                if beam_path[i][j] == '|' {
                    if beam_path[i+1][j] == '^' {
                        if j > 0 {
                            beam_path[i+1][j-1] = '|';
                            n_worlds_in_position[i+1][j-1] += n_worlds_in_position[i][j];
                        }
                        if j+1 < width {
                            beam_path[i+1][j+1] = '|';
                            n_worlds_in_position[i+1][j+1] += n_worlds_in_position[i][j];
                        }
                    } else {
                        beam_path[i+1][j] = '|';
                        n_worlds_in_position[i+1][j] += n_worlds_in_position[i][j];
                    }
                }
            }
        }
        let n_worlds: i64 = n_worlds_in_position[n_worlds_in_position.len()-1].iter().sum();
        println!("Number of worlds at the end {n_worlds}");
    }
   
}

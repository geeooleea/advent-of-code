use std::{fs, i64};


pub fn day12(part: i64) {
    let file_path = "/Users/giuliacarocari/aoc/inputs/day12.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    const N_SHAPES : usize= 6;

    let mut shapes: Vec<Vec<Vec<bool>>> = vec![vec![]; N_SHAPES];
    let mut shape_sizes: Vec<i64> = vec![];
    let mut tree_sizes: Vec<(i64, i64)> = vec![];
    let mut tree_packing_requirements: Vec<Vec<i64>> = vec![];

    let mut curr_shape : usize = 0;

    let mut i = 0;
    while i < lines.len() {  // for shapes this line has index information
        if curr_shape < N_SHAPES {
            for _ in 1..4 {
                i+=1;
                let line = lines[i].chars().map(|x| if x=='#' {true} else {false});
                shapes[curr_shape].push(Vec::from_iter(line));
            }
            let shape_size = shapes[curr_shape].iter().map(|b| b.iter().filter(|&&a| a).count()).sum::<usize>() as i64;
            shape_sizes.push(shape_size);
            curr_shape += 1;
            i+=1;  // burn empty line between shapes
        } else {
            let split_line: Vec<String> = lines[i].split(":").map(|x| x.to_string()).collect();
            let sizes_vec: Vec<i64> = split_line[0].split("x").map(|x| x.to_string().parse::<i64>().unwrap()).collect();
            tree_sizes.push((sizes_vec[0], sizes_vec[1]));
            let tree_requirements = split_line[1][1..].split(" ").map(|x| x.to_string().parse::<i64>().unwrap()).collect();
            tree_packing_requirements.push(tree_requirements);
        }
        i+=1;
    }

    if part == 1 {
        let mut satisf_tree_sizes: Vec<(i64, i64)> = vec![];
        let mut satisf_tree_packing_requirements: Vec<Vec<i64>> = vec![];
        let mut excluded_configs = 0;
        let mut valid_configs = 0;

        for ((x, y), requirements) in tree_sizes.iter().zip(&tree_packing_requirements) {
            let area = x * y;
            let mut present_area = 0;
            for (size, requirement) in shape_sizes.iter().zip(requirements) {
                present_area += requirement * size;
            }
            if present_area > area {
                excluded_configs += 1;
            } else if 9 * requirements.iter().sum::<i64>() <= area {
                valid_configs += 1;
            } else {
                satisf_tree_sizes.push((*x, *y));
                satisf_tree_packing_requirements.push(requirements.clone());
            }
        }

        println!("Excluded {excluded_configs} because present area is greater than tree area!");
        println!("Found {valid_configs} valid configs for trivial packing");
        println!("{} configs remaining", satisf_tree_sizes.len());
    }
}

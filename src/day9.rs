use std::{cmp::{max, min}, fs};


pub fn day9(part: i32) {
    let file_path = "/Users/giuliacarocari/aoc/inputs/day9.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut points: Vec<(usize, usize)> =  vec![];
    for line in lines {
        let mut split_line = line.split(",");
        let x = split_line.next().unwrap().parse::<usize>().unwrap();
        let y = split_line.next().unwrap().parse::<usize>().unwrap();
        points.push((x, y));
    }

    if part == 1 {
        let mut max_size = 0;
        for i in 0..points.len() - 1 {
            for j in i+1..points.len() {
                let a_size = points[i].0 - points[j].0;
                let b_size = points[i].1 - points[j].1;
                max_size = max(max_size, ((a_size as i64).abs() + 1) * ((b_size as i64).abs() + 1));
            }
        }
        println!("Max rectangle size: {}", max_size)
    } else {
        let max_x= *points.iter().map(|(x, _)| x).max().unwrap();
        let max_y= *points.iter().map(|(_, y)| y).max().unwrap();

        println!("Grid size: {max_x} x {max_y}");

        let mut grid: Vec<Vec<bool>> = vec![vec![false; max_x+1 as usize]; max_y+1 as usize];

        let mut vertical_lines : Vec<(usize, (usize, usize))>= vec![];
        let mut horizontal_lines : Vec<(usize, (usize, usize))>= vec![];
        for i in 0..points.len() - 1 {
            if points[i].0 == points[i+1].0 {
                // let (bottom, top) = (min(points[i].1, points[i+1].1), max(points[i].1, points[i+1].1));
                vertical_lines.push((points[i].0, (points[i].1, points[i+1].1)));
            } else if points[i].1 == points[i+1].1 {
                // let (left, right) = (min(points[i].0, points[i+1].0), max(points[i].0, points[i+1].0));
                horizontal_lines.push((points[i].1, (points[i].0, points[i+1].0)));
            } else {
                panic!("Looks like my assumption was wrong...")
            }
        }

        if points[0].0 == points[points.len()-1].0 {
            // let (bottom, top) = (min(points[0].1, points[points.len()-1].1), max(points[0].1, points[points.len()-1].1));
            vertical_lines.push((points[0].0, (points[0].1, points[points.len()-1].1)));
        } else if points[0].1 == points[points.len()-1].1 {
            // let (left, right) = (min(points[0].0, points[points.len()-1].0), max(points[0].0, points[points.len()-1].0));
            horizontal_lines.push((points[0].1, (points[0].0, points[points.len()-1].0)));
        } else {
            panic!("Looks like my assumption was wrong...")
        }

        horizontal_lines.sort();
        
        // We are looking at these axes:
        //  --------> x
        //  |
        //  |
        //  v y
        //
        for (y, (x1, x2)) in &horizontal_lines {
            
            let (x_min, x_max) = (*x1.min(x2), *x1.max(x2));
            let inverted_slice: Vec<bool> = vec![!grid[*y][x_min]; x_max-x_min];
            let start: usize;
            if grid[*y][x_min] { // Flipping from positive to negative: do not touch this line
                start = y+1;
            } else {
                start = *y;
            }
            for y_i in start..max_y+1 {
                grid[y_i][x_min..x_max].copy_from_slice(&inverted_slice);
            }
        }

        println!("Finished constructing grid");

        // ..............
        // .......#XXX#..
        // .......X000X..
        // ..#XXXX#000X..
        // ..X00000000X..
        // ..#XXXXXX#0X..
        // .........X0X..
        // ..#XXXXXX#0#..
        // ..#XXXXXXXX#..
        // ..............

        let mut max_size: i64 = 0;
        for i in 0..points.len() - 1 {
            for j in i+1..points.len() {
                let x_min = min(points[i].0, points[j].0);
                let x_max = max(points[i].0, points[j].0);
                let y_min = min(points[i].1, points[j].1);
                let y_max = max(points[i].1, points[j].1);
                
                let mut is_valid = true;

                for xp in x_min..x_max+1 {
                    if !grid[y_min][xp] {
                        is_valid = false;
                        break;
                    }
                    if !grid[y_max][xp] {
                        is_valid = false;
                        break;
                    }
                }
                for yp in y_min..y_max+1 {
                    if !grid[yp][x_min] {
                        is_valid = false;
                        break;
                    }
                    if !grid[yp][x_max] {
                        is_valid = false;
                        break;
                    }
                }

                if is_valid {
                    max_size = max(max_size, ((x_max-x_min+1) as i64) * ((y_max-y_min+1) as i64));
                }
            }
        }
        println!("Max rectangle size: {}", max_size)
        
    }
}

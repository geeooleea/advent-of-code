use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, fs};

struct Point {
    x: i64,
    y: i64,
    z: i64
}

impl Point {
    fn square_distance(&self, other: &Point) -> i64 {
        return self.x - other.x * self.x - other.x +
            self.y - other.y * self.y - other.y +
            self.z - other.z * self.z - other.z;
    }
}

pub fn day8(part: i32) {
    let file_path = "/Users/giuliacarocari/aoc/inputs/day8.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut points: Vec<Point> =  vec![];
    for line in lines {
        let mut split_line = line.split(",");
        let x = split_line.next().unwrap().parse::<i64>().unwrap();
        let y = split_line.next().unwrap().parse::<i64>().unwrap();
        let z = split_line.next().unwrap().parse::<i64>().unwrap();
        points.push(Point { x, y, z });
    }

    let mut pairwise_distances: BinaryHeap<(Reverse<i64>, (usize, usize))> = BinaryHeap::new();

    for i in 0..points.len() - 1 {
        for j in i+1..points.len() {
            pairwise_distances.push((Reverse(points[i].square_distance(&points[j])), (i, j)));
        }
    }

    let mut connected_components: Vec<usize> = (0..points.len()).collect();
    let mut n_components = points.len();
    let mut last_x: i64 = 0;
    let mut last_other_x: i64 = 0;
    while n_components > 1 {
        let (_, (i, j)) = pairwise_distances.pop().expect("");
        if connected_components[i] != connected_components[j] {
            n_components -= 1;
            if n_components == 1 {
                last_x = points[i].x;
                last_other_x = points[j].x;
            }
            let component_to_replace = connected_components[i];
            let new_component = connected_components[j];
            for x in 0..connected_components.len() {
                if connected_components[x] == new_component {
                    connected_components[x] = component_to_replace;
                }
            }
        }
    }
    
    if part == 1 {
        let mut m: HashMap<i32, usize> = HashMap::new();
        for x in connected_components {
            *m.entry(x as i32).or_default() += 1;
        }
        let mut components_with_size : Vec<(usize, i32)> = m.into_iter().map(|(k, v)| (v, k)).collect();
        components_with_size.sort();
        components_with_size.reverse();

        let largest_component_sizes = components_with_size[0].0 * components_with_size[1].0 * components_with_size[2].0;

        println!("Distinct components after merging 1000 circuits: {largest_component_sizes}");
    } else {
        println!("X coordinates of last points {}", last_x * last_other_x);
    }
}

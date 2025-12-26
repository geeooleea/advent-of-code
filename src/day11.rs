use std::{collections::{HashMap, VecDeque}, fs, i32};


fn get_id_or_add_new(graph: &mut Vec<Vec<usize>>, inverted_graph: &mut Vec<Vec<usize>>, id_map: &mut HashMap<String, usize>, source_name: String) -> usize {
    let source_id  = id_map.get(&source_name);
    let actual_id: usize;
    if source_id == None {
        actual_id = graph.len();
        graph.push(vec![]);
        inverted_graph.push(vec![]);
        id_map.insert(source_name,actual_id);
    } else {
        actual_id = *(source_id.unwrap());
    }
    return actual_id;
}

fn topsort(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, finished_visiting: &mut Vec<bool>, node: usize, ordered_list: &mut VecDeque<usize>) {
    if visited[node] || finished_visiting[node] { return; }

    visited[node] = true;
    let n_neighbors = graph[node].len();
    for i in 0..n_neighbors {
        topsort(graph, visited, finished_visiting, graph[node][i], ordered_list);
    }
    ordered_list.push_front(node);
    finished_visiting[node] = true;
}

pub fn day11(part: i32) {
    let file_path = "/Users/giuliacarocari/aoc/inputs/day11.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut graph: Vec<Vec<usize>> = vec![];
    let mut inverted_graph: Vec<Vec<usize>> = vec![];

    let mut id_map: HashMap<String, usize> = HashMap::new();

    for line in lines {
        let mut split_line = line.split(" ");
        
        let source_name = split_line.next().unwrap()[0..3].to_string();
        let source_id = get_id_or_add_new(&mut graph, &mut inverted_graph, &mut id_map, source_name);
        let mut destination_name = split_line.next();
        while destination_name != None {
            let destination_id = get_id_or_add_new(&mut graph, &mut inverted_graph, &mut id_map, destination_name.unwrap().to_string());
            graph[source_id].push(destination_id);
            inverted_graph[destination_id].push(source_id);
            destination_name = split_line.next();
        }
    }

    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut finished_visiting: Vec<bool> = vec![false; graph.len()];
    let mut ordered_list: VecDeque<usize> = VecDeque::new();

    if part == 1 {
        let start = *(id_map.get("you").unwrap());

        topsort(&graph, &mut visited, &mut finished_visiting, start, &mut ordered_list);

        let mut n_paths = vec![0; graph.len()];
        n_paths[start] = 1;

        for node in ordered_list {
            let parents = &inverted_graph[node];
            for parent in parents {
                n_paths[node] += n_paths[*parent];
            }
        }
        let n_paths_to_out = n_paths[*(id_map.get("out").unwrap())];

        println!("Number of paths: {n_paths_to_out}");
    } else {
        let start = *(id_map.get("svr").unwrap());
        let fft_id = *(id_map.get("fft").unwrap());
        let dac_id = *(id_map.get("dac").unwrap());
        topsort(&graph, &mut visited, &mut finished_visiting, start, &mut ordered_list);

        let mut n_paths: Vec<u64> = vec![0; graph.len()];
        n_paths[start] = 1;
        for node in ordered_list {
            let parents = &inverted_graph[node];
            for parent in parents {
                n_paths[node] += n_paths[*parent];
            }
            if node == fft_id || node == dac_id {
                let n_paths_so_far = n_paths[node];
                n_paths.fill(0);
                n_paths[node] = n_paths_so_far;
            }
        }
        let n_paths_to_out = n_paths[*(id_map.get("out").unwrap())];
        println!("Number of paths: {n_paths_to_out}");
    }
}

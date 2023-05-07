mod bfs;
mod dijkstra;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

fn check_six_degrees_of_separation(output: &HashMap<usize, i32>) -> bool {
    for val in output.values() {
        if *val < 6 {
            return true;
        }
    }

    false
}
fn bfs_transfer(file_path: &str) -> Vec<Vec<(usize, i32)>> {
    let mut graph = Vec::new();
    let file = std::fs::File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let nodes: Vec<&str> = line.split_whitespace().collect();
        let u: usize = nodes[0].parse().unwrap();
        let v: usize = nodes[1].parse().unwrap();
        let weight = 1;
        while graph.len() <= usize::max(u, v) {
            graph.push(Vec::new());
        }
        graph[u].push((v, weight));
        graph[v].push((u, weight));
    }
    graph
}

fn djikstra_transfer(filename: &str) -> Vec<Vec<(usize, i32)>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut graph = vec![vec![]; 4039];

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let node1 = parts[0] as usize;
        let node2 = parts[1] as usize;
        graph[node1].push((node2, 1));
        graph[node2].push((node1, 1));
    }

    graph
}

fn main() {
    // Dijkstra Algorithm
    let graph: Vec<Vec<(usize, i32)>> =
        djikstra_transfer("/Users/danieldoh/Desktop/facebook_combined.txt");
    let dist = dijkstra::dijkstra(0, &graph);
    println!("{:?}", dist);
    println!("{}", check_six_degrees_of_separation(&dist));

    // BFS Algorithm
    // let graph: Vec<Vec<(usize, i32)>> =
    //     bfs_transfer("/Users/danieldoh/Desktop/facebook_combined.txt");
    // let dist = bfs::bfs(0, &graph);
    // println!("{:?}", dist);
    // println!("{}", check_six_degrees_of_separation(&dist));
}

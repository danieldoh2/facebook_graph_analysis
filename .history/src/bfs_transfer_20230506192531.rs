use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn bfs_transfer(file_path: &str) -> Vec<Vec<(usize, i32)>> {
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

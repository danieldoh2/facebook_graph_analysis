use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    cost: i32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i32)>>) -> HashMap<usize, i32> {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: start,
    });
    dist.insert(start, 0);

    while let Some(State { cost, position }) = heap.pop() {
        if cost > *dist.get(&position).unwrap() {
            continue;
        }

        for &(neighbor, weight) in &graph[position] {
            let new_cost = cost + weight;
            if new_cost < *dist.get(&neighbor).unwrap_or(&i32::MAX) {
                heap.push(State {
                    cost: new_cost,
                    position: neighbor,
                });
                dist.insert(neighbor, new_cost);
            }
        }
    }

    dist
}

fn read_graph_data(filename: &str) -> Vec<Vec<usize>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut graph = vec![vec![]; 4039];

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let node1 = parts[0] as usize;
        let node2 = parts[1] as usize;
        graph[node1].push(node2);
        graph[node2].push(node1);
    }

    graph
}

fn main() {
    let graph = read_graph_data("/Users/danieldoh/Desktop/facebook_combined.txt");
    let dist = dijkstra(0, &graph);
    println!("{:?}", dist);
}

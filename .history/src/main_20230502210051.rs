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

fn read_graph_data(filename: &str) -> Vec<Vec<(usize, i32)>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut graph = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split_whitespace().collect();
        let mut neighbors = vec![];

        for i in 1..parts.len() {
            let (neighbor, weight) = parts[i].split_once(':').unwrap();
            neighbors.push((neighbor.parse().unwrap(), weight.parse().unwrap()));
        }

        graph.push(neighbors);
    }

    graph
}

fn main() {
    let graph = read_graph_data("/Users/danieldoh/Desktop/final_210/facebook_combined.txt");
    let dist = dijkstra(0, &graph);
    println!("{:?}", dist);
}

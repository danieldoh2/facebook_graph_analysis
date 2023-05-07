use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

type NodeId = usize;
type Weight = i32;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: Weight,
    node: NodeId,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Graph {
    adjacency_list: HashMap<NodeId, Vec<(NodeId, Weight)>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: NodeId, v: NodeId, weight: Weight) {
        self.adjacency_list
            .entry(u)
            .or_insert(Vec::new())
            .push((v, weight));
        self.adjacency_list
            .entry(v)
            .or_insert(Vec::new())
            .push((u, weight));
    }

    fn get_shortest_path(&self, start: NodeId, end: NodeId) -> Option<Vec<NodeId>> {
        let mut dist = HashMap::new();
        let mut prev = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();

        for &node in self.adjacency_list.keys() {
            dist.insert(node, usize::MAX);
            prev.insert(node, None);
        }

        dist.insert(start, 0);
        heap.push(State {
            cost: 0,
            node: start,
        });

        while let Some(State { cost, node }) = heap.pop() {
            if node == end {
                let mut path = Vec::new();
                let mut current = node;
                while let Some(prev_node) = prev.get(&current).unwrap() {
                    path.push(current);
                    current = *prev_node;
                }
                path.push(start);
                path.reverse();
                return Some(path);
            }

            if visited.contains(&node) {
                continue;
            }

            visited.insert(node);

            for &(neighbor, weight) in &self.adjacency_list[&node] {
                let new_cost = cost + weight;
                if new_cost < *dist.get(&neighbor).unwrap() {
                    dist.insert(neighbor, new_cost);
                    prev.insert(neighbor, Some(node));
                    heap.push(State {
                        cost: new_cost,
                        node: neighbor,
                    });
                }
            }
        }

        None
    }
}

fn read_facebook_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("/Users/danieldoh/Desktop/final_210/facebook_combined.txt")?;
    let reader = BufReader::new(file);

    let mut graph = Graph::new();
    for line in reader.lines() {
        let line = line?;
        let values: Vec<&str> = line.split_whitespace().collect();
        let u: NodeId = values[0].parse()?;
        let v: NodeId = values[1].parse()?;
        graph.add_edge(u, v, 1);
    }

    let path = graph.get_shortest_path(0, 5);
    println!("{:?}", path); // Some([0, 2, 4, 5])

    Ok(())
}

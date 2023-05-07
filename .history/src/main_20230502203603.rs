use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type NodeId = usize;
type Edge = (NodeId, NodeId);

struct Graph {
    adjacency_list: HashMap<NodeId, HashSet<NodeId>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: NodeId, v: NodeId) {
        self.adjacency_list
            .entry(u)
            .or_insert(HashSet::new())
            .insert(v);
        self.adjacency_list
            .entry(v)
            .or_insert(HashSet::new())
            .insert(u);
    }

    fn get_shortest_path(&self, u: NodeId, v: NodeId) -> Option<Vec<NodeId>> {
        let mut visited = HashSet::new();
        let mut queue = vec![vec![u]];
        while let Some(path) = queue.pop() {
            let node = *path.last().unwrap();
            if node == v {
                return Some(path);
            }
            if !visited.insert(node) {
                continue;
            }
            for neighbor in self.adjacency_list.get(&node).unwrap() {
                if visited.contains(neighbor) {
                    continue;
                }
                let mut new_path = path.clone();
                new_path.push(*neighbor);
                queue.push(new_path);
            }
        }
        None
    }
}

fn read_facebook_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut graph = Graph::new();
    for line in reader.lines() {
        let line = line?;
        let values: Vec<&str> = line.split_whitespace().collect();
        let u: NodeId = values[0].parse()?;
        let v: NodeId = values[1].parse()?;
        graph.add_edge(u, v);
    }

    let path = graph.get_shortest_path(0, 5);
    println!("{:?}", path); // Some([0, 2, 4, 5])

    Ok(())
}

fn main() {
    read_facebook_file("facebook_combined.txt").unwrap();
}

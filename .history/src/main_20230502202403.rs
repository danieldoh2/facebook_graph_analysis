use std::collections::{HashMap, HashSet};

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

fn main() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(3, 4);
    graph.add_edge(4, 5);

    let path = graph.get_shortest_path(0, 5);
    println!("{:?}", path); // Some([0, 2, 4, 5])
}

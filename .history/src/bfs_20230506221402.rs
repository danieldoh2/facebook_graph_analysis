use std::collections::{HashMap, VecDeque};

pub fn bfs(start: usize, graph: &Vec<Vec<(usize, i32)>>) -> HashMap<usize, i32> {
    let mut queue = VecDeque::new();
    let mut dist = HashMap::new();
    queue.push_back(start);
    dist.insert(start, 0);
    while let Some(node) = queue.pop_front() {
        for &(neigh, weight) in &graph[node] {
            if !dist.contains_key(&neigh) {
                dist.insert(neigh, dist[&node] + weight);
                queue.push_back(neigh);
            }
        }
    }
    dist
}

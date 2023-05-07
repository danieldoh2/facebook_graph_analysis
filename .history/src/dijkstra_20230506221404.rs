use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i32)>>) -> HashMap<usize, i32> {
    let mut dist: HashMap<usize, i32> = HashMap::new();

    for i in 0..graph.len() {
        dist.insert(i, std::i32::MAX);
    }

    dist.insert(start, 0);

    let mut pq: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
    pq.push((Reverse(0), start));

    while let Some((Reverse(cur_dist), cur_node)) = pq.pop() {
        if cur_dist > dist[&cur_node] {
            continue;
        }

        for (neighbor, weight) in &graph[cur_node] {
            let new_dist = cur_dist + weight;
            if new_dist < dist[neighbor] {
                dist.insert(*neighbor, new_dist);
                pq.push((Reverse(new_dist), *neighbor));
            }
        }
    }

    dist
}

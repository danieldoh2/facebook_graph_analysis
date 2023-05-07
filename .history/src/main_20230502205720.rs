use std::collections::{BinaryHeap, HashMap};
use std::convert::TryInto;

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

fn main() {
    let graph = vec![
        vec![(1, 7), (2, 9), (5, 14)],
        vec![(0, 7), (2, 10), (3, 15)],
        vec![(0, 9), (1, 10), (3, 11), (5, 2)],
        vec![(1, 15), (2, 11), (4, 6)],
        vec![(3, 6), (5, 9)],
        vec![(0, 14), (2, 2), (4, 9)],
    ];
    let dist = dijkstra(0, &graph);
    println!("{:?}", dist);
}

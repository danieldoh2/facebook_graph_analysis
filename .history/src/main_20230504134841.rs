use std::collections::VecDeque;
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

fn bfs(start: usize, graph: &Vec<Vec<usize>>) -> HashMap<usize, usize> {
    let mut dist = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    dist.insert(start, 0);

    while let Some(node) = queue.pop_front() {
        let current_dist = dist[&node];

        for &neighbor in &graph[node] {
            if !dist.contains_key(&neighbor) {
                dist.insert(neighbor, current_dist + 1);
                queue.push_back(neighbor);
            }
        }
    }

    dist
}

fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i32)>>) -> HashMap<usize, usize> {
    let mut dist: HashMap<usize, usize> = HashMap::new();
    let mut pq: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    dist.insert(start, 0);
    pq.push((Reverse(0), start));
    while !pq.is_empty() {
        let (Reverse(cur_dist), cur_node) = pq.pop().unwrap();
        if let Some(neighbors) = graph.get(cur_node) {
            for (neighbor, weight) in neighbors {
                let new_dist = cur_dist + weight;
                if new_dist < *dist.get(neighbor).unwrap_or(&usize::MAX) {
                    dist.insert(*neighbor, new_dist);
                    pq.push((Reverse(new_dist), *neighbor));
                }
            }
        }
    }
    dist
}

fn bfs_transfer(filename: &str) -> Vec<Vec<usize>> {
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
        graph[node1].push(node2);
        graph[node2].push(node1);
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

fn check_six_degrees_of_separation(output: &HashMap<usize, i32>) -> bool {
    for i in 0..output.len() {
        for j in 0..output.len() {
            if i != j && output.get(&i).unwrap() + output.get(&j).unwrap() == 6 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let graph: Vec<Vec<(usize, i32)>> =
        djikstra_transfer("/Users/danieldoh/Desktop/facebook_combined.txt");
    let dist: HashMap<usize, usize> = dijkstra(0, &graph);
    // println!("{:?}", dist);
    println!("{}", check_six_degrees_of_separation(&dist));

    // let graph: Vec<Vec<(usize, i32)>> = bfs_transfer("/Users/danieldoh/Desktop/facebook_combined.txt");
    // let dist = bfs(0, &graph);
    // println!("{:?}", dist);
}

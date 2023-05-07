use std::cmp::Reverse;
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

fn bfs(start: usize, graph: &Vec<Vec<(usize, i32)>>) -> HashMap<usize, i32> {
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

fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i32)>>) -> HashMap<usize, i32> {
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

fn bfs_transfer(file_path: &str) -> Vec<Vec<(usize, i32)>> {
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
    for val in output.values() {
        if *val < 6 {
            return true;
        }
    }

    false
}

fn main() {
    // let graph: Vec<Vec<(usize, i32)>> =
    //     djikstra_transfer("/Users/danieldoh/Desktop/facebook_combined.txt");
    // let dist: HashMap<usize, i32> = dijkstra(0, &graph);
    // println!("{:?}", dist);
    // println!("{}", check_six_degrees_of_separation(&dist));

    let graph: Vec<Vec<(usize, i32)>> =
        bfs_transfer("/Users/danieldoh/Desktop/facebook_combined.txt");
    let dist = bfs(0, &graph);
    println!("{:?}", dist);
    // println!("{}", check_six_degrees_of_separation(&dist));
}

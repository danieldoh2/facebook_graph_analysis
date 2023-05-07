// use final_210::bfs;
// use final_210::dijkstra;
use final_210::bfs::bfs;
use final_210::dijkstra::dijkstra;

#[test]
fn test_dijkstra() {
    let graph = vec![
        vec![(1, 1), (2, 4)],
        vec![(2, 2), (3, 5)],
        vec![(3, 1)],
        vec![],
    ];

    let dist = dijkstra(0, &graph);
    assert_eq!(dist.get(&0), Some(&0));
    assert_eq!(dist.get(&1), Some(&1));
    assert_eq!(dist.get(&2), Some(&3));
    assert_eq!(dist.get(&3), Some(&4));
}

#[test]
fn test_bfs() {
    let graph = vec![
        vec![(1, 1), (2, 4)],
        vec![(2, 2), (3, 5)],
        vec![(3, 1)],
        vec![],
    ];

    let dist = bfs(0, &graph);
    assert_eq!(bfs(0, &graph), Some(2));
    // assert_eq!(dist.get(&0), Some(&0));
    // assert_eq!(dist.get(&1), Some(&1));
    // assert_eq!(dist.get(&2), Some(&3));
    // assert_eq!(dist.get(&3), Some(&4));
}
#[test]
fn main() {
    test_dijkstra();
    test_bfs();
}

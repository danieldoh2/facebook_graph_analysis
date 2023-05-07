#[cfg(test)]
mod dijikstra
mod bfs
mod tests {
    use super::*;

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
        assert_eq!(dist.get(&0), Some(&0));
        assert_eq!(dist.get(&1), Some(&1));
        assert_eq!(dist.get(&2), Some(&3));
        assert_eq!(dist.get(&3), Some(&4));
    }
}

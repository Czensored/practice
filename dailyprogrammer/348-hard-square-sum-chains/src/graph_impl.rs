use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
// Not my implementation, yoinked from the python implementation in this comment:
// https://www.reddit.com/r/dailyprogrammer/comments/7t6fnc/20180126_challenge_348_hard_square_sum_chains/dtaxd5w/
// Is very fast on some values, but still very slow on others
pub fn square_sum(n: usize) -> Option<Vec<usize>> {
    let maxsq = isqrt(n + (n - 1));
    let squares = generate_squares(maxsq);

    // Build graph
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for u in 1..=n {
        let neighbors: Vec<usize> = (1..=n)
            .filter(|&v| v != u && squares.contains(&(u + v)))
            .collect();
        graph.insert(u, neighbors);
    }

    // Sort nodes by ascending degree
    let mut nodes: Vec<usize> = (1..=n).collect();
    nodes.sort_by_key(|&u| graph[&u].len());

    // Sort each adjacency list by descending degree
    for u in &nodes {
        let mut adj = graph[u].clone();
        adj.sort_by_key(|&v| usize::MAX - graph[&v].len()); // reverse order
        graph.insert(*u, adj);
    }

    // DFS stack: each entry is (path, visited set)
    let mut stack: Vec<(Vec<usize>, HashSet<usize>)> = nodes
        .iter()
        .map(|&n| (vec![n], [n].iter().cloned().collect()))
        .collect();

    while let Some((path, visited)) = stack.pop() {
        if path.len() == n {
            return Some(path);
        }

        if let Some(neighbors) = graph.get(path.last().unwrap()) {
            for &u in neighbors {
                if !visited.contains(&u) {
                    let mut new_path = path.clone();
                    new_path.push(u);
                    let mut new_visited = visited.clone();
                    new_visited.insert(u);
                    stack.push((new_path, new_visited));
                }
            }
        }
    }

    None
}

fn isqrt(n: usize) -> usize {
    (n as f64).sqrt() as usize
}

fn generate_squares(limit: usize) -> HashSet<usize> {
    let mut squares = HashSet::new();
    for i in 2..=limit {
        squares.insert(i * i);
    }
    squares
}

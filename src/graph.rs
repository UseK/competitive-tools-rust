/// Directed Acyclic Graph
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dag {
    /// The number of Vertex
    pub n: usize,
    /// adjacency_list\[i\] returns edges from i vertex
    pub adjacency_list: Vec<Vec<usize>>,
}

impl Dag {
    /// Returns vertexes in topologically sorted order
    /// ```
    /// use competitive_tools_rust::graph::Dag;
    /// let dag = Dag { n: 3, adjacency_list: vec![vec![], vec![2], vec![0]] };
    /// assert_eq!(dag.topological_sort(), vec![1, 2, 0]);
    /// ```
    pub fn topological_sort(&self) -> Vec<usize> {
        let mut seen: Vec<bool> = vec![false; self.n];
        let mut rev_order: Vec<usize> = vec![];
        for i in 0..self.n {
            if !seen[i] {
                self.dfs(&mut seen, i, &mut rev_order);
            }
        }
        rev_order.into_iter().rev().collect()
    }

    fn dfs(&self, seen: &mut Vec<bool>, i: usize, rev_order: &mut Vec<usize>) {
        seen[i] = true;
        self.adjacency_list[i].iter().for_each(|&to| {
            if !seen[to] {
                self.dfs(seen, to, rev_order);
            }
        });
        rev_order.push(i);
    }
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Edge {
    pub to: usize,
    pub cost: usize,
}

/// (distance, from_path)
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Pair(usize, usize);

pub fn dijkstra(s: usize, max_v: usize, edges_list: &[Vec<Edge>]) -> Vec<Option<usize>> {
    let mut que = BinaryHeap::new();
    let mut min_dists: Vec<Option<usize>> = (0..max_v).map(|_| None).collect();
    min_dists[s] = Some(0);
    que.push(Reverse(Pair(0, s)));
    while let Some(Reverse(p)) = que.pop() {
        let v = p.1;
        if min_dists[v].unwrap() < p.0 {
            continue;
        };
        for e in &edges_list[v] {
            let candidate_dist = min_dists[v].unwrap() + e.cost;
            if candidate_dist < min_dists[e.to].unwrap_or(usize::MAX) {
                min_dists[e.to] = Some(candidate_dist);
                que.push(Reverse(Pair(candidate_dist, e.to)));
            }
        }
    }
    min_dists
}

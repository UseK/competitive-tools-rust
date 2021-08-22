/// Directed Acyclic Graph
pub trait Dag {
    /// Returns vertexes in topologically sorted order
    fn topological_sort(&self) -> Vec<usize>;
}

/// Vec<Vec<usize>> as Adjacency List
/// self\[i\] returns edges from i vertex
/// The number of Vertex is self.len()
impl Dag for Vec<Vec<usize>> {
    /// Returns vertexes in topologically sorted order
    /// ```
    /// use competitive_tools_rust::graph::Dag;
    /// // 1 -> 2 -> 0
    /// let dag = vec![vec![], vec![2], vec![0]];
    /// assert_eq!(dag.topological_sort(), vec![1, 2, 0]);
    /// ```
    fn topological_sort(&self) -> Vec<usize> {
        /// post-order Depth First Search
        fn dfs(me: &[Vec<usize>], seen: &mut Vec<bool>, i: usize, rev_order: &mut Vec<usize>) {
            seen[i] = true;
            me[i].iter().for_each(|&to| {
                if !seen[to] {
                    dfs(me, seen, to, rev_order);
                }
            });
            rev_order.push(i);
        }
        let mut seen: Vec<bool> = vec![false; self.len()];
        let mut rev_order: Vec<usize> = vec![];
        for i in 0..self.len() {
            if !seen[i] {
                dfs(self, &mut seen, i, &mut rev_order);
            }
        }
        rev_order.into_iter().rev().collect()
    }
}

pub trait AdjacencyList {
    fn rev_edge_direction(&self) -> Self;
    fn from_atcoder_tuples(n: usize, tuples: &[(usize, usize)]) -> Self;
    fn strongly_connected_component(&self) -> Vec<usize>;
}

impl AdjacencyList for Vec<Vec<usize>> {
    /// Reverse the direction of all edges
    /// ```
    /// use competitive_tools_rust::graph::AdjacencyList;
    /// // 0 <=> 1 => 2 <- 3
    /// // ^               ^
    /// // ||=============||
    /// let adjacency_list = vec![
    ///     vec![1, 3],
    ///     vec![0, 2, 2],
    ///     vec![],
    ///     vec![2, 0],
    /// ];
    /// // 0 <=> 1 <= 2 -> 3
    /// // ^               ^
    /// // ||=============||
    /// assert_eq!(adjacency_list.rev_edge_direction(), vec![
    ///     vec![1, 3],
    ///     vec![0],
    ///     vec![1, 1, 3],
    ///     vec![0],
    /// ]);
    ///
    /// ```
    fn rev_edge_direction(&self) -> Self {
        let mut reversed: Self = vec![vec![]; self.len()];
        self.iter()
            .enumerate()
            .for_each(|(ind, edges)| edges.iter().for_each(|&to| reversed[to].push(ind)));
        reversed
    }

    /// ```
    /// use competitive_tools_rust::graph::AdjacencyList;
    /// let edges = vec![(1, 2), (2, 1), (2, 3), (4, 3), (4, 1), (1, 4), (2, 3)];
    /// let adjacency_list: Vec<Vec<usize>> = AdjacencyList::from_atcoder_tuples(4, &edges);
    /// let expected: Vec<Vec<usize>> = vec![
    ///     vec![1, 3],
    ///     vec![0, 2, 2],
    ///     vec![],
    ///     vec![2, 0],
    /// ];
    /// assert_eq!(adjacency_list, expected);
    /// ```
    fn from_atcoder_tuples(n: usize, tuples: &[(usize, usize)]) -> Self {
        tuples.iter().fold(vec![vec![]; n], |mut acc, edge| {
            acc[edge.0 - 1].push(edge.1 - 1);
            acc
        })
    }

    /// ```
    /// use competitive_tools_rust::graph::AdjacencyList;
    /// // 0 <=> 1 => 2 <- 3
    /// // ^               ^
    /// // ||=============||
    /// let adjacency_list: Vec<Vec<usize>> = vec![
    ///     vec![1, 3],
    ///     vec![0, 2, 2],
    ///     vec![],
    ///     vec![2, 0],
    /// ];
    /// let scc_order = adjacency_list.strongly_connected_component();
    /// // [0, 1, 3] -> [2] in SCC order
    /// assert_eq!(scc_order, vec![0, 0, 1, 0]);
    /// ```
    fn strongly_connected_component(&self) -> Vec<usize> {
        fn dfs(
            start_id: usize,
            graph: &[Vec<usize>],
            seen: &mut Vec<bool>,
            order: &mut [usize],
            scc_id: usize,
        ) {
            seen[start_id] = true;
            order[start_id] = scc_id;
            for &to in &graph[start_id] {
                if !seen[to] {
                    dfs(to, graph, seen, order, scc_id);
                }
            }
        }
        let topological_order = self.topological_sort();
        let reversed = self.rev_edge_direction();
        let mut order = vec![0; self.len()];
        let mut seen: Vec<bool> = vec![false; self.len()];
        let mut scc_id: usize = 0;
        for start_id in topological_order {
            if !seen[start_id] {
                dfs(start_id, &reversed, &mut seen, &mut order, scc_id);
                scc_id += 1;
            }
        }
        order
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{AdjacencyList, Dag};

    #[test]
    fn test_topological_sort_for_cyclic() {
        let n = 4;
        let edges = vec![(1, 2), (2, 1), (2, 3), (4, 3), (4, 1), (1, 4), (2, 3)];
        let adjacency_list: Vec<Vec<usize>> = AdjacencyList::from_atcoder_tuples(n, &edges);
        assert_eq!(adjacency_list.topological_sort(), vec![0, 3, 1, 2]);
    }

    #[test]
    fn test_topological_sort_for_cyclic2() {
        let n = 3;
        let edges = vec![(1, 2), (2, 3), (3, 1)];
        let adjacency_list: Vec<Vec<usize>> = AdjacencyList::from_atcoder_tuples(n, &edges);
        assert_eq!(adjacency_list.topological_sort(), vec![0, 1, 2]);
    }

    #[test]
    fn test_topological_sort_for_one_edge() {
        let n = 4;
        let edges = vec![(2, 1)];
        let adjacency_list: Vec<Vec<usize>> = AdjacencyList::from_atcoder_tuples(n, &edges);
        assert_eq!(adjacency_list.topological_sort(), vec![3, 2, 1, 0]);
    }

    fn scc_cmp(n: usize, edges: Vec<(usize, usize)>, result: Vec<usize>) {
        let adjacency_list: Vec<Vec<usize>> = AdjacencyList::from_atcoder_tuples(n, &edges);
        assert_eq!(adjacency_list.strongly_connected_component(), result,);
    }

    #[test]
    fn test_strongly_connected_component_for_one_hub() {
        scc_cmp(
            4,
            vec![(1, 2), (2, 1), (1, 3), (3, 1), (1, 4)],
            vec![0, 0, 0, 1],
        );
        scc_cmp(
            4,
            vec![(1, 2), (2, 1), (1, 3), (3, 1), (4, 1)],
            vec![1, 1, 1, 0],
        );
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

pub trait BipartiteGraph {
    fn bi_partition(&self) -> (Vec<usize>, Vec<usize>);
}

impl BipartiteGraph for Vec<Vec<usize>> {
    ///
    /// Partition bipartite graph into two vertex ids.
    /// ```
    /// use competitive_tools_rust::graph::BipartiteGraph;
    /// // 0
    /// // |
    /// // 1--3
    /// // |
    /// // 2
    /// let adjacency_list: Vec<Vec<usize>> = vec![
    ///     vec![1],
    ///     vec![0, 2, 3],
    ///     vec![1],
    ///     vec![1],
    /// ];
    /// assert_eq!(adjacency_list.bi_partition(), (vec![0, 2, 3], vec![1]));
    /// // 0
    /// // |
    /// // 2--5
    /// // |
    /// // 4
    /// // |
    /// // 1
    /// // |
    /// // 3
    /// let adjacency_list: Vec<Vec<usize>> = vec![
    ///     vec![2],
    ///     vec![3, 4],
    ///     vec![0, 4, 5],
    ///     vec![1],
    ///     vec![1, 2],
    ///     vec![2],
    /// ];
    /// assert_eq!(adjacency_list.bi_partition(), (vec![0, 3, 4, 5], vec![1, 2]));
    /// ```
    fn bi_partition(&self) -> (Vec<usize>, Vec<usize>) {
        let mut current: Vec<Option<bool>> = vec![None; self.len()];
        let mut stack: Vec<(usize, bool)> = vec![];
        current[0] = Some(true);
        stack.append(&mut self[0].iter().map(|&i| (i, false)).collect());

        while let Some((vertex_id, bi)) = stack.pop() {
            current[vertex_id] = Some(bi);
            let nexts = self[vertex_id]
                .iter()
                .filter(|&&i| current[i].is_none())
                .map(|&i| (i, !bi));
            stack.append(&mut nexts.collect());
        }
        let mut x: Vec<usize> = vec![];
        let mut y: Vec<usize> = vec![];
        for (ind, bi) in current.iter().enumerate() {
            if bi.unwrap() {
                x.push(ind);
            } else {
                y.push(ind);
            }
        }
        (x, y)
    }
}

/// Directed Acyclic Graph
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DAG {
    /// The number of Vertex
    pub n: usize,
    /// adjacency_list[i] returns edges from i vertex
    pub adjacency_list: Vec<Vec<usize>>,
}

impl DAG {
    /// Returns vertexes in topologically sorted order
    /// ```
    /// use competitive_tools_rust::graph::DAG;
    /// let dag = DAG { n: 3, adjacency_list: vec![vec![], vec![2], vec![0]] };
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

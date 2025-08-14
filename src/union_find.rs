pub struct UnionFindTree {
    parents: Vec<usize>,
}

impl UnionFindTree {
    pub fn new(n: usize) -> Self {
        let parents = (0..n).collect();
        UnionFindTree { parents }
    }

    /// Get root with compressing path
    /// ```
    /// use competitive_tools_rust::union_find::UnionFindTree;
    /// let mut tree = UnionFindTree::new(3);
    /// assert_eq!(tree.root(0), 0);
    /// assert_eq!(tree.root(1), 1);
    /// assert_eq!(tree.root(2), 2);
    /// ```
    pub fn root(&mut self, x: usize) -> usize {
        let mut paths = Vec::new();
        let mut current_x = x;
        loop {
            if current_x == self.parents[current_x] {
                break;
            } else {
                paths.push(current_x);
                current_x = self.parents[current_x];
            }
        }
        for i in paths {
            // compress path
            self.parents[i] = current_x;
        }
        current_x
    }

    /// Check if two nodes are in the same set
    /// ```
    /// use competitive_tools_rust::union_find::UnionFindTree;
    /// let mut tree = UnionFindTree::new(3);
    /// assert!(!tree.is_same(0, 1));
    /// assert!(!tree.is_same(1, 2));
    /// assert!(!tree.is_same(0, 2));
    /// tree.unite(0, 1);
    /// assert!(tree.is_same(0, 1));
    /// assert!(!tree.is_same(1, 2));
    /// assert!(!tree.is_same(0, 2));
    /// tree.unite(1, 2);
    /// assert!(tree.is_same(0, 1));
    /// assert!(tree.is_same(1, 2));
    /// assert!(tree.is_same(0, 2));
    /// ```
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// Unite two nodes
    /// ```
    /// use competitive_tools_rust::union_find::UnionFindTree;
    /// let mut tree = UnionFindTree::new(3);
    /// tree.unite(0, 2);
    /// assert_eq!(tree.root(0), tree.root(2));
    /// ```
    pub fn unite(&mut self, x: usize, y: usize) {
        let root_x = self.root(x);
        let root_y = self.root(y);
        if root_x != root_y {
            self.parents[root_x] = root_y;
        }
    }
}

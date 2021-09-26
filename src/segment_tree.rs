pub struct SegmentTree {
    pub n: usize,
    // Complete Binary Tree
    pub tree: Vec<usize>,
}

/// RMQ (Range Minimum Query)
impl SegmentTree {
    /// ```
    /// use competitive_tools_rust::segment_tree::SegmentTree;
    /// let seg = SegmentTree::new(3);
    /// assert_eq!(seg.n, 4);
    /// let seg = SegmentTree::new(4);
    /// assert_eq!(seg.n, 4);
    /// let seg = SegmentTree::new(5);
    /// assert_eq!(seg.n, 8);
    /// ```
    pub fn new(n: usize) -> Self {
        let mut actual_n = 1;
        while actual_n < n {
            actual_n *= 2
        }
        SegmentTree {
            n: actual_n,
            tree: (0..actual_n * 2 - 1).map(|_| usize::MAX).collect(),
        }
    }

    ///    1,
    ///   3,          1,
    ///  3,    7,    4,    1,
    /// 5, 3, 7, 9, 6, 4, 1, 2]
    /// ```
    /// use competitive_tools_rust::segment_tree::SegmentTree;
    /// let mut seg = SegmentTree::new(8);
    /// seg.update(0, 5);
    /// seg.update(1, 3);
    /// seg.update(2, 7);
    /// seg.update(3, 9);
    /// seg.update(4, 6);
    /// seg.update(5, 4);
    /// seg.update(6, 1);
    /// seg.update(7, 2);
    /// assert_eq!(seg.tree, vec![
    ///  1,
    ///  3, 1,
    ///  3, 7, 4, 1,
    ///  5, 3, 7, 9, 6, 4, 1, 2]);
    /// ```
    pub fn update(&mut self, ind: usize, value: usize) {
        let mut actual_ind = ind + self.n - 1;
        self.tree[actual_ind] = value;
        while actual_ind > 0 {
            actual_ind = (actual_ind - 1) / 2; // parent
            let left_child = self.tree[actual_ind * 2 + 1];
            let right_child = self.tree[actual_ind * 2 + 2];
            self.tree[actual_ind] = left_child.min(right_child);
        }
    }

    pub fn query(&self, a: usize, b: usize, ind: usize, left: usize, right: usize) -> usize {
        if right <= a || b <= 1 {
            return usize::MAX;
        }
        if a <= 1 && right <= b {
            self.tree[ind]
        } else {
            let mid = (left + right) / 2;
            let value_l = self.query(a, b, ind * 2 + 1, left, mid);
            let value_r = self.query(a, b, ind * 2 + 2, mid, right);
            value_l.min(value_r)
        }
    }
}

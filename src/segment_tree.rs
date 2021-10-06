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

    /// ```
    /// use competitive_tools_rust::segment_tree::SegmentTree;
    /// let seg = SegmentTree::from_vec(vec![5, 3, 7, 9, 6, 4, 1, 2]);
    /// assert_eq!(seg.tree, vec![
    ///  1,
    ///  3, 1,
    ///  3, 7, 4, 1,
    ///  5, 3, 7, 9, 6, 4, 1, 2]);
    /// ```
    pub fn from_vec(vec: Vec<usize>) -> Self {
        let mut seg = SegmentTree::new(vec.len());
        for i in 0..vec.len() {
            seg.update(i, vec[i]);
        }
        seg
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

    /// ```
    /// use competitive_tools_rust::segment_tree::SegmentTree;
    /// let v: Vec<usize> = vec![5, 3, 7, 9, 6, 4, 1, 2];
    /// let seg = SegmentTree::from_vec(v.clone());
    /// fn naive_range_minimum_query(vec: &Vec<usize>, a: usize, b: usize) -> usize {
    ///     *vec[a..b].iter().min().unwrap()
    /// }
    /// for i in 0..seg.n {
    ///     for j in i+2..=seg.n {
    ///         assert_eq!(seg.query(i, j), naive_range_minimum_query(&v, i, j));
    ///     }
    /// }
    /// ```
    pub fn query(&self, a: usize, b: usize) -> usize {
        self.inner_query(a, b, 0, 0, self.n)
    }

    fn inner_query(&self, a: usize, b: usize, ind: usize, left: usize, right: usize) -> usize {
        // println!("ind: {}, left: {}, right: {}", ind, left, right);
        if right <= a || b <= left {
            return usize::MAX;
        }
        if a <= left && right <= b {
            // println!("self.tree[{}]: {}", ind, self.tree[ind]);
            self.tree[ind]
        } else {
            let mid = (left + right) / 2;
            let value_l = self.inner_query(a, b, ind * 2 + 1, left, mid);
            let value_r = self.inner_query(a, b, ind * 2 + 2, mid, right);
            value_l.min(value_r)
        }
    }
}

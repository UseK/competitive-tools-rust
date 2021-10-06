pub struct SegmentTree<T>
where
    T: core::cmp::Ord,
{
    pub n: usize,
    // Complete Binary Tree
    pub tree: Vec<T>,
    max_item: T,
}

/// RMQ (Range Minimum Query)
impl<T> SegmentTree<T>
where
    T: core::cmp::Ord,
    T: Copy,
{
    /// ```
    /// use competitive_tools_rust::segment_tree::SegmentTree;
    /// let seg = SegmentTree::new(3, usize::MAX);
    /// assert_eq!(seg.n, 4);
    /// let seg = SegmentTree::new(4, usize::MAX);
    /// assert_eq!(seg.n, 4);
    /// let seg = SegmentTree::new(5, usize::MAX);
    /// assert_eq!(seg.n, 8);
    /// ```
    pub fn new(n: usize, max_item: T) -> Self {
        let mut actual_n = 1;
        while actual_n < n {
            actual_n *= 2
        }
        SegmentTree {
            n: actual_n,
            tree: (0..actual_n * 2 - 1).map(|_| max_item).collect(),
            max_item,
        }
    }

    /// ```
    /// use competitive_tools_rust::segment_tree::SegmentTree;
    /// let seg = SegmentTree::from_vec(vec![5, 3, 7, 9, 6, 4, 1, 2], usize::MAX);
    /// assert_eq!(seg.tree, vec![
    ///  1,
    ///  3, 1,
    ///  3, 7, 4, 1,
    ///  5, 3, 7, 9, 6, 4, 1, 2]);
    /// ```
    pub fn from_vec(vec: Vec<T>, max_item: T) -> Self {
        let mut seg = SegmentTree::new(vec.len(), max_item);
        (0..vec.len()).for_each(|i| {
            seg.update(i, vec[i]);
        });
        seg
    }

    ///    1,
    ///   3,          1,
    ///  3,    7,    4,    1,
    /// 5, 3, 7, 9, 6, 4, 1, 2]
    /// ```
    /// use competitive_tools_rust::segment_tree::SegmentTree;
    /// let mut seg = SegmentTree::new(8, usize::MAX);
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
    pub fn update(&mut self, ind: usize, value: T) {
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
    /// let seg = SegmentTree::from_vec(v.clone(), usize::MAX);
    /// fn naive_range_minimum_query(vec: &Vec<usize>, a: usize, b: usize) -> usize {
    ///     *vec[a..b].iter().min().unwrap()
    /// }
    /// for i in 0..seg.n {
    ///     for j in i+2..=seg.n {
    ///         assert_eq!(seg.query(i, j), naive_range_minimum_query(&v, i, j));
    ///     }
    /// }
    ///
    /// let char_vec: Vec<char> = vec!['b', 'c', 'a', 'd'];
    /// let char_seg = SegmentTree::from_vec(char_vec.clone(), char::MAX);
    /// assert_eq!(char_seg.query(0, 1), 'b');
    /// assert_eq!(char_seg.query(0, 2), 'b');
    /// assert_eq!(char_seg.query(1, 4), 'a');
    /// assert_eq!(char_seg.query(0, 4), 'a');
    /// ```
    pub fn query(&self, a: usize, b: usize) -> T {
        self.inner_query(a, b, 0, 0, self.n)
    }

    fn inner_query(&self, a: usize, b: usize, ind: usize, left: usize, right: usize) -> T {
        // println!("ind: {}, left: {}, right: {}", ind, left, right);
        if right <= a || b <= left {
            return self.max_item;
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

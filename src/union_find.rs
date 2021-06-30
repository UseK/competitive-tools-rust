pub struct UnionFindTree {
    parents: Vec<usize>,
}

impl UnionFindTree {
    pub fn new(n: usize) -> Self {
        let parents = (0..n).collect();
        UnionFindTree { parents }
    }

    /// Get root with compressing path
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

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let root_x = self.root(x);
        let root_y = self.root(y);
        if root_x != root_y {
            self.parents[root_x] = root_y;
        }
    }
}

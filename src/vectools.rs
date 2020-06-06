pub trait Vectools {
    fn keep(&mut self, bools: &Vec<bool>);
}

impl<T> Vectools for Vec<T> {
    /// ```
    /// use competitive_tools_rust::vectools::Vectools;
    /// let mut v = vec![1, 2, 3, 4, 5];
    /// let bools = vec![false, true, true, false, true];
    /// v.keep(&bools);
    /// assert_eq!(v, vec![2, 3, 5]);
    /// ```
    fn keep(&mut self, bools: &Vec<bool>) {
        let mut i = 0;
        self.retain(|_| (bools[i], i +=1).0)
    }
}

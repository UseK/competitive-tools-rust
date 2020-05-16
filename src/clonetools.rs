pub trait ClonePush<T> {
    fn clone_push(&self, v: T) -> Self;
}

impl<T: Clone> ClonePush<T> for Vec<T> {
    /// Clone and push(Appends an element to the back of a collection.)
    ///
    /// # Examples
    ///
    /// ```
    /// use competitive_tools_rust::clonetools::ClonePush;
    /// let v = vec![2, 5, 8];
    /// let new_v = v.clone_push(1);
    /// assert_eq!(new_v, vec![2, 5, 8, 1]);
    /// ```
    fn clone_push(&self, v: T) -> Self {
        let mut new_vec = self.clone();
        new_vec.push(v);
        new_vec
    }
}

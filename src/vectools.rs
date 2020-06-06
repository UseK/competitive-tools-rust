use std::collections::HashMap;

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

/// ```
/// use competitive_tools_rust::vectools::count_items;
/// use std::collections::HashMap;
/// let v = [2, 5, 8, 5];
/// let mut expected = HashMap::new();
/// expected.insert(2, 1);
/// expected.insert(5, 2);
/// expected.insert(8, 1);
/// assert_eq!(count_items(&v), expected);
/// ```
pub fn count_items<T>(v: &[T]) -> HashMap<T, usize>
    where T: std::cmp::Eq + std::hash::Hash + Copy
{
    v.iter().fold(HashMap::new(),|mut counter, &item| {
        *counter.entry(item).or_insert(0) += 1;
        counter
    })
}

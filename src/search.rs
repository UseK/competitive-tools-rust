use std::cmp::Ordering;

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    ///
    /// ```
    /// use competitive_tools_rust::search::BinarySearch;
    /// let v = vec![2, 3, 3, 5, 6];
    /// assert_eq!(v.lower_bound(&0), 0);
    /// assert_eq!(v.lower_bound(&1), 0);
    /// assert_eq!(v.lower_bound(&2), 0);
    /// assert_eq!(v.lower_bound(&3), 1);
    /// assert_eq!(v.lower_bound(&4), 3);
    /// assert_eq!(v.lower_bound(&5), 3);
    /// assert_eq!(v.lower_bound(&6), 4);
    /// assert_eq!(v.lower_bound(&7), 5);
    /// assert_eq!(v.lower_bound(&8), 5);
    /// ```
    ///
    fn lower_bound(&self, x: &T) -> usize {
        fn rec<T: Ord>(me: &[T], x: &T, left: usize, right: usize) -> usize {
            if left == right {
                left
            } else {
                let mid = (left + right) / 2;
                match me[mid].cmp(x) {
                    Ordering::Less => rec(me, x, mid + 1, right),
                    Ordering::Equal | Ordering::Greater => rec(me, x, left, mid),
                }
            }
        }
        rec(self, x, 0, self.len())
    }

    ///
    /// ```
    /// use competitive_tools_rust::search::BinarySearch;
    /// let v = vec![2, 3, 3, 5, 6];
    /// assert_eq!(v.upper_bound(&0), 0);
    /// assert_eq!(v.upper_bound(&1), 0);
    /// assert_eq!(v.upper_bound(&2), 1);
    /// assert_eq!(v.upper_bound(&3), 3);
    /// assert_eq!(v.upper_bound(&4), 3);
    /// assert_eq!(v.upper_bound(&5), 4);
    /// assert_eq!(v.upper_bound(&6), 5);
    /// assert_eq!(v.upper_bound(&7), 5);
    /// assert_eq!(v.upper_bound(&8), 5);
    /// ```
    ///
    fn upper_bound(&self, x: &T) -> usize {
        fn rec<T: Ord>(me: &[T], x: &T, left: usize, right: usize) -> usize {
            if left == right {
                left
            } else {
                let mid = (left + right) / 2;
                match me[mid].cmp(x) {
                    Ordering::Less | Ordering::Equal => rec(me, x, mid + 1, right),
                    Ordering::Greater => rec(me, x, left, mid),
                }
            }
        }
        rec(self, x, 0, self.len())
    }
}

///
/// Return pair of
/// ( maximum number i in F(i) = false,
///   minimum number j in F(j) = true)
/// ```
/// use competitive_tools_rust::search::bound;
/// assert_eq!(bound(0, 100, |i| i * i >= 48), (6, 7));
/// assert_eq!(bound(0, 100, |i| i * i >  48), (6, 7));
/// assert_eq!(bound(0, 100, |i| i * i >= 49), (6, 7));
/// assert_eq!(bound(0, 100, |i| i * i >  49), (7, 8));
/// assert_eq!(bound(0, 100, |i| i * i >= 50), (7, 8));
/// assert_eq!(bound(0, 100, |i| i * i >  50), (7, 8));
/// ```
pub fn bound<F>(ng_min: isize, ok_max: isize, condition: F) -> (isize, isize)
where
    F: Fn(isize) -> bool,
{
    let mut ng = ng_min;
    let mut ok = ok_max;
    while (ok - ng).abs() > 1 {
        let mid = (ng + ok) / 2;
        if condition(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    (ng, ok)
}

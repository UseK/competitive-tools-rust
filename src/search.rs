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

/// Return pair of
/// ( maximum number i in F(i) = false,
///   minimum number j in F(j) = true)
/// ```
/// use competitive_tools_rust::search::bound;
/// assert_eq!(bound(0, 100, |i| i * i >  48), (6, 7));
/// assert_eq!(bound(0, 100, |i| i * i >  49), (7, 8));
/// assert_eq!(bound(0, 100, |i| i * i >  50), (7, 8));
/// assert_eq!(bound(-100, -1, |i| i > -10), (-10, -9));
/// ```
pub fn bound<T, F>(ng_min: T, ok_max: T, condition: F) -> (T, T)
where
    T: num::Integer + num::FromPrimitive + Copy,
    F: Fn(T) -> bool,
{
    assert!(condition(ok_max));
    assert!(!condition(ng_min));
    let mut ng = ng_min;
    let mut ok = ok_max;
    while abs_diff(ok, ng) > T::one() {
        let mid = (ng + ok).div(T::from_u8(2).unwrap()); // expected type parameter `T`, found integer
        if condition(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    (ng, ok)
}

fn abs_diff<T>(src: T, dst: T) -> T
where
    T: num::Integer + num::FromPrimitive,
{
    let sub = if src > dst { src - dst } else { dst - src };
    if sub < T::zero() {
        sub * T::from_i8(-1).unwrap()
    } else {
        sub
    }
}

#[cfg(test)]
mod tests {
    use crate::search::bound;

    #[test]
    #[should_panic]
    fn test_bound_should_panic_when_not_ng_for_min() {
        bound(50, 100, |i| i * i > 50);
    }

    #[test]
    #[should_panic]
    fn test_bound_should_panic_when_not_ok_for_max() {
        bound(0, 5, |i| i * i > 50);
    }

    #[test]
    fn test_bound_for_any_integer() {
        // bound(0i8, 100i8, |i| i * i > 50i8);
        assert_eq!(bound(0usize, 100usize, |i| i * i > 50usize), (7, 8));
    }

    #[test]
    fn test_bound_when_minus() {
        assert_eq!(bound(-100, -1, |i| i > -10), (-10, -9));
        assert_eq!(bound(-100i128, -1i128, |i| i > -10i128), (-10i128, -9i128));
    }
}

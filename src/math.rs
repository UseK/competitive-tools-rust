/// ```
/// use competitive_tools_rust::math::is_prime;
/// assert_eq!(is_prime(0), false);
/// assert_eq!(is_prime(1), false);
/// assert_eq!(is_prime(2), true);
/// assert_eq!(is_prime(3), true);
/// assert_eq!(is_prime(4), false);
/// assert_eq!(is_prime(5), true);
/// assert_eq!(is_prime(6), false);
/// assert_eq!(is_prime(7), true);
/// assert_eq!(is_prime(8), false);
/// assert_eq!(is_prime(9), false);
/// assert_eq!(is_prime(1000000007), true);
/// assert_eq!(is_prime(997764507000), false);
/// ```
pub fn is_prime(n: usize) -> bool {
    if n < 2 { return false }
    min_prime_factor(n).is_none()
}

/// ```
/// use competitive_tools_rust::math::min_prime_factor;
/// assert_eq!(min_prime_factor(0), None);
/// assert_eq!(min_prime_factor(1), None);
/// assert_eq!(min_prime_factor(2), None);
/// assert_eq!(min_prime_factor(3), None);
/// assert_eq!(min_prime_factor(4), Some(2));
/// assert_eq!(min_prime_factor(5), None);
/// assert_eq!(min_prime_factor(6), Some(2));
/// assert_eq!(min_prime_factor(7), None);
/// assert_eq!(min_prime_factor(8), Some(2));
/// assert_eq!(min_prime_factor(9), Some(3));
/// assert_eq!(min_prime_factor(1000000007), None);
/// assert_eq!(min_prime_factor(997764507000), Some(2));
/// ```
pub fn min_prime_factor(n: usize) -> Option<usize> {
    if n < 4 { return None }
    let max_sqrt = (n as f64).sqrt().ceil() as usize;
    for i in 2..=max_sqrt {
        if n % i == 0 { return Some(i) }
    }
    None
}

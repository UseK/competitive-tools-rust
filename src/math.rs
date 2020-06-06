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
    match smallest_prime_factor(n) {
        Some(factor) => factor == n,
        None => false,
    }
}

/// ```
/// use competitive_tools_rust::math::prime_factors;
/// assert_eq!(prime_factors(0), vec![]);
/// assert_eq!(prime_factors(1), vec![]);
/// assert_eq!(prime_factors(2), vec![2]);
/// assert_eq!(prime_factors(24), vec![2, 2, 2, 3]);
/// assert_eq!(prime_factors(1000000007), vec![1000000007]);
/// assert_eq!(prime_factors(997764507000), vec![2, 2, 2, 3, 3, 3, 5, 5, 5, 6079, 6079]);
/// assert_eq!(prime_factors(997764507000), vec![2, 2, 2, 3, 3, 3, 5, 5, 5, 6079, 6079]);
/// ```
pub fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut factors = vec![];
    while let Some(i) = smallest_prime_factor(n) {
        factors.push(i);
        n = n / i;
    }
    factors
}

/// ```
/// use competitive_tools_rust::math::smallest_prime_factor;
/// assert_eq!(smallest_prime_factor(0), None);
/// assert_eq!(smallest_prime_factor(1), None);
/// assert_eq!(smallest_prime_factor(2), Some(2));
/// assert_eq!(smallest_prime_factor(3), Some(3));
/// assert_eq!(smallest_prime_factor(4), Some(2));
/// assert_eq!(smallest_prime_factor(5), Some(5));
/// assert_eq!(smallest_prime_factor(6), Some(2));
/// assert_eq!(smallest_prime_factor(7), Some(7));
/// assert_eq!(smallest_prime_factor(8), Some(2));
/// assert_eq!(smallest_prime_factor(9), Some(3));
/// assert_eq!(smallest_prime_factor(1000000007), Some(1000000007));
/// assert_eq!(smallest_prime_factor(997764507000), Some(2));
/// ```
pub fn smallest_prime_factor(n: usize) -> Option<usize> {
    if n < 2 { return None }
    if n % 2 == 0 { return Some(2)}
    let max_sqrt = (n as f64).sqrt().ceil() as usize;
    for odd in (3..=max_sqrt).step_by(2) {
        if n % odd == 0 { return Some(odd) }
    }
    Some(n)
}

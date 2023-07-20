use crate::bitset::Bitset;

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
        n /= i;
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
    if n < 2 {
        return None;
    }
    if n % 2 == 0 {
        return Some(2);
    }
    let max_sqrt = (n as f64).sqrt().ceil() as usize;
    for odd in (3..=max_sqrt).step_by(2) {
        if n % odd == 0 {
            return Some(odd);
        }
    }
    Some(n)
}

/// ```
/// use competitive_tools_rust::math::sieve_of_eratosthenes;
/// let primes = sieve_of_eratosthenes(30);
/// assert_eq!(primes, vec![2, 3, 5, 7,11, 13, 17, 19, 23, 29]);
/// let primes = sieve_of_eratosthenes(31);
/// assert_eq!(primes, vec![2, 3, 5, 7,11, 13, 17, 19, 23, 29, 31]);
/// ```
pub fn sieve_of_eratosthenes(max_n: usize) -> Vec<usize> {
    let mut sieve = vec![true; max_n + 1];
    let mut prime_numbers = Vec::with_capacity(max_n / 2);
    for i in 2..sieve.len() {
        if sieve[i] {
            prime_numbers.push(i);
        }
        for ind in (i * 2..sieve.len()).step_by(i) {
            sieve[ind] = false;
        }
    }
    prime_numbers
}

#[deprecated(note = "Please use usize::abs_diff, stabilized in Rust 1.60")]
pub trait AbsDiff: Copy {
    /// Calc absolute difference
    fn abs_diff(self, other: Self) -> Self;
}

#[allow(deprecated)]
impl AbsDiff for usize {
    /// Calc absolute difference
    /// It's similar to
    /// ```not run
    /// (x - y).abs()
    /// ```
    /// but It not consider the case that x - y is minus and when x, y is unsigned integer
    /// ```
    /// use competitive_tools_rust::math::AbsDiff;
    /// assert_eq!(0.abs_diff(0), 0);
    /// assert_eq!(1.abs_diff(0), 1);
    /// assert_eq!(0.abs_diff(2), 2);
    /// assert_eq!(10.abs_diff(100), 90);
    /// ```
    fn abs_diff(self, other: Self) -> Self {
        if self < other {
            other - self
        } else {
            self - other
        }
    }
}

pub trait Gcd: Copy {
    /// Calc GCD (Greatest Common Divisor)
    fn gcd(self, other: Self) -> Self;
}

impl Gcd for usize {
    ///
    /// ```
    /// use competitive_tools_rust::math::Gcd;
    /// assert_eq!(24.gcd(32), 8);
    /// assert_eq!(32.gcd(24), 8);
    /// assert_eq!(1.gcd(7), 1);
    /// assert_eq!(7.gcd(1), 1);
    /// assert_eq!(0.gcd(99), 99);
    /// assert_eq!(99.gcd(0), 99);
    /// assert_eq!(0.gcd(0), 0);
    /// ```
    fn gcd(self, other: Self) -> Self {
        fn rec(a: usize, b: usize) -> usize {
            if b == 0 {
                a
            } else {
                rec(b, a % b)
            }
        }
        rec(self, other)
    }
}

/// Used 082 - Counting Numbers（★3）
/// https://atcoder.jp/contests/typical90/tasks/typical90_cd
pub trait ModDiv: Sized {
    fn mod_div(self, divisor: Self, p: Self) -> Self;
    fn checked_mod_div(self, divisor: Self, p: Self) -> Result<Self, String>;
}

impl ModDiv for usize {
    /// Multiply inverse element calclated by Fermat's little theorem:  
    /// a^(p - 1) == 1 (mod p)
    /// a^(p - 2) == a^(-1) (mod p)
    /// p must be prime number
    /// divisor and p must be relatively prime
    /// ```
    /// use competitive_tools_rust::math::ModDiv;
    /// assert_eq!(10.mod_div(1, 101), (10 / 1) % 101);
    /// assert_eq!(10.mod_div(2, 101), (10 / 2) % 101);
    /// assert_eq!(10.mod_div(5, 101), (10 / 5) % 101);
    ///
    /// assert_eq!(
    ///     18_000_000_000_000_000_000.mod_div(5, 1_000_000_007),
    ///     18_000_000_000_000_000_000 / 5 % 1_000_000_007
    /// );
    /// ```
    fn mod_div(self, divisor: Self, p: Self) -> Self {
        self % p * divisor.mod_pow(p - 2, p) % p
    }

    /// Check following
    /// 1. p must be prime number
    /// 2. divisor and p must be relatively prime
    /// ```
    /// use competitive_tools_rust::math::ModDiv;
    /// assert_eq!(10.checked_mod_div(1, 101), Ok((10 / 1) % 101));
    /// assert_eq!(10.checked_mod_div(2, 101), Ok((10 / 2) % 101));
    /// assert_eq!(10.checked_mod_div(5, 101), Ok((10 / 5) % 101));
    ///
    /// assert_eq!(10.checked_mod_div(5, 100), Err("p: 100 is not prime".to_string()));
    /// assert_eq!(10.checked_mod_div(202, 101), Err("p: 101 and divisor: 202 are not relatively prime".to_string()));
    /// assert_eq!(202.checked_mod_div(5, 101), Ok(0));
    /// ```
    fn checked_mod_div(self, divisor: Self, p: Self) -> Result<Self, String> {
        if !is_prime(p) {
            return Err(format!("p: {} is not prime", p));
        }
        if p.gcd(divisor) != 1 {
            return Err(format!(
                "p: {} and divisor: {} are not relatively prime",
                p, divisor
            ));
        }
        Ok(self.mod_div(divisor, p))
    }
}

/// Used in 069 - Colorful Blocks 2
/// https://atcoder.jp/contests/typical90/tasks/typical90_bq
pub trait ModPow {
    /// Returns Modular Exponentiation  with
    fn mod_pow(self, exp: Self, m: Self) -> Self;
}

impl ModPow for usize {
    ///
    /// ```
    /// use competitive_tools_rust::math::ModPow;
    /// // Fermat's little theorem:  a^(p - 1) == 1 (mod p)
    /// // p is prime number
    /// assert_eq!(1usize.mod_pow(6, 7), 1);
    /// assert_eq!(2usize.mod_pow(6, 7), 1);
    /// assert_eq!(3usize.mod_pow(6, 7), 1);
    /// assert_eq!(4usize.mod_pow(6, 7), 1);
    /// assert_eq!(5usize.mod_pow(6, 7), 1);
    /// assert_eq!(6usize.mod_pow(6, 7), 1);
    /// assert_eq!(7usize.mod_pow(6, 7), 0);
    /// assert_eq!(8usize.mod_pow(6, 7), 1);
    /// // x^(0) == 1
    /// assert_eq!(111.mod_pow(0, 7), 1);
    /// assert_eq!(111.mod_pow(1, 7), 111 % 7);
    ///
    /// assert_eq!(18_000_000_000_000_000_000.mod_pow(6, 7), 1);
    /// ```
    fn mod_pow(self, exp: Self, m: Self) -> Self {
        let mut acc = 1;
        let mut current_mod_pow = self % m;
        for i in 0..exp {
            if 2usize.pow(i as u32) > exp {
                break;
            }
            if exp.is_bit_on(i) {
                acc = (acc * current_mod_pow) % m;
            }
            current_mod_pow = (current_mod_pow * current_mod_pow) % m;
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use crate::math::ModPow;
    const LAW: usize = 10usize.pow(9) + 7;

    fn naive_modular_exponentiation(b: usize, e: usize, m: usize) -> usize {
        let mut exp = 1;
        for _ in 0..e {
            exp = (exp * b) % m;
        }
        exp
    }

    #[test]
    fn test_mod_pow_in_small_case() {
        let b = 2;
        let exp = 10;
        assert_eq!(b.mod_pow(exp, 5), 4);
    }

    #[test]
    fn test_mod_pow_in_small_case2() {
        let b = 2;
        let exp = 4;
        assert_eq!(b.mod_pow(exp, 5), 1);
    }

    #[test]
    fn test_mod_pow_in_big_case() {
        let b = 2019;
        let exp = 615;
        assert_eq!(b.mod_pow(exp, LAW), 492000830);
        assert_eq!(naive_modular_exponentiation(b, exp, LAW), 492000830);
    }

    #[test]
    fn test_mod_pow_in_all_pattern() {
        for b in 0..100 {
            for exp in 0..100 {
                for m in 2..100 {
                    assert_eq!(b.mod_pow(exp, m), naive_modular_exponentiation(b, exp, m));
                }
            }
        }
    }
}

pub trait Divisors
where
    Self: Sized,
{
    fn divisors(&self) -> Vec<Self>;
}

impl Divisors for usize {
    /// ```
    /// use competitive_tools_rust::math::Divisors;
    /// assert_eq!(1.divisors(), vec![1]);
    /// assert_eq!(2.divisors(), vec![1, 2]);
    /// assert_eq!(3.divisors(), vec![1, 3]);
    /// assert_eq!(4.divisors(), vec![1, 2, 4]);
    /// assert_eq!(24.divisors(), vec![1, 2, 3, 4, 6, 8, 12, 24]);
    /// ```
    fn divisors(&self) -> Vec<Self> {
        let mut dvs = vec![];
        for i in 1..=*self {
            if i * i > *self {
                break;
            }
            if *self % i == 0 {
                dvs.push(i);
                if i * i != *self {
                    dvs.push(*self / i)
                }
            }
        }
        dvs.sort_unstable();
        dvs
    }
}

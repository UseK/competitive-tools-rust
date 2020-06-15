use std::ops::Range;

pub struct PermutationWithReplacement<I>
where
    I: Iterator,
    I::Item: Clone,
{
    k: usize,
    indices_source: Range<usize>,
    pool: Vec<I::Item>,
}

/// ```
/// use competitive_tools_rust::my_itertools::permutation_with_replacement::permutation_with_replacement;
/// let mut perm = permutation_with_replacement(0..3, 2);
/// assert_eq!(perm.next(), Some(vec![0, 0, 0]))
/// ```
pub fn permutation_with_replacement<I>(iter: I, k: usize) -> PermutationWithReplacement<I>
where
    I: Iterator,
    I::Item: Clone,
{
    let pool: Vec<I::Item> = iter.collect();
    let indices_source = 0..k.pow(pool.len() as u32);
    PermutationWithReplacement {
        k,
        indices_source,
        pool,
    }
}


impl<I> Iterator for PermutationWithReplacement<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let index_source = self.indices_source.next()?;
        let mut next_vec = Vec::new();
        Some(next_vec)
    }
}
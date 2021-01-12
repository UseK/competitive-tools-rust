use std::ops::Range;

#[allow(dead_code)]
struct PermutationWithReplacement<I>
where
    I: Iterator,
    I::Item: Clone,
{
    _k: usize,
    _indices_source: Range<usize>,
    _pool: Vec<I::Item>,
}

/// ```
/// //use competitive_tools_rust::my_itertools::permutation_with_replacement::permutation_with_replacement;
/// //let mut perm = permutation_with_replacement(0..3, 2);
/// //assert_eq!(perm.next(), Some(vec![0, 0, 0]))
/// ```
#[allow(dead_code)]
fn permutation_with_replacement<I>(iter: I, k: usize) -> PermutationWithReplacement<I>
where
    I: Iterator,
    I::Item: Clone,
{
    let pool: Vec<I::Item> = iter.collect();
    let indices_source = 0..k.pow(pool.len() as u32);
    PermutationWithReplacement {
        _k: k,
        _indices_source: indices_source,
        _pool: pool,
    }
}

#[allow(dead_code)]
impl<I> Iterator for PermutationWithReplacement<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let _index_source = self._indices_source.next()?;
        let next_vec = Vec::new();
        Some(next_vec)
    }
}

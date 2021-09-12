/// Used https://atcoder.jp/contests/typical90/tasks/typical90_ac
/// 029 - Long Bricks（★5）
/// ```
///
/// use competitive_tools_rust::coordinate_compression::compress;
/// assert_eq!(compress(vec![2, 5, 8]), (vec![0, 1, 2], 3));
/// assert_eq!(compress(vec![9, 3, 6, 2]), (vec![3, 1, 2, 0], 4));
/// assert_eq!(compress(vec![2, 5, 8, 5]), (vec![0, 1, 2, 1], 3));
/// assert_eq!(compress(vec![7, 7, 7, 7]), (vec![0, 0, 0, 0], 1));
/// ```
pub fn compress(values: Vec<usize>) -> (Vec<usize>, usize) {
    let (compression_map, w) = compression_map(values.clone());
    (
        values
            .into_iter()
            .map(|v| compression_map[v].unwrap())
            .collect(),
        w,
    )
}

/// Make map of Coordinate Compression
pub fn compression_map(mut values: Vec<usize>) -> (Vec<Option<usize>>, usize) {
    values.sort_unstable();
    values.dedup();
    let compression_map = values.iter().enumerate().fold(
        vec![None; values[values.len() - 1] + 1],
        |mut acc, (ind, &v)| {
            acc[v] = Some(ind);
            acc
        },
    );
    (compression_map, values.len())
}

#[cfg(test)]
mod tests {
    use crate::coordinate_compression::compress;
    use itertools::Itertools;
    use std::iter::once;

    #[test]
    /// https://atcoder.jp/contests/typical90/tasks/typical90_ac
    /// 029 - Long Bricks（★5）
    fn test_with_typical90_029_long_bricks() {
        let lr = vec![(27, 100), (8, 39), (83, 97), (24, 75)];
        let values: Vec<usize> = lr
            .into_iter()
            .flat_map(|(x, y)| once(x).chain(once(y)))
            .collect();
        let (compressed_values, _) = compress(values);
        let compressed_lr: Vec<(usize, usize)> = compressed_values.into_iter().tuples().collect();
        assert_eq!(compressed_lr, vec![(2, 7), (0, 3), (5, 6), (1, 4)])
    }
}

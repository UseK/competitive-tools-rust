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

pub mod permutation_with_replacement;

/// Run Lengh Encoding
/// Counting same items for compression
/// Used /// https://atcoder.jp/contests/typical90/tasks/typical90_cf
/// ```
/// use competitive_tools_rust::my_itertools::run_length_encoding;
/// assert_eq!(run_length_encoding(&vec!['o', 'o', 'x', 'o']), vec![('o', 2), ('x', 1), ('o', 1)]);
/// assert_eq!(run_length_encoding(&vec!['o', 'o', 'o', 'o']), vec![('o', 4)]);
/// assert_eq!(run_length_encoding(&vec![7, 7, 5, 3]), vec![(7, 2), (5, 1), (3, 1)]);
/// ```
pub fn run_length_encoding<T>(values: &[T]) -> Vec<(T, usize)>
where
    T: Eq + Copy,
{
    let mut count = 1;
    let mut acc = vec![];
    for i in 1..values.len() {
        if values[i] == values[i - 1] {
            count += 1;
        } else {
            acc.push((values[i - 1], count));
            count = 1;
        }
    }
    acc.push((*values.last().unwrap(), count));
    acc
}

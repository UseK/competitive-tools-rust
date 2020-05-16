use std::io;

pub fn parse_line<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn parse_values<T: std::str::FromStr>(n: usize) -> Vec<T> {
    let line = parse_line::<String>();
    let mut split = line.split_whitespace();
    let mut values = Vec::with_capacity(n);
    for _ in 0..n {
        let v = split.next().unwrap().parse::<T>().ok().unwrap();
        values.push(v);
    }
    values
}

/// Parse stdin into Tuple which has 2 items
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use competitive_tools_rust::io::parse_tuple2;
/// let (a, b) = parse_tuple2::<usize>();
/// ```
pub fn parse_tuple2<T: std::str::FromStr>() -> (T, T)
    where
        T: Copy
{
    let vs = parse_values(2);
    (vs[0], vs[1])
}

/// Parse stdin into Tuple which has 2 items
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use competitive_tools_rust::io::parse_tuple3;
/// let (a, b, c) = parse_tuple3::<usize>();
/// ```
pub fn parse_tuple3<T: std::str::FromStr>() -> (T, T, T)
    where
        T: Copy
{
    let vs = parse_values(3);
    (vs[0], vs[1], vs[2])
}

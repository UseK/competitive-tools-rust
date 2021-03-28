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
    T: Copy,
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
    T: Copy,
{
    let vs = parse_values(3);
    (vs[0], vs[1], vs[2])
}

/// Parse stdin into Tuple which has 2 items
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use competitive_tools_rust::io::parse_tuple4;
/// let (a, b, c) = parse_tuple4::<usize>();
/// ```
pub fn parse_tuple4<T: std::str::FromStr>() -> (T, T, T, T)
where
    T: Copy,
{
    let vs = parse_values(4);
    (vs[0], vs[1], vs[2], vs[3])
}

#[macro_export]
macro_rules! d {
    ( $( $x:expr ),* ) => {{
        let mut s_vec = vec![];
        $(
            let s = format!("{}: {:?}", stringify!($x), $x);
            s_vec.push(s);
        )*
        let joined = s_vec.join(", ");
        println!("{}", joined);
    }}
}

#[macro_export]
macro_rules! debug_str {
    ( $( $x:expr ),* ) => {{
        let mut s_vec = vec![];
        $(
            let s = format!("{}: {:?}", stringify!($x), $x);
            s_vec.push(s);
        )*
        s_vec.join(", ")
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macro_when_primitive() {
        let five = 5;
        let result = debug_str!(five);
        assert_eq!(result, "five: 5");
    }

    #[test]
    fn test_macro_when_vec() {
        let v = vec![2, 5, 8];
        let result = debug_str!(v);
        assert_eq!(result, "v: [2, 5, 8]");
    }

    #[test]
    fn test_macro_when_multiple() {
        let x = 2;
        let y = 9;
        let result = debug_str!(x, y);
        assert_eq!(result, "x: 2, y: 9");
        //d!(x, y); //=> x: 2, y: 9
    }
}

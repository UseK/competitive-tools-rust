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
/// let (a, b): (usize, usize) = parse_tuple2();
/// ```
pub fn parse_tuple2<T1, T2>() -> (T1, T2)
where
    T1: std::str::FromStr,
    T2: std::str::FromStr,
{
    let line: String = parse_line();
    let mut split = line.split_whitespace();
    (
        split.next().unwrap().parse::<T1>().ok().unwrap(),
        split.next().unwrap().parse::<T2>().ok().unwrap(),
    )
}

/// Parse stdin into Tuple which has 2 items
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use competitive_tools_rust::io::parse_tuple3;
/// let (a, b, c): (String, usize, isize) = parse_tuple3();
/// ```
pub fn parse_tuple3<T1, T2, T3>() -> (T1, T2, T3)
where
    T1: std::str::FromStr,
    T2: std::str::FromStr,
    T3: std::str::FromStr,
{
    let line: String = parse_line();
    let mut split = line.split_whitespace();
    (
        split.next().unwrap().parse::<T1>().ok().unwrap(),
        split.next().unwrap().parse::<T2>().ok().unwrap(),
        split.next().unwrap().parse::<T3>().ok().unwrap(),
    )
}

/// Parse stdin into Tuple which has 2 items
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use competitive_tools_rust::io::parse_tuple4;
/// let (a, b, c, d): (usize, usize, usize, usize) = parse_tuple4();
/// ```
pub fn parse_tuple4<T1, T2, T3, T4>() -> (T1, T2, T3, T4)
where
    T1: std::str::FromStr,
    T2: std::str::FromStr,
    T3: std::str::FromStr,
    T4: std::str::FromStr,
{
    let line: String = parse_line();
    let mut split = line.split_whitespace();
    (
        split.next().unwrap().parse::<T1>().ok().unwrap(),
        split.next().unwrap().parse::<T2>().ok().unwrap(),
        split.next().unwrap().parse::<T3>().ok().unwrap(),
        split.next().unwrap().parse::<T4>().ok().unwrap(),
    )
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

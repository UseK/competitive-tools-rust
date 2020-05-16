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

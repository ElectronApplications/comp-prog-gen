use input::*;

@start@gcd
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
@end@gcd

fn main() {
    @start@t
    let t: usize = read_value();

    for _ in 0..t {
        
    }
    @end@t
}

pub mod input {
    use std::{convert::TryFrom, fmt::Debug, io, str::FromStr};

    pub fn read_line() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_owned()
    }

    pub fn read_value<T: FromStr>() -> T
    where
        <T as FromStr>::Err: Debug,
    {
        let input = read_line();
        input.parse::<T>().unwrap()
    }

    pub fn read_vec<T: FromStr>() -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let input = read_line();
        input
            .split_whitespace()
            .map(|x| x.parse::<T>().unwrap())
            .collect()
    }

    pub fn read_slice<T: FromStr + Debug, const N: usize>() -> [T; N]
    where
        <T as FromStr>::Err: Debug,
    {
        let input = read_vec::<T>();
        <[T; N]>::try_from(input).unwrap()
    }

    pub fn read_tuple<T: FromStr, E: FromStr>() -> (T, E)
    where
        <T as FromStr>::Err: Debug,
        <E as FromStr>::Err: Debug,
    {
        let input = read_line();
        let (a, b) = input.split_once(' ').unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }
}

use proconio::input;
use std::cmp;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", cmp::max(s(a), s(b)));
}

fn s(mut x: usize) -> usize {
    let mut res = 0;
    while x > 0 {
        res += x % 10;
        x /= 10;
    }

    return res;
}

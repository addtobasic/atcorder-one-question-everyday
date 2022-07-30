use proconio::{input, marker::Chars};
use std::cmp;
use std::num;

fn main() {
    input! {
        n : usize,
        d : f64,
        xy : [(i64, i64); n],
    }

    let mut ans = 0;

    for &(x, y) in &xy {
        if d >= ((x.pow(2) + y.pow(2)) as f64).sqrt() {
            ans += 1;
        }
    }

    println!("{}", ans);
}

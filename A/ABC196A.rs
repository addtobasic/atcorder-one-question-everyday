use proconio::{input, marker::Chars};
use std::cmp;

fn main() {
    input! {
        a : isize,
        b : isize,
        c : isize,
        d : isize
    }

    println!("{}", cmp::max(a, b) - cmp::min(c, d))
}

use proconio::{input, marker::Chars};
use std::cmp;

fn main() {
    input! {
        v : usize,
        t : usize,
        s : usize,
        d : usize
    }

    if  d < v*t || s*v < d {
        println!("Yes");
    } else {
        println!("No");
    }
}

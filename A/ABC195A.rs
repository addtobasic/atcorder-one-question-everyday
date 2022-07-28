use proconio::{input, marker::Chars};
use std::cmp;

fn main() {
    input! {
        m : usize,
        h : usize
    }

    if h % m == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}

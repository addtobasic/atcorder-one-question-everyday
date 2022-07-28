use proconio::{input, marker::Chars};
use std::cmp;

fn main() {
    input! {
        a : usize,
        b : usize,
        c : usize
    }

    if a.pow(2) + b.pow(2) < c.pow(2) {
        println!("Yes");
    } else {
        println!("No");
    }
}

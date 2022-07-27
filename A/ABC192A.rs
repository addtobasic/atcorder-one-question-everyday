use proconio::{input, marker::Chars};
use std::cmp;

fn main() {
    input! {
        x : usize,
    }

    println!("{}", 100 - x % 100);
}

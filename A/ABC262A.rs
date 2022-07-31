use proconio::{input, marker::Chars};
use std::cmp;
use std::num;

fn main() {
    input! {
        y : u64,
    }

    if y % 4 == 2 {
        println!("{}", y);
    } else if y % 4 == 0 {
        println!("{}", y + 2);
    } else {
        println!("{}", y + y % 4);
    }
}

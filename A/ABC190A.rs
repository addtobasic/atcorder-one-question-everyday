use proconio::{input, marker::Chars};
use std::cmp;

fn main() {
    input! {
        a : isize,
        b : isize,
        c : isize
    }

    if c == 0 {
        if a > b {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    } else {
        if b > a {
            println!("Aoki");
        } else {
            println!("Takahashi");
        }
    }
}

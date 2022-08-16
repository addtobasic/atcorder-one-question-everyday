use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    for idx in a {
        if idx != x {
            print!("{} ",idx);
        }
    }
}

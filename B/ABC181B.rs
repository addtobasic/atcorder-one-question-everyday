use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n: usize,
        ab: [(isize, isize); n],
    }

    let mut ans = 0;

    for (a, b) in &ab {
        ans += b * (b + 1) / 2 - a * (a - 1) / 2;
    }

    println!("{}", ans);
}

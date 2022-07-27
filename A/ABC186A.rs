use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        w: usize,
    }

    let ans = n / w;

    println!("{}", ans);
}

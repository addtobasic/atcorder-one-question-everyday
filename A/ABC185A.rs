use proconio::input;
use std::cmp;

fn main() {
    input! {
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
    }

    let ans = cmp::min(cmp::min(cmp::min(a1, a2), a3), a4);

    println!("{}", ans);
}

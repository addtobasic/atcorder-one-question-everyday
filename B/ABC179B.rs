use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n: usize,
        d: [(usize, usize); n]
    }

    let mut ans = false;

    for i in 0..n - 2 {
        if d[i].0 == d[i].1 {
            if d[i + 1].0 == d[i + 1].1 {
                if d[i + 2].0 == d[i + 2].1 {
                    ans = true;
                }
            }
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}

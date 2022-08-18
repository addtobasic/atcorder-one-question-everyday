use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut ans = std::usize::MAX;

    for (i, &(a, _)) in ab.iter().enumerate() {
        for (j, &(_, b)) in ab.iter().enumerate() {
            if i == j {
                ans = cmp::min(ans, a + b);
            } else {
                ans = cmp::min(ans, cmp::max(a, b));
            }
        }
    }

    println!("{}", ans);
}

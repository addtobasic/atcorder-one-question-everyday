use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n: usize,
        apx: [(usize, usize, usize); n],
    }
    let mut ans = 1_000_000_001;

    for (a, p, x) in apx {
        if x > a {
            ans = cmp::min(p, ans);
        }
    }

    if ans < 1_000_000_001 {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n: usize,
        x: usize,
        vp: [(usize, usize); n],
    }

    let mut alcohol_sum = 0;
    let mut ans = 0;
    let mut cnt = 0;

    for i in 0..n {
        let (v, p) = vp[i];
        alcohol_sum += v * p;
        cnt += 1;

        if alcohol_sum > x * 100{
            ans = cnt;
            break;
        }
    }

    if alcohol_sum > x * 100 {
        println!("{}", ans);
    } else {
        println!("{}", -1);
    }
}

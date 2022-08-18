use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        a: usize,
        b: usize,
        w: usize,
    }

    let mut ans_min = std::usize::MAX;
    let mut ans_max = 0;

    for n in 1..=1000000 {
        if a * n <= 1000 * w && 1000 * w <= b * n {
            ans_min = cmp::min(ans_min, n);
            ans_max = cmp::max(ans_max, n);
        }
    }

    if ans_max == 0 {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", ans_min, ans_max);
    }
}

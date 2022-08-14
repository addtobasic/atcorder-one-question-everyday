use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        h: usize,
        w: usize,
        hw: [[usize; w]; h],
    }

    let mut min = 101;
    let mut ans = 0;

    for i in 0..h {
        for j in 0..w {
            if hw[i][j] < min {
                min = hw[i][j];
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if hw[i][j] > min {
                ans += hw[i][j] - min;
            }
        }
    }

    println!("{}", ans);
}

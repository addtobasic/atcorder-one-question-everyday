use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }

    let mut ans = 0;

    for i in 0..n {
        for j in i + 1..n {
            let dx = xy[i].0 - xy[j].0;
            let dy = xy[i].1 - xy[j].1;

            let slope = dy as f64 / dx as f64;

            if -1.0 <= slope && slope <= 1.0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

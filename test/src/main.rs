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
        let (xi, yi) = xy[i];
        for j in i + 1..n {
            let (xj, yj) = xy[j];
            let dx = xj - xi;
            let dy = yj - yi;

            let slope = dy as f64 / dx as f64;

            if -1.0 <= slope && slope <= 1.0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n: usize,
        x: [isize; n]
    }

    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut ans3 = 0;

    for i in 0..n {
        ans1 += x[i].abs();
        ans2 += x[i].pow(2);
        ans3 = cmp::max(x[i].abs(), ans3);
    }

    println!("{}", ans1);
    println!("{}", (ans2 as f64).sqrt());
    println!("{}", ans3);
}

use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        s: (f64, f64),
        g: (f64, f64),
    }

    println!("{}", (s.0 * g.1 + g.0 * s.1) / (s.1 + g.1));
}

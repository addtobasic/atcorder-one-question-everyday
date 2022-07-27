use proconio::input;
use std::cmp;

fn main() {
    input! {
        x: isize,
        y: isize,
    }

    if (x - y).abs() < 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}

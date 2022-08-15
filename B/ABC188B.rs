use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
        b: [isize; n],
    }

    let mut inner_product = 0;

    for i in 0..n {
        inner_product += a[i] * b[i];
    }

    if inner_product == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

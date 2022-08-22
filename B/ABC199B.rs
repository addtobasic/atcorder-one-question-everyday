use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }

    a.sort_by_key(|&x| cmp::Reverse(x));
    b.sort();

    if b[0] >= a[0] {
        println!("{}", b[0] - a[0] + 1);
    } else {
        println!("{}", 0);
    }
}

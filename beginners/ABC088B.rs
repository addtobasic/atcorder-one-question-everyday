use proconio::input;
use std::cmp::Reverse;

fn main() {
    input! {
      n: usize,
      mut a: [usize; n]
    }

    let mut bob = 0;
    let mut alice = 0;
    a.sort_by_key(|&x| Reverse(x));

    for i in 0..n {
        if i % 2 == 0 {
            alice += a[i];
        } else {
            bob += a[i];
        }
    }

    println!("{}", alice - bob);
}

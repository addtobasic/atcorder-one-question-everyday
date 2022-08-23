use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        mut n: usize,
    }

    let mut v = vec![];
    while n > 0 {
        n -= 1;
        v.push(n % 26);
        n /= 26
    }

    let ans = v
        .into_iter()
        .rev()
        .map(|i| (i as u8 + 'a' as u8) as char)
        .collect::<String>();

    println!("{}", ans);
}

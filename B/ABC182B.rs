use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut max = 0;
    let mut ans = 0;

    for i in 2..=1000 {
        let mut count = 0;
        for j in 0..n {
            if a[j] % i == 0 {
                count += 1;
            }
        }

        if max <= count {
            max = count;
            ans = i;
        }
    }

    println!("{}", ans);
}

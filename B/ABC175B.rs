use proconio::{input, marker::Chars};
use std::cmp;
use std::num;

fn main() {
    input! {
        n : usize,
        mut l : [i64; n],
    }

    l.sort();
    let mut ans = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if l[i] != l[j] && l[j] != l[k] && l[k] < l[i] + l[j] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}

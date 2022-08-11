use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n: isize,
        m: usize,
        t: isize,
        ab: [(isize, isize); m],
    }

    let mut battery = n;
    let mut is_no_battery = false;
    let mut pre_time = 0;

    for &(a, b) in &ab {
        battery -= a - pre_time;
        if battery <= 0 {
            is_no_battery = true;
            break;
        }
        battery += b - a;
        if battery > n {
            battery = n;
        }
        pre_time = b;
    }
    if !is_no_battery && battery - (t - pre_time) <= 0 {
        is_no_battery = true;
    }

    if !is_no_battery {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}

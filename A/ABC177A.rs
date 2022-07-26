use proconio::input;

fn main() {
    input! {
        d: isize,
        t: isize,
        s: isize
    }

    if s * t >= d {
        println!("Yes");
    } else {
        println!("No");
    }
}

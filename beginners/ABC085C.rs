use proconio::input;

fn main() {
    input! {
        n: i32,
        y: i32
    }

    for a in 0..=n {
        for b in 0..=n {
            let c = n - a - b;
            if a * 10000 + b * 5000 + c * 1000 == y && c >= 0 {
                println!("{} {} {}", a, b, c);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}

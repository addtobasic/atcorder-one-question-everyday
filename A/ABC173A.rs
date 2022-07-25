use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", (10000 - n) % 1000);
}

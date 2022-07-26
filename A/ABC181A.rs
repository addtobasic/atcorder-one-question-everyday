use proconio::input;

fn main() {
    input! {
        n: isize,
    }

    if n % 2 == 0 {
        println!("White");
    } else {
        println!("Black");
    }
}

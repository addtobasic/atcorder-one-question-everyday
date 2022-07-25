use proconio::input;

fn main() {
    input! {
        n: isize,
    }

    if n >= 30 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}

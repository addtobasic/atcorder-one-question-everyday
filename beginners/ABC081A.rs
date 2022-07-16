use proconio::input;

fn main() {
    input! {
      a: String,
    }

    let mut num = 0;
    for i in a.as_str().chars() {
        if i == '1' {
            num += 1;
        }
    }

    println!("{}", num);
}

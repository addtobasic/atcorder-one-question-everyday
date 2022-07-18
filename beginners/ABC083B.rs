use proconio::input;

fn main() {
    input! {
      n: i32,
      a: i32,
      b: i32,
    }

    let mut num = 0;

    for i in 0..=n {
        let mut tmp = i;
        let mut count = 0;

        while tmp > 0 {
            count += tmp % 10;
            tmp /= 10;
        }

        if count >= a && b >= count {
            num += i;
        }
    }

    println!("{}", num);
}

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut count = 0;
    let mut ans= 0;

    for c in s.chars() {
        if c == 'R' {
            count += 1;
        } else {
            count = 0;
        }
        ans = ans.max(count);
    }

    println!("{}", ans);
}

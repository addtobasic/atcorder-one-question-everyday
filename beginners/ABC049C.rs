use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let words = ["eraser", "erase", "dreamer", "dream",];
    let mut t = s;

    for i in 0..words.len() {
        t = t.replace(words[i], "").to_string();
    }

    if t.is_empty() {
        println!("YES");
    } else {
        println!("NO");
    }
}

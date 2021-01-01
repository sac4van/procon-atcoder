use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let mut t = String::from(s);

    if t.chars().collect::<Vec<char>>()[t.len() - 1] == 's' {
        t.push_str("es");
    } else {
        t.push_str("s");
    };

    println!("{}", t);
}

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        n: usize,
        mut l: [i64; n],
    }

    let mut ans = s;

    println!("{}", ans);
}

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [i64; 4],
    }

    let ans = a[0] * a[3] - a[2] * a[1];

    println!("{}", ans);
}

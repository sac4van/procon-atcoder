use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }

    let ans = if s == "RRR" {
        3
    } else if s == "RRS" || s == "SRR" {
        2
    } else if s == "SSS" {
        0
    } else {
        1
    };

    println!("{}", ans);
}

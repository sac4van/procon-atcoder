use proconio::*;

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let ans = if s == "AAA" || s == "BBB" {
        "No"
    } else {
        "Yes"
    };

    println!("{}", ans);
}

use proconio::*;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let ans = if c == 0 {
        if a > b {
            "Takahashi"
        } else {
            "Aoki"
        }
    } else {
        if a >= b {
            "Takahashi"
        } else {
            "Aoki"
        }
    };

    println!("{}", ans);
}

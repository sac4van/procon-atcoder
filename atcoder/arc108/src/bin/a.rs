use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: u128,
        p: u128,
    }

    let mut ans = "No";

    for i in 1..=1000000 {
        if i * i > p {
            break;
        }

        if p % i == 0 {
            let j = p / i;
            if i + j == s {
                ans = "Yes";
                break;
            }
        }
    }

    println!("{}", ans);
}

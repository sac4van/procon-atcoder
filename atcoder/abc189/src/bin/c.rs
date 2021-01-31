use proconio::*;
use std::cmp::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;

    for i in 0..n {
        let mut tmp = 0;
        for j in i..n {
            if a[j] < a[i] {
                break;
            }
            tmp += a[i];
        }
        for j in (0..i).rev() {
            if a[j] < a[i] {
                break;
            }
            tmp += a[i];
        }

        ans = ans.max(tmp);
    }

    println!("{}", ans);
}

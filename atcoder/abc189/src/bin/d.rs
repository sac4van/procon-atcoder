use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut f = vec![0_u128; n + 1];
    let mut t = vec![0_u128; n + 1];

    t[0] = 1;
    f[0] = 1;

    for i in 1..=n {
        if s[i - 1] == "AND" {
            t[i] = t[i - 1];
            f[i] = t[i - 1] + f[i - 1] * 2;
        } else {
            t[i] = t[i - 1] * 2 + f[i - 1];
            f[i] = f[i - 1];
        }
    }

    let ans = t[n];

    println!("{}", ans);
}

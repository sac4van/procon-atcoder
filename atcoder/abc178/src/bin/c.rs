use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let m: u128 = 1000000000 + 7;
    let mut pow10 = vec![1_u128; n + 1];
    let mut pow9 = vec![1_u128; n + 1];
    let mut pow8 = vec![1_u128; n + 1];

    for i in 1..=n {
        pow10[i] = (10 * pow10[i - 1]) % m;
        pow9[i] = (9 * pow9[i - 1]) % m;
        pow8[i] = (8 * pow8[i - 1]) % m;
    }

    let ans = (pow10[n] + pow8[n] + 2 * (m - pow9[n])) % m;

    println!("{}", ans);
}

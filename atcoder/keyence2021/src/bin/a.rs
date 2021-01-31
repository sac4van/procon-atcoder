use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u128; n],
        b: [u128; n],
    }

    let mut a_max: u128 = 1;

    let mut ans = 1;
    for i in 0..n {
        // prev ans, a[i]*b[i], a_max*b[i]
        ans = ans.max(a[i] * b[i]).max(a_max * b[i]);

        a_max = a_max.max(a[i]);

        println!("{}", ans);
    }
}

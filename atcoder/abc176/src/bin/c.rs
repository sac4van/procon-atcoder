use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;

    let mut prev = 0;
    for ai in a {
        let add = if prev > ai { prev - ai } else { 0 };
        prev = ai + add;
        ans += add;
    }

    println!("{}", ans);
}

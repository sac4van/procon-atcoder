use proconio::*;

#[fastout]
fn main() {
    input! {
        a: [i128; 4],
    }

    let mut cand: Vec<i128> = Vec::new();

    for i in 0..2 {
        for j in 2..4 {
            cand.push(a[i] * a[j]);
        }
    }

    let ans = cand.iter().max().unwrap();

    println!("{}", ans);
}

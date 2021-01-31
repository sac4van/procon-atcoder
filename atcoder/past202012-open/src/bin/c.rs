use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let d: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let mut ans_vec: Vec<char> = Vec::new();

    let mut x = n;

    while x > 0 {
        ans_vec.push(d[x % 36]);
        x /= 36;
    }

    if n > 0 {
        let ans = ans_vec.iter().rev().collect::<String>();
        println!("{}", ans);
    } else {
        println!("0");
    };
}

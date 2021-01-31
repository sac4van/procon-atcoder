use proconio::marker::*;
use proconio::*;
use std::collections::*;

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut set: HashSet<char> = HashSet::new();
    let mut vec: Vec<char> = Vec::new();

    for &c in s.iter().rev() {
        if !set.contains(&c) {
            vec.push(c);
            set.insert(c);
        }
    }

    let ans = vec.iter().rev().collect::<String>();
    println!("{}", ans);
}

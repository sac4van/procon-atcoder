use proconio::*;
use std::cmp::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [(isize, isize); n],
    }

    let mut z = vec![0; n];
    let mut w = vec![0; n];

    for i in 0..n {
        let (x, y) = p[i];
        z[i] = x + y;
        w[i] = x - y;
    }

    fn _max(vec: &Vec<isize>) -> isize {
        *vec.iter().max().unwrap()
    }

    fn _min(vec: &Vec<isize>) -> isize {
        *vec.iter().min().unwrap()
    }

    let ans = max(_max(&z) - _min(&z), _max(&w) - _min(&w));

    println!("{}", ans);
}

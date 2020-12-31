use proconio::marker::Chars;
use proconio::{fastout, input};
use std::vec::Vec;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut valid_i = Vec::new();
    let mut valid_j = Vec::new();

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                valid_i.push(i);
                break;
            }
        }
    }

    for j in 0..w {
        for i in 0..h {
            if a[i][j] == '#' {
                valid_j.push(j);
                break;
            }
        }
    }

    for &i in valid_i.iter() {
        for &j in valid_j.iter() {
            print!("{}", a[i][j]);
        }
        println!("");
    }
}

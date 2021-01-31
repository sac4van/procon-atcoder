use proconio::marker::*;
use proconio::*;
use std::collections::*;

#[fastout]
fn main() {
    input! {
        s: Chars,
        q: usize,
    }
    let mut deq: VecDeque<char> = VecDeque::from(s);
    let mut reversed = false;

    for _ in 0..q {
        input! {
            t: u8,
        }
        if t == 1 {
            reversed = !reversed;
        } else {
            input! {
                f: u8,
                c: char
            }
            if (!reversed && f == 1) || (reversed && f == 2) {
                deq.push_front(c);
            } else {
                deq.push_back(c);
            }
        }
    }

    let ans = if reversed {
        deq.iter().rev().collect::<String>()
    } else {
        deq.iter().collect::<String>()
    };

    println!("{}", ans);
}

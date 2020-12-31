use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    fn winner(x: char, y: char) -> Option<char> {
        if x == y {
            Some(x)
        } else {
            match (x, y) {
                ('R', 'P') => Some('P'),
                ('P', 'R') => Some('P'),
                ('P', 'S') => Some('S'),
                ('S', 'P') => Some('S'),
                ('S', 'R') => Some('R'),
                ('R', 'S') => Some('R'),
                _ => None,
            }
        }
    }

    let mut t = s.clone();
    for _ in 0..k {
        t = (0..n)
            .map(|i| winner(t[(i * 2) % n], t[(i * 2 + 1) % n]).unwrap())
            .collect();
    }

    print!("{}", t[0]);
}

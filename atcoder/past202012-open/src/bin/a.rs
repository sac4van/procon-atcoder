use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    let mut done = false;
    for c in vec!['o', 'x'] {
        if done {
            break;
        }
        for i in 0..3 {
            let mut win = true;
            for j in 0..3 {
                if s[i + j] != c {
                    win = false;
                    break;
                }
            }
            if win {
                println!("{}", c);
                done = true;
                break;
            }
        }
    }

    if !done {
        println!("draw");
    }
}

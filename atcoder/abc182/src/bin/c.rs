use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n_str: Chars,
    }

    let mut num_mod3 = vec![0_isize; 3];
    let l = n_str.len();

    for c in n_str {
        let d = (c as usize) - ('0' as usize);
        num_mod3[d % 3] += 1;
    }

    let a1 = num_mod3[1];
    let a2 = num_mod3[2];
    let r = (num_mod3[1] + 2 * num_mod3[2]) % 3;

    let ans: isize = if r % 3 == 0 {
        0
    } else if a1 > 0 && r % 3 == 1 {
        1
    } else if a2 > 0 && r % 3 == 2 {
        1
    } else if a1 > 1 && r % 3 == 2 {
        2
    } else if a2 > 1 && r % 3 == 1 {
        2
    } else {
        -1
    };

    println!("{}", if ans == l as isize { -1 } else { ans });
}

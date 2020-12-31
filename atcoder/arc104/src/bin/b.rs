use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut at = vec![0; n + 1];
    let mut cg = vec![0; n + 1];
    let mut ans = 0;
    for _i in 1..(n + 1) {
        at[_i] = at[_i - 1];
        cg[_i] = cg[_i - 1];
        match s[_i - 1] {
            'A' => at[_i] += 1,
            'T' => at[_i] -= 1,
            'C' => cg[_i] += 1,
            'G' => cg[_i] -= 1,
            _ => panic!("Invalid input."),
        };
    }

    for i in 0..n {
        for j in (i + 1)..n {
            if at[j + 1] == at[i] && cg[j + 1] == cg[i] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

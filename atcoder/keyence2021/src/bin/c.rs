use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        l: [(Usize1, Usize1, char); k],
    }

    let modulo: u64 = 998244353;
    let mut f: Vec<Vec<char>> = vec![vec!['E'; w]; h];

    for (hi, wi, ci) in l {
        f[hi][wi] = ci;
    }

    let mut dp: Vec<Vec<u64>> = vec![vec![0_u64; w + 1]; h + 1];
    dp[h - 1][w - 1] = if f[h - 1][w - 1] == 'E' { 3 } else { 1 };

    let mut num_empty_h: Vec<Vec<u64>> = vec![vec![1_u64; w + 1]; h + 1];
    for j in 0..w {
        let mut acc: u64 = 1;
        for i in (0..h).rev() {
            num_empty_h[i][j] = acc;
            if f[i][j] == 'E' {
                acc = (acc * 3) % modulo;
            }
        }
    }
    let mut num_empty_w: Vec<Vec<u64>> = vec![vec![1_u64; w + 1]; h + 1];
    for i in 0..h {
        let mut acc: u64 = 1;
        for j in (0..w).rev() {
            num_empty_w[i][j] = acc;
            if f[i][j] == 'E' {
                acc = (acc * 3) % modulo;
            }
        }
    }

    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if i == h - 1 && j == w - 1 {
                continue;
            }
            dp[i][j] = match f[i][j] {
                'X' => {
                    (dp[i + 1][j] * num_empty_w[i][j] + dp[i][j + 1] * num_empty_h[i][j]) % modulo
                }
                'R' => dp[i][j + 1] * num_empty_h[i][j] % modulo,
                'D' => dp[i + 1][j] * num_empty_w[i][j] % modulo,
                _ => {
                    2 * (dp[i][j + 1] * num_empty_h[i][j] + dp[i + 1][j] * num_empty_w[i][j])
                        % modulo
                }
            }
        }
    }

    // for i in 0..h {
    //     for j in 0..w {
    //         print!("{}", num_empty_w[i][j]);
    //     }
    //     println!("");
    // }

    // for i in 0..h {
    //     for j in 0..w {
    //         print!("{}", num_empty_h[i][j]);
    //     }
    //     println!("");
    // }

    let ans = dp[0][0];

    println!("{}", ans);
}

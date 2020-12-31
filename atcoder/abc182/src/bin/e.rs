use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        bulb: [(usize, usize); n],
        block: [(usize, usize); m],
    }
    let s_empty = 0;
    let s_bulb = 1;
    let s_block = 2;
    let mut field = vec![vec![s_empty; w]; h];
    let mut illuminated = vec![vec![false; w]; h];
    for &(i, j) in bulb.iter() {
        field[i - 1][j - 1] = s_bulb;
    }
    for &(i, j) in block.iter() {
        field[i - 1][j - 1] = s_block;
    }
    for i in 0..h {
        let mut _bulb = false;
        let mut left = 0;
        for j in 0..=w {
            if j < w && field[i][j] == s_bulb {
                _bulb = true;
            }
            if j == w || field[i][j] == s_block {
                if _bulb {
                    for k in left..j {
                        illuminated[i][k] = true;
                    }
                    _bulb = false;
                }
                left = j + 1;
            }
        }
    }
    for j in 0..w {
        let mut _bulb = false;
        let mut up = 0;
        for i in 0..=h {
            if i < h && field[i][j] == s_bulb {
                _bulb = true;
            }
            if i == h || field[i][j] == s_block {
                if _bulb {
                    for k in up..i {
                        illuminated[k][j] = true;
                    }
                    _bulb = false;
                }
                up = i + 1;
            }
        }
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans += if illuminated[i][j] { 1 } else { 0 };
        }
    }
    println!("{}", ans);
}

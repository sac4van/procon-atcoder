use proconio::*;
use std::cmp::*;
use std::collections::*;

#[fastout]
fn main() {
    input! {
        x: i128,
        y: i128,
    }

    let mut memo: HashMap<i128, i128> = HashMap::new();

    fn ans(y: i128, x: i128, memo: &mut HashMap<i128, i128>) -> i128 {
        if !memo.contains_key(&y) {
            *memo.entry(y).or_default() = if y == 1 {
                (x - y).abs()
            } else if y & 1 == 1 {
                (x - y)
                    .abs()
                    .min(ans((y + 1) / 2, x, memo) + 2)
                    .min(ans((y - 1) / 2, x, memo) + 2)
            } else {
                (x - y).abs().min(ans(y / 2, x, memo) + 1)
            }
        };
        *memo.get(&y).unwrap()
    }

    println!("{}", ans(x, y, &mut memo));
}

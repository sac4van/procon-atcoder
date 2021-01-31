use itertools::Itertools;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut ans = Vec::new();
    let size = 1 << n;

    let mut cnt = 0;
    for p in (0..size).combinations(size / 2) {
        cnt += 1;
        let mut t = vec!['A'; 1 << n];
        for pi in p {
            t[pi] = 'B';
        }
        ans.push(t.into_iter().collect::<String>());
    }

    println!("{}", cnt / 2);
    for i in 0..(cnt / 2) {
        println!("{}", ans[i]);
    }
}

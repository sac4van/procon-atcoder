use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        e: [(Usize1, Usize1); m],
    }

    let mut g = vec![HashSet::new(); n];

    for (x, y) in e {
        g[y].insert(x);
    }

    let mut used = vec![false; n];
    let mut dp = vec![0; n];

    fn dfs(i: usize, g: &Vec<HashSet<usize>>, dp: &mut Vec<usize>, used: &mut Vec<bool>) -> usize {
        if !used[i] {
            for &j in &g[i] {
                dp[i] = dp[i].max(1 + dfs(j, g, dp, used));
            }
            used[i] = true;
        }
        dp[i]
    }

    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(dfs(i, &g, &mut dp, &mut used));
    }

    println!("{}", ans);
}

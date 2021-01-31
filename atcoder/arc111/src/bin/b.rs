use proconio::*;
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut g: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut used: HashMap<usize, bool> = HashMap::new();

    for (a, b) in ab {
        g.entry(a).or_default().push(b);
        g.entry(b).or_default().push(a);
        used.entry(a).or_insert(false);
        used.entry(b).or_insert(false);
    }

    let mut ans = 0;

    for &s in g.keys() {
        if used[&s] {
            continue;
        }

        let mut contains_loop = false;
        let mut size_of = 0;
        // (node, parent)
        let mut que: VecDeque<(usize, usize)> = VecDeque::new();
        que.push_back((s, 0));
        *used.entry(s).or_default() = true;
        size_of += 1;

        while !que.is_empty() {
            let (n, p) = que.pop_front().unwrap();
            for &nn in &g[&n] {
                if used[&nn] {
                    if nn != p {
                        contains_loop = true;
                    }
                } else {
                    que.push_back((nn, n));
                    *used.entry(nn).or_default() = true;
                    size_of += 1;
                }
            }
        }

        ans += if contains_loop { size_of } else { size_of - 1 };
    }

    println!("{}", ans);
}

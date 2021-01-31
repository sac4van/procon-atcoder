use proconio::*;
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: isize,
        c: isize,
        l: [(isize, isize, isize); n],
    }

    let mut set: HashMap<isize, isize> = HashMap::new();
    let mut pq = Vec::new();

    for (a, b, c) in l {
        *set.entry(a).or_default() += c;
        *set.entry(b + 1).or_default() -= c;
    }
    for (&k, &v) in &set {
        pq.push((v, k));
    }
    pq.sort_by(|x, y| y.1.cmp(&x.1));

    let mut ans: isize = 0;
    let mut pos: isize = 0;
    let mut cost: isize = 0;

    while !pq.is_empty() {
        let (x, y) = pq.pop().unwrap();
        let cost2 = if cost < c { cost } else { c };
        ans += cost2 * (y - pos);
        pos = y;
        cost += x;
    }

    println!("{}", ans);
}

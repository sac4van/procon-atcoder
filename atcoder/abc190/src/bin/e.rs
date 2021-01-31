use proconio::marker::*;
use proconio::*;
// use std::cmp::*;
// use std::collections::*;

mod union_find {
    #[allow(dead_code)]
    pub struct DSU {
        size: usize,
        pos: Vec<isize>,
    }
    #[allow(dead_code)]
    impl DSU {
        pub fn new(n: usize) -> Self {
            let size = n;
            let pos = vec![-1; n];
            DSU { size, pos }
        }
        pub fn find(&mut self, x: usize) -> usize {
            if self.pos[x] < 0 {
                x
            } else {
                let v = self.pos[x] as usize;
                self.pos[x] = self.find(v) as isize;
                self.pos[x] as usize
            }
        }
        pub fn unite(&mut self, x: usize, y: usize) -> Result<usize, ()> {
            let mut x = self.find(x);
            let mut y = self.find(y);
            if x == y {
                return Err(());
            };
            if self.pos[x] > self.pos[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.pos[x] += self.pos[y];
            self.pos[y] = x as isize;
            Ok(x)
        }
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }
        pub fn size(&mut self, x: usize) -> usize {
            let x = self.find(x);
            -self.pos[x] as usize
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1);m],
        k: usize,
        c: [Usize1; k],
    }

    let mut uf = union_find::DSU::new(n);
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];

    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
        uf.unite(a, b).ok();
    }

    let mut ok = true;
    for &i in &c {
        if !uf.same(c[0], i) {
            ok = false;
            break;
        }
    }

    let ans = if ok { uf.size(c[0]) as isize } else { -1 };

    println!("{}", ans);
}

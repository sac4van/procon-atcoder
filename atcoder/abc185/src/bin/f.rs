use proconio::marker::*;
use proconio::*;

// ---------- begin SegmentTree Point update Range query ----------
mod segment_tree {
    pub struct PURQ<T, F> {
        n: usize,
        a: Vec<T>,
        id: T,
        op: F,
    }
    #[allow(dead_code)]
    impl<T: Clone, F: Fn(&T, &T) -> T> PURQ<T, F> {
        pub fn new(n: usize, id: T, op: F) -> PURQ<T, F> {
            let mut k = 1;
            while k < n {
                k *= 2;
            }
            PURQ {
                n: k,
                a: vec![id.clone(); 2 * k],
                id: id,
                op: op,
            }
        }
        pub fn update(&mut self, x: usize, v: T) {
            let mut k = self.n + x;
            let a = &mut self.a;
            a[k] = v;
            k >>= 1;
            while k > 0 {
                a[k] = (self.op)(&a[2 * k], &a[2 * k + 1]);
                k >>= 1;
            }
        }

        pub fn update_all(&mut self, b: &Vec<T>)
        where
            T: Copy,
        {
            let a = &mut self.a;
            if b.len() > self.n {
                panic!(
                    "Length of update vector should not exceed k={}. [b.len={}]",
                    self.n,
                    b.len()
                );
            }
            for i in 0..self.n {
                a[i + self.n] = if i < b.len() { b[i] } else { self.id.clone() };
            }
            for k in (1..self.n).rev() {
                a[k] = (self.op)(&a[2 * k], &a[2 * k + 1]);
            }
        }

        pub fn find(&self, mut l: usize, mut r: usize) -> T {
            let mut p = self.id.clone();
            let mut q = self.id.clone();
            l += self.n;
            r += self.n;
            let a = &self.a;
            while l < r {
                if (l & 1) == 1 {
                    p = (self.op)(&p, &a[l]);
                    l += 1;
                }
                if (r & 1) == 1 {
                    r -= 1;
                    q = (self.op)(&a[r], &q);
                }
                l >>= 1;
                r >>= 1;
            }
            (self.op)(&p, &q)
        }

        pub fn get_leaf(&self, i: usize) -> T
        where
            T: Copy,
        {
            let a = &self.a;
            if i >= self.n {
                panic!("Index should be less than n={}. [i={}]", self.n, i);
            }
            a[self.n + i]
        }
    }
}
// ---------- end SegmentTree Point update Range query ----------

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        ask : [(usize, Usize1, usize); q]
    }

    let mut seg_tree = segment_tree::PURQ::new(n, 0, |a, b| *a ^ *b);
    seg_tree.update_all(&a);

    for (t, x, y) in ask {
        if t == 1 {
            let ax = seg_tree.get_leaf(x);
            seg_tree.update(x, ax ^ y);
        } else {
            let ans = seg_tree.find(x, y);
            println!("{}", ans);
        }
    }
}

use proconio::*;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    fn s(mut a: usize) -> usize {
        let mut ret = 0;
        while a > 0 {
            ret += a % 10;
            a /= 10;
        }
        ret
    }

    let sa = s(a);
    let sb = s(b);

    let ans = if sa > sb { sa } else { sb };

    println!("{}", ans);
}

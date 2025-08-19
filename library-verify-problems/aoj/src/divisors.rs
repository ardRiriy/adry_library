// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_3_D&lang=ja

use library::math::prime::divisors;
use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }

    let v = divisors(c);
    println!("{}", v.iter().filter(|&c| a <= *c && *c <= b).count());
}

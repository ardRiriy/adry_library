// TODO: verification-helper: PROBLEM https://judge.yosupo.jp/problem/factorize
// TODO: 正しく実装し直す
// use std::iter::repeat;

// use cps::prime::prime_factorization;
// use itertools::Itertools;
// use proconio::input;

// fn main() {
//     input! {
//         mut q: usize,
//     }
//     while q > 0 {
//         solve();
//         q -= 1;
//     }
// }

// fn solve() {
//     input! {
//         n: u64,
//     }
//     if n == 1 {
//         println!("0");
//         return;
//     }
//     let pf = prime_factorization(n);
//     let sum = pf.iter().map(|(_, v)| v).sum::<u64>();
//     println!("{} {}", sum, pf.iter().map(|(k, v)| repeat(*k).take(*v as usize).join(" ")).join(" "));
// }

fn main() {}

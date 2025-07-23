// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_C&lang=en

use library::graph::warshall_floyd::{DefaultWFelm, WFelm, WarshallFloyd};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        e: [(usize, usize, i64); m],
    }

    let g = e.iter()
        .fold(vec![vec![]; n], |mut g, &(u, v, w)| {
            g[u].push((v, w));
            g
        }
    );

    let wf = WarshallFloyd::new(&g, DefaultWFelm);

    if (0..n).any(|i| wf.get(i, i) < 0) {
        println!("NEGATIVE CYCLE");
    } else {
        println!("{}", 
            (0..n).map(|i| {
                (0..n).map(|j| wf.get(i, j))
                    .map(|d| if d == DefaultWFelm.infinity() { "INF".to_string() } else { format!("{d}") })
                    .join(" ")
            }).join("\n")
        );
    }
}
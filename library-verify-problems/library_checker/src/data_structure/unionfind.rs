// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use library::{data_structure::unionfind::UnionFind, utils::input::Input};

fn main() {
    let mut input = Input::new();
    let (n, q) = input.pair::<usize>();
    let mut uf = UnionFind::new(n, |_u, _v| 0);

    for _ in 0..q {
        let (t, u, v) = input.triple::<usize>();
        if t == 0 {
            uf.merge(u, v);
        } else {
            println!("{}", if uf.same(u, v) { "1" } else { "0" });
        }
    }
}

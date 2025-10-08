// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A

use library::{data_structure::segtree::{monoids::RangeMinMonoid, segment_tree::SegmentTree}, utils::input::Input};

fn main() {
    let mut ip = Input::new();
    let (n, q) = ip.pair();
    
    let mut segtree :SegmentTree<RangeMinMonoid<i64>> = SegmentTree::new(n);
    for i in 0..n {
        segtree.set(i, (1<<31) - 1);
    }
    
    for _ in 0..q {
        let t = ip.next::<u8>();
        if t == 0 {
            segtree.set(ip.next(), ip.next());
        } else {
            let (l, r) = ip.pair::<usize>();
            println!("{}", segtree.get(l..=r));
        }
    }
}

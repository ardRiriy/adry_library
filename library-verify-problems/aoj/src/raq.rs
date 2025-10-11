// verification-helper: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_E
use library::{data_structure::segtree::{lazy_segment_tree::LazySegmentTree, monoids::{MapMonoid, RangeSumMonoid}}, utils::input::Input};

struct F;
impl MapMonoid for F {
    type M = RangeSumMonoid<u64>;

    type F = u64;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(f: &Self::F, x: &<Self::M as library::data_structure::segtree::monoids::Monoid>::S) -> <Self::M as library::data_structure::segtree::monoids::Monoid>::S {
        *f+x
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f+g
    }
}


fn main() {
    let mut ip = Input::new();
    let (n, q) = ip.pair();
    let mut lseg :LazySegmentTree<F> = LazySegmentTree::new(n);
    for i in 0..n {
        lseg.set(i, 0);
    }
    for _ in 0..q {
        if ip.next::<u8>() == 0 {
            let (l, r) = ip.pair::<usize>();
            let x = ip.next();
            lseg.update(l..=r, &x);
        } else {
            let i = ip.next::<usize>();
            println!("{}", lseg.get(i..=i));
        }
    }
}
// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_D
use library::{data_structure::segtree::{lazy_segment_tree::LazySegmentTree, monoids::{MapMonoid, Monoid, RangeMinMonoid}}, utils::input::Input};

static ID: u64 = (1<<31)-1;

struct F;
impl MapMonoid for F {
    type M = RangeMinMonoid<u64>;
    type F = u64;

    fn identity_map() -> Self::F {
        ID
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if *f == ID {
            *x
        } else {
            *f
        }
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        if *f == ID {
            *g
        } else {
            *f
        }
    }
}

fn main() {
    let mut ip = Input::new();
    let (n, q) = ip.pair();
    let mut lseg :LazySegmentTree<F> = LazySegmentTree::new(n);
    for i in 0..n {
        lseg.set(i, 2147483647);
    }
    for _ in 0..q {
        if ip.next::<u8>() == 0 {
            let (l, r) = ip.pair::<usize>();
            let val = ip.next();
            lseg.update(l..=r, &val);
        } else {
            let i = ip.next::<usize>();
            println!("{}", lseg.get(i..=i));
        }
    }
}

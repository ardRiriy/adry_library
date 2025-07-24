use library::{data_structure::segment_tree::{Monoid, SegmentTree}, utils::input::Input};
use std::io::{stdout, Write};

#[derive(Debug, Clone, Copy)]
struct Mono;
impl Monoid for Mono {
    type S = u64;

    fn op(a: Self::S, b: Self::S) -> Self::S {
        a + b
    }

    fn id() -> Self::S {
        0
    }
}

fn main() {
    let mut lock = stdout().lock();
    let mut input = Input::new();
    let (n, q) = input.pair::<usize, usize>();
    let a = input.vector::<u64>(n);
    let mut seg :SegmentTree<Mono> = SegmentTree::new(n);

    for (i,ai) in a.iter().enumerate() {
        seg.set(i, *ai);
    }

    for _ in 0..q {
        let t = input.next::<u8>();
        if t==0 {
            let (p,x) = input.pair::<usize, u64>();
            seg.set(p, seg.get(p..=p)+x);
        } else {
            let (l, r) = input.pair::<usize,usize>();
            let _ = writeln!(lock, "{}", seg.get(l..r));
        }

        dbg!(&seg);
    }
}
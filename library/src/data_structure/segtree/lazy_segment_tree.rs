use std::ops::RangeBounds;

use crate::{
    data_structure::segtree::monoids::{MapMonoid, Monoid},
    utils::versatility_functions::unpack_range,
};

/*
 * mapping: 値xにfを作用させる
 * composition: 作用gを行ったあとに、作用fを行う
 *  → 可換なら演算足せばいいし、そうじゃないならgの作用の有無で場合分け
*/

pub struct LazySegmentTree<T: MapMonoid> {
    n: usize,
    len: usize,
    data: Vec<<T::M as Monoid>::S>,
    lazy: Vec<T::F>,
}

impl<T: MapMonoid> LazySegmentTree<T> {
    pub fn new(len: usize) -> Self {
        let mut n = 1;
        while n <= len {
            n <<= 1;
        }
        Self {
            n,
            len,
            data: vec![T::id(); 2 * n],
            lazy: vec![T::identity_map(); 2 * n],
        }
    }

    pub fn set(&mut self, index: usize, value: <T::M as Monoid>::S) {
        self.data[self.n + index - 1] = value;
    }

    fn eval(&mut self, idx: usize) {
        /* 配列のk番目の要素を更新する */

        if self.lazy[idx] == T::identity_map() {
            // 更新なし
            return;
        }

        if idx < self.n - 1 {
            // 子要素に伝播
            self.lazy[idx * 2 + 1] = T::composition(&self.lazy[idx], &self.lazy[idx * 2 + 1]);
            self.lazy[idx * 2 + 2] = T::composition(&self.lazy[idx], &self.lazy[idx * 2 + 2]);
        }

        self.data[idx] = T::mapping(&self.lazy[idx], &self.data[idx]);
        self.lazy[idx] = T::identity_map();
    }

    fn get_range_rec(
        &mut self,
        l: usize,
        r: usize,
        k: usize,
        seg_l: usize,
        seg_r: usize,
    ) -> <T::M as Monoid>::S {
        self.eval(k);
        if seg_r <= l || r <= seg_l {
            return T::id();
        }
        if l <= seg_l && seg_r <= r {
            return self.data[k];
        }

        T::op(
            &self.get_range_rec(l, r, 2 * k + 1, seg_l, (seg_l + seg_r) / 2),
            &self.get_range_rec(l, r, 2 * k + 2, (seg_l + seg_r) / 2, seg_r),
        )
    }

    pub fn get<R: RangeBounds<usize>>(&mut self, range: R) -> <T::M as Monoid>::S {
        let (l, r) = unpack_range(range, 0, self.len);
        self.get_range_rec(l, r, 0, 0, self.n)
    }

    fn update_range_rec(
        &mut self,
        l: usize,
        r: usize,
        k: usize,
        f: &T::F,
        seg_l: usize,
        seg_r: usize,
    ) {
        self.eval(k);
        if l <= seg_l && seg_r <= r {
            self.lazy[k] = T::composition(&f, &self.lazy[k]);
            self.eval(k);
        } else if l < seg_r && seg_l < r {
            self.update_range_rec(l, r, k * 2 + 1, f, seg_l, (seg_l + seg_r) / 2);
            self.update_range_rec(l, r, k * 2 + 2, f, (seg_l + seg_r) / 2, seg_r);
            self.data[k] = T::op(&self.data[2 * k + 1], &self.data[2 * k + 2]);
        }
    }

    pub fn update<R: RangeBounds<usize>>(&mut self, range: R, f: &T::F) {
        let (l, r) = unpack_range(range, 0, self.len);
        self.update_range_rec(l, r, 0, f, 0, self.n);
    }
}

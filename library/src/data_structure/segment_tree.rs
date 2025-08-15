use std::{fmt::{self, Debug, Formatter}, ops::{Bound, RangeBounds}};

pub trait Monoid {
    type S: Copy;
    fn op(a: Self::S, b: Self::S) -> Self::S;
    fn id() -> Self::S;
}

pub struct SegmentTree<T: Monoid> {
    n: usize,
    len: usize,
    data: Vec<T::S>,
}

impl<T: Monoid> SegmentTree<T> {
    pub fn new(len: usize) -> Self {
        let mut n = 1;
        while n < len { n <<= 1; }
        let data = vec![T::id(); 2 * n];
        Self { n, len, data }
    }

    pub fn from_vec(vec: &Vec<T::S>) -> Self {
        let len = vec.len();
        let mut seg = Self::new(len);
        seg.data[seg.n..][..len].copy_from_slice(&vec);
        for i in (1..seg.n).rev() {
            seg.data[i] = T::op(seg.data[i << 1], seg.data[(i << 1) + 1]);
        }
        seg
    }

    pub fn set(&mut self, index: usize, value: T::S) {
        assert!(index < self.len);
        let mut cur = index + self.n;
        self.data[cur] = value;
        while cur > 1 {
            cur >>= 1;
            self.data[cur] = T::op(self.data[cur << 1], self.data[(cur << 1) + 1]);
        }
    }

    pub fn get<R>(&self, range: R) -> T::S
    where
        R: RangeBounds<usize>,
    {
        let l = match range.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
            Bound::Unbounded    => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(&e) => e + 1,
            Bound::Excluded(&e) => e,
            Bound::Unbounded    => self.len,
        };
        self.get_rec(1, 0, self.n, l, r)
    }

    fn get_rec(&self, cur: usize, seg_l: usize, seg_r: usize,
               q_l: usize, q_r: usize) -> T::S
    {
        if seg_r <= q_l || q_r <= seg_l { return T::id(); }
        if q_l <= seg_l && seg_r <= q_r { return self.data[cur]; }
        let mid = (seg_l + seg_r) >> 1;
        let left  = self.get_rec(cur << 1,     seg_l, mid, q_l, q_r);
        let right = self.get_rec((cur << 1)+1, mid,   seg_r, q_l, q_r);
        T::op(left, right)
    }
}

impl<T> Debug for SegmentTree<T>
where
    T: Monoid,
    T::S: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;
        let total = self.data.len() - 1;
        if total == 0 {
            return write!(f, "(empty)");
        }

        let leaves = (total + 1) / 2;
        debug_assert!(leaves.is_power_of_two());
        let h = leaves.trailing_zeros() as usize;

        let repr: Vec<String> = self.data
            .iter()
            .skip(1)
            .map(|v| format!("{:?}", v))
            .collect();
        let w  = repr.iter().map(|s| s.len()).max().unwrap();
        let sep = 1;
        let unit = w + sep;
        let line_len = leaves * unit - sep;

        let mut idx = 0;
        for d in 0..=h {
            let nodes = 1 << d;
            let block = 1 << (h - d);
            let stride = block * unit;
            let mut line = vec![' '; line_len];

            for i in 0..nodes {
                if idx >= repr.len() { break; }
                let s = &repr[idx];
                let center = i * stride + stride / 2;
                let start  = center - s.len() / 2;

                for (j, ch) in s.chars().enumerate() {
                    let pos = start + j;
                    if pos < line_len { line[pos] = ch; }
                }
                idx += 1;
            }
            // 右端の余白を落として印字
            let end = line.iter().rposition(|&c| c != ' ').map_or(0, |p| p + 1);
            writeln!(f, "{}", line[..end].iter().collect::<String>())?;
        }
        Ok(())
    }
}
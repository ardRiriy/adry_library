#[derive(Debug)]
pub struct CumulativeSum<T> {
    sum: Vec<T>
}

impl<T> CumulativeSum<T> 
where T: num_traits::Num + Clone + Ord
{
    pub fn new(init_vec: &Vec<T>) -> CumulativeSum<T> {
        let sum = init_vec
            .iter()
            .enumerate()
            .fold(vec![T::zero()], |mut sum, (idx, x)| {
                let next :T = sum[idx].clone() + x.clone();
                sum.push(next);
                sum
            });
        CumulativeSum { sum }
    }

    // 区間[l, r)の総和を答える
    pub fn get<R>(&self, range: R) -> T
    where
        R: std::ops::RangeBounds<usize>
    {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => self.sum.len()-1,
        };

        self.sum[end].clone() - self.sum[start].clone()
    }

    // key以上で最小のindexを返却する
    // a[i] > 0を期待して標準のbinary_searchを使う
    pub fn binary_search(&self, key: T) -> Result<usize, usize> {
        self.sum.binary_search(&key)
    }
}

mod tests {
    #[test]
    fn cumulative_sum_get() {
        use super::CumulativeSum;
        let v = vec![1, 3, 8];
        let cs = CumulativeSum::new(&v);
        assert_eq!(1 + 3 + 8, cs.get(0..3));
        assert_eq!(1 + 3, cs.get(0..2));
        assert_eq!(3 + 8, cs.get(1..3));
        assert_eq!(3, cs.get(1..2));
        assert_eq!(1 + 3 + 8, cs.get(0..=2));
    }
}

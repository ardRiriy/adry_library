pub trait VecLibs<T: std::cmp::Ord> {
    fn lower_bound(&self, elm: T) -> usize;
    fn upper_bound(&self, elm: T) -> usize;
}

impl<T: std::cmp::Ord> VecLibs<T> for Vec<T> {
    fn lower_bound(&self, elm: T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left < right {
            let mid = (left + right) / 2;
            if self[mid] < elm {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    fn upper_bound(&self, elm: T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left < right {
            let mid = (left + right) / 2;
            if self[mid] <= elm {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

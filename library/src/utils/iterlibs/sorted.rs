/// iterを受け取り、ソート済みのiterを返す

pub struct Sorted<I>
where
    I: Iterator,
{
    inner: I,
}

impl<I> Iterator for Sorted<I>
where
    I: Iterator,
    I::Item: Ord,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub trait SortedIterator: Iterator + Sized {
    fn sorted(self) -> Sorted<std::vec::IntoIter<Self::Item>>
    where
        Self::Item: Ord,
    {
        let mut v: Vec<Self::Item> = self.collect();
        v.sort();
        Sorted {
            inner: v.into_iter(),
        }
    }
}

impl<T> SortedIterator for T where T: Iterator + Sized {}

#[cfg(test)]
mod tests {
    use crate::utils::iterlibs::{collect::CollectIter, sorted::SortedIterator};

    #[test]
    fn with_elements() {
        let v = vec![3, 1, 4, 1, 5];
        assert_eq!(
            v.iter().copied().sorted().collect_vec(),
            vec![1, 1, 3, 4, 5]
        );
    }

    #[test]
    fn empty() {
        let v: Vec<i32> = Vec::new();
        assert_eq!(v.iter().copied().sorted().collect_vec(), Vec::new());
    }
}

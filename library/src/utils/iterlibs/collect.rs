pub trait CollectIter {
    type Item;
    fn collect_vec(self) -> Vec<Self::Item>;
}

impl<I> CollectIter for I 
where
    I: Iterator
{
    type Item = I::Item;
    
    fn collect_vec(mut self) -> Vec<Self::Item> {
        match self.next() {
            None => Vec::new(),
            Some(val) => {
                let mut res = Vec::new();
                res.push(val);
                for item in self  {
                    res.push(item);
                }
                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::iterlibs::collect::CollectIter;

    #[test]
    fn with_elements() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(
            v.iter().map(|vi| *vi + 1).collect_vec(),
            vec![2, 3, 4, 5]
        );
    }
    
    #[test]
    fn empty() {
        let v :Vec<i32> = vec![];
        assert_eq!(
            v.iter().map(|vi| *vi + 1).collect_vec(),
            vec![]
        );
    }
}

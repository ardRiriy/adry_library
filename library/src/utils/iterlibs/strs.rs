use std::fmt::Display;

pub trait StrUtilIter {
    type Item;
    fn join(self, sep: &str) -> String;
}

impl<I> StrUtilIter for I
where
    I: Iterator,
    I::Item: Display,
{
    type Item = I::Item;

    fn join(mut self, sep: &str) -> String {
        match self.next() {
            None => String::new(),
            Some(first) => {
                let mut res = first.to_string();
                for item in self {
                    res.push_str(sep);
                    res.push_str(&item.to_string());
                }
                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::iterlibs::strs::StrUtilIter;

    #[test]
    fn with_elements() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(v.iter().join(" "), "1 2 3 4".to_string());
    }

    #[test]
    fn empty() {
        let v: Vec<i32> = Vec::new();
        assert_eq!(v.iter().join(" "), "".to_string());
    }
}

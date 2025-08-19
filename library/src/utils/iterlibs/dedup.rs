use std::iter::Peekable;

pub struct Rle<I: Iterator> {
    iter: Peekable<I>,
}

impl<I> Iterator for Rle<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let mut len = 1;
        let val = self.iter.next()?;

        while let Some(nxt) = self.iter.peek() {
            if &val == nxt {
                self.iter.next()?;
                len += 1;
            } else {
                break;
            }
        }
        Some((len, val))
    }
}

pub trait RleItertor: Iterator + Sized {
    fn rle(self) -> Rle<Self>
    where
        Self::Item: PartialEq,
    {
        Rle {
            iter: self.peekable(),
        }
    }
}

impl<I: Iterator> RleItertor for I {}

pub struct Dedup<I>
where
    I: Iterator,
{
    inner: Rle<I>,
}

impl<I> Iterator for Dedup<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, val)| val)
    }
}

pub trait DedupIterator: Iterator + Sized {
    fn dedup(self) -> Dedup<Self>
    where
        Self::Item: PartialEq,
    {
        Dedup { inner: self.rle() }
    }
}
impl<I: Iterator> DedupIterator for I {}

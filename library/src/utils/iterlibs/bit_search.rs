/// 長さ N の iter から 2^N 個の状態を順に生成する bit 全探索
///
/// 各状態は `Vec<(T, bool)>` で返される。要素は元の iter と同じ順で並び、
/// `bool` がその要素を選択したか否かを表す。

pub struct BitSearch<T> {
    items: Vec<T>,
    state: usize,
    end: usize,
}

impl<T: Clone> Iterator for BitSearch<T> {
    type Item = Vec<(T, bool)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state >= self.end {
            return None;
        }
        let result = self
            .items
            .iter()
            .enumerate()
            .map(|(i, x)| (x.clone(), (self.state >> i) & 1 == 1))
            .collect();
        self.state += 1;
        Some(result)
    }
}

pub trait BitSearchIterator: Iterator + Sized {
    fn bit_search(self) -> BitSearch<Self::Item>
    where
        Self::Item: Clone,
    {
        let items: Vec<Self::Item> = self.collect();
        let end = 1usize << items.len();
        BitSearch {
            items,
            state: 0,
            end,
        }
    }
}

impl<I: Iterator> BitSearchIterator for I {}

#[cfg(test)]
mod tests {
    use crate::utils::iterlibs::{bit_search::BitSearchIterator, collect::CollectIter};

    #[test]
    fn length_three() {
        let v = vec![1, 2, 3];
        let states = v.iter().copied().bit_search().collect_vec();
        assert_eq!(states.len(), 8);
        assert_eq!(
            states[0],
            vec![(1, false), (2, false), (3, false)]
        );
        assert_eq!(
            states[1],
            vec![(1, true), (2, false), (3, false)]
        );
        assert_eq!(
            states[5],
            vec![(1, true), (2, false), (3, true)]
        );
        assert_eq!(states[7], vec![(1, true), (2, true), (3, true)]);
    }

    #[test]
    fn empty_yields_one_state() {
        let v: Vec<i32> = vec![];
        let states = v.iter().copied().bit_search().collect_vec();
        assert_eq!(states.len(), 1);
        assert_eq!(states[0], vec![]);
    }

    #[test]
    fn subset_sum() {
        let v = vec![1, 2, 4, 8];
        let mut sums = v
            .iter()
            .copied()
            .bit_search()
            .map(|state| {
                state
                    .into_iter()
                    .filter(|(_, b)| *b)
                    .map(|(x, _)| x)
                    .sum::<i32>()
            })
            .collect_vec();
        sums.sort();
        assert_eq!(sums, (0..16).collect_vec());
    }
}

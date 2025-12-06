use std::{collections::BTreeSet, ops::RangeBounds};

use crate::utils::{chlibs::ChLibs, integer::Integer, versatility_functions::unpack_range};

pub struct IntervalSet<T: Integer> {
    sum: T,
    set: BTreeSet<(T, T)>,
}

impl<T: Integer> IntervalSet<T> {
    pub fn new() -> Self {
        Self {
            sum: T::zero(),
            set: BTreeSet::new(),
        }
    }

    pub fn insert<R>(&mut self, range: R) -> T
    where
        R: RangeBounds<T>,
    {
        let (l, r) = unpack_range(range, T::MIN, T::MAX);

        let start_l = if let Some(inv) = self.set.range(..(l, T::MAX)).last() {
            if inv.1 < l {
                l
            } else {
                inv.0
            }
        } else {
            l
        };

        let mut removes = vec![];
        let mut sub = T::zero();
        let mut max_r = r;
        for inv in self.set.range((start_l, T::zero())..) {
            if inv.0 > r {
                break;
            }
            sub += inv.1 - inv.0;
            max_r.chmax(inv.1);
            removes.push(*inv);
            if inv.1 >= r {
                break;
            }
        }
        let plus = max_r - start_l - sub;
        self.sum += plus;

        for iv in removes {
            if !self.set.remove(&iv) {
                panic!("interval {:?} is not found on set", iv);
            }
        }
        self.set.insert((start_l, max_r));

        plus
    }

    pub fn erase<R>(&mut self, range: R) -> T
    where
        R: RangeBounds<T>,
    {
        let (l, r) = unpack_range(range, T::MIN, T::MAX);
        let mut removes = vec![];
        let mut add_backs = vec![];

        let start_l = if let Some(inv) = self.set.range(..(l, T::MAX)).last() {
            if inv.1 < l {
                l
            } else {
                inv.0
            }
        } else {
            l
        };

        let mut sub = T::zero();
        for inv in self.set.range((start_l, T::zero())..) {
            if inv.0 >= r {
                break;
            }
            if l <= inv.0 && inv.1 <= r {
                sub += inv.1 - inv.0;
                removes.push(*inv);
            } else if inv.0 < l && r < inv.1 {
                sub += r - l;
                removes.push(*inv);
                add_backs.push((inv.0, l));
                add_backs.push((r, inv.1));
            } else if inv.0 < l && l < inv.1 && inv.1 <= r {
                sub += inv.1 - l;
                removes.push(*inv);
                add_backs.push((inv.0, l));
            } else if l <= inv.0 && inv.0 < r && r < inv.1 {
                sub += r - inv.0;
                removes.push(*inv);
                add_backs.push((r, inv.1));
            }
        }

        self.sum -= sub;
        for iv in removes {
            if !self.set.remove(&iv) {
                panic!("interval {:?} is not found on set", iv);
            }
        }
        for iv in add_backs {
            self.set.insert(iv);
        }

        sub
    }

    pub fn contains(&self, x: T) -> bool {
        if let Some(inv) = self.set.range(..(x + T::from_i32(1), T::MAX)).last() {
            inv.0 <= x && x < inv.1
        } else {
            false
        }
    }

    pub fn covered_length(&self) -> T {
        self.sum
    }
}

impl<T: Integer> IntervalSet<T> {
    pub fn iter(&self) -> impl Iterator<Item = &(T, T)> {
        self.set.iter()
    }
}

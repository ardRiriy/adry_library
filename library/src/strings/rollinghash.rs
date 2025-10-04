use std::ops::{Bound, RangeBounds};

use crate::misc::rand::Pcg32;

static MODULO: u128 = (1<<61)-1;

pub struct RollingHash {
    n: usize,
    pub base: u128,
    prefix: Vec<u128>,
    power: Vec<u128>,
}

impl RollingHash {
    pub fn new(s: &String, base: Option<u128>) -> Self {
        let mut rng = Pcg32::new();
        let base = if let Some(val) = base {
            val
        } else {
            rng.gen_range(100..u32::MAX as u128)
        };
        let n= s.len();
        let mut prefix = vec![0; n+1];
        let mut power = vec![1; n+1];
        for (i, c) in s.chars().enumerate() {
            let ci = c as u128;
            prefix[i+1] = (prefix[i] * base + ci) % MODULO;
            power[i+1] = (power[i] * base) % MODULO;
        }

        Self { n, base, prefix, power }
    }
    
    pub fn hash<R>(&self, range: R) -> u128
    where 
        R: RangeBounds<usize>,
    {
        let l = match range.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(&e) => e+1,
            Bound::Excluded(&e) => e,
            Bound::Unbounded => self.n,
        };
        let res = self.prefix[r] + MODULO - ((self.power[r-l] * self.prefix[l]) % MODULO);
        return res % MODULO;
    }
    
}
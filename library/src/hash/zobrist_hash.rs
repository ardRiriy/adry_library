use std::collections::{BTreeMap, BTreeSet};

use crate::misc::rand::Pcg32;

const MOD: u64 = (1 << 61) - 1;
pub struct ZobristHash<T> {
    map: BTreeMap<T, u64>,
    set: BTreeSet<u64>,
    rng: Pcg32,
}

impl<T: Ord> Default for ZobristHash<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> ZobristHash<T> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            set: BTreeSet::new(),
            rng: Pcg32::new(),
        }
    }

    pub fn get(&mut self, key: T) -> u64 {
        if let Some(val) = self.map.get(&key) {
            *val
        } else {
            let mut val = self.rng.gen_range(0..=MOD);
            while self.set.contains(&val) {
                val = self.rng.gen_range(0..MOD);
            }
            self.map.insert(key, val);
            self.set.insert(val);
            val
        }
    }
}

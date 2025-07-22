const MOD: u64 = (1 << 61) - 1;
pub struct ZobristHash<T> {
    map: std::collections::BTreeMap<T, u64>,
    set: std::collections::BTreeSet<u64>,
    rng: rand::rngs::ThreadRng,
}

impl<T: Ord> ZobristHash<T> {
    pub fn new() -> Self {
        use std::collections::*;
        use rand::thread_rng;
        Self { 
            map: BTreeMap::new(), 
            set: BTreeSet::new(),
            rng: thread_rng(),
        }
    }

    pub fn get(&mut self, key: T) -> u64 {
        use rand::Rng;
        if let Some(val) = self.map.get(&key) {
            return *val;
        } else {
            let mut val = self.rng.gen_range(0..=MOD);
            while self.set.contains(&val) {
                val = self.rng.gen_range(0..MOD);
            }
            self.map.insert(key, val);
            self.set.insert(val);
            return val;
        }
    }
}
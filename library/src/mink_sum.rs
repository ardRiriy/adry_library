// 小さい方からK個を管理し、和を答える
// 基本的には問題に応じて書き換える想定
#[derive(Debug)]
pub struct MinK {
    k: usize,
    cnt: usize,
    small_k_map: std::collections::BTreeMap<u64, u64>,
    big_map: std::collections::BTreeMap<u64, u64>,
    sum_small_k: u64,
}

impl MinK {
    pub fn new(k: usize) -> Self {
        assert_ne!(k, 0);
        MinK {
            k,
            cnt: 0,
            small_k_map: std::collections::BTreeMap::new(),
            big_map: std::collections::BTreeMap::new(),
            sum_small_k: 0,
        }
    }

    pub fn add(&mut self, x: u64) {
        *self.small_k_map.entry(x).or_insert(0) += 1;
        self.cnt += 1;
        self.sum_small_k += x;


        if self.cnt > self.k {
            let (&key, _) = self.small_k_map.last_key_value().unwrap();
            // 大きい方から管理したい場合は↓を使う
            // let (&key, _) = self.small_k_map.first_key_value().unwrap();
            remove_from_map(&mut self.small_k_map, key);
            self.cnt -= 1;
            *self.big_map.entry(key).or_insert(0) += 1;

            self.sum_small_k -= key;
        }
    }

    pub fn remove(&mut self, x: u64) {
        if self.big_map.contains_key(&x) {
            remove_from_map(&mut self.big_map, x);
        } else {
            remove_from_map(&mut self.small_k_map, x);
            self.sum_small_k -= x;

            if let Some((&key, _)) = self.big_map.first_key_value() { // 大きい方からK個を管理したい場合は、last_key_value()に書き換える
                *self.small_k_map.entry(key).or_insert(0) += 1;
                remove_from_map(&mut self.big_map, key);
                self.sum_small_k += key;
            } else {
                self.cnt -= 1;
            }
        }
    }

    pub fn ans(&self) -> u64 {
        self.sum_small_k
    }

    pub fn size(&self) -> usize {
        self.cnt
    }
}

fn remove_from_map(map: &mut std::collections::BTreeMap<u64, u64>, key: u64) {
    if map[&key] == 1 {
        map.remove(&key);
    } else {
        map.insert(key, map[&key] - 1);
    }
}

#[derive(Debug)]
pub struct MaxK {
    k: usize,
    cnt: usize,
    small_k_map: std::collections::BTreeMap<u64, u64>,
    big_map: std::collections::BTreeMap<u64, u64>,
    sum_small_k: u64,
}

impl MaxK {
    pub fn new(k: usize) -> Self {
        assert_ne!(k, 0);
        Self {
            k,
            cnt: 0,
            small_k_map: std::collections::BTreeMap::new(),
            big_map: std::collections::BTreeMap::new(),
            sum_small_k: 0,
        }
    }

    pub fn add(&mut self, x: u64) {
        *self.small_k_map.entry(x).or_insert(0) += 1;
        self.cnt += 1;
        self.sum_small_k += x;


        if self.cnt > self.k {
            // let (&key, _) = self.small_k_map.last_key_value().unwrap();
            // 大きい方から管理したい場合は↓を使う
            let (&key, _) = self.small_k_map.first_key_value().unwrap();
            remove_from_map(&mut self.small_k_map, key);
            self.cnt -= 1;
            *self.big_map.entry(key).or_insert(0) += 1;

            self.sum_small_k -= key;
        }
    }

    pub fn remove(&mut self, x: u64) {
        if self.big_map.contains_key(&x) {
            remove_from_map(&mut self.big_map, x);
        } else {
            remove_from_map(&mut self.small_k_map, x);
            self.sum_small_k -= x;

            if let Some((&key, _)) = self.big_map.last_key_value() { // 大きい方からK個を管理したい場合は、last_key_value()に書き換える
                *self.small_k_map.entry(key).or_insert(0) += 1;
                remove_from_map(&mut self.big_map, key);
                self.sum_small_k += key;
            } else {
                self.cnt -= 1;
            }
        }
    }

    pub fn ans(&self) -> u64 {
        self.sum_small_k
    }

    pub fn size(&self) -> usize {
        self.cnt
    }
}

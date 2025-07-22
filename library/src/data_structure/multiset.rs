#[derive(Debug, Clone)]
pub struct MultiSet<T, U> {
    map: std::collections::BTreeMap<T, U>,
}

impl<T, U> MultiSet<T, U> 
where T: Ord + Copy, U: num::PrimInt + std::ops::AddAssign
{
    pub fn new() -> Self {
        use std::collections::BTreeMap;
        MultiSet { map: BTreeMap::new() }
    }

    /* keyをsizeだけ増やして新しい値を返す */
    pub fn add(&mut self, key: T, size: U) -> U {
        *self.map.entry(key).or_insert(U::zero()) += size;
        *self.map.get(&key).unwrap()
    }

    /* sizeだけ減らして新しい値を返す */
    pub fn remove(&mut self, key: T, size: U) -> Option<U> {
        if let Some(&current_size) = self.map.get(&key) {
            if current_size > size {
                let new_size = current_size - size;
                self.map.insert(key, new_size);
                Some(new_size)
            } else {
                self.map.remove(&key);
                Some(U::zero())
            }
        } else {
            None
        }
    }

    pub fn max_key(&self) -> Option<(&T, &U)> {
        self.map.last_key_value()
    }

    pub fn min_key(&self) -> Option<(&T, &U)> {
        self.map.first_key_value()
    } 

    pub fn get(&self, key: T) -> Option<U> {
        if let Some(&v) = self.map.get(&key) {
            Some(v)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<T, U> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::btree_map::IterMut<T, U> {
        self.map.iter_mut()
    }
}

impl<T, U> IntoIterator for MultiSet<T, U> {
    type Item = (T, U);
    type IntoIter = std::collections::btree_map::IntoIter<T, U>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<'a, T, U> IntoIterator for &'a MultiSet<T, U> {
    type Item = (&'a T, &'a U);
    type IntoIter = std::collections::btree_map::Iter<'a, T, U>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

impl<'a, T, U> IntoIterator for &'a mut MultiSet<T, U> {
    type Item = (&'a T, &'a mut U);
    type IntoIter = std::collections::btree_map::IterMut<'a, T, U>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter_mut()
    }
}

pub trait PotentialMergeOp<T> {
    fn identity() -> T;
    fn merge(a: T, b: T) -> T;
    fn invert(a: T) -> T;
}

pub struct DefaultPotentialMergeOp;
impl<T: num::PrimInt + std::ops::Neg<Output=T>> PotentialMergeOp<T> for DefaultPotentialMergeOp {
    fn identity() -> T {
        T::zero()
    }

    fn merge(a: T, b: T) -> T {
        a + b
    }

    fn invert(a: T) -> T {
        -a
    }
}

impl Default for DefaultPotentialMergeOp {
    fn default() -> Self {
        DefaultPotentialMergeOp
    }
}

pub struct PotentialityUnionfind<T, F: PotentialMergeOp<T> + Default> {
    vertex: Vec<usize>,
    diff_weights: Vec<T>,
    #[allow(dead_code)]
    merge_op: F,
}

impl<T: num::PrimInt + std::ops::Neg<Output=T>, F: PotentialMergeOp<T> + Default> PotentialityUnionfind<T, F> {
    pub fn new(size: usize, op: Option<F>) -> Self {
        let op = op.unwrap_or_default();
        Self {
            vertex: vec![!1; size],
            diff_weights: vec![T::zero(); size],
            merge_op: op,
        }
    }

    pub fn leader(&mut self, u: usize) -> usize {
        let elm = self.vertex[u];
        if elm > self.vertex.len() {
            u
        } else {
            let parent = self.leader(elm);
            self.diff_weights[u] = F::merge(self.diff_weights[u], self.diff_weights[elm]);

            self.vertex[u] = parent;
            self.vertex[u]
        }
    }

    pub fn same(&mut self, u: usize, v: usize) -> bool {
        self.leader(u) == self.leader(v)
    }

    // マージして重みを返す
    // すでに同じ場合は「既存の情報と矛盾しないか？」を判定して、
    // 矛盾する場合はErrを返す
    pub fn merge(&mut self, u: usize, v: usize, w: T) -> Result<T, T> {
        if let Ok(diff) = self.diff(u, v) {
            return if w == diff {
                Ok(diff)
            } else {
                Err(diff)
            }
        }

        // size(u) >= size(v) となるようにswap
        if self.size(u) < self.size(v) {
            self.merge(v, u, F::invert(w))?;
            return Ok(F::invert(w));
        }

        let w_sub = F::merge(w, F::merge(self.diff_weights[u], F::invert(self.diff_weights[v])));

        // vの親をuに変更
        let v_leader = self.leader(v);
        let u_leader = self.leader(u);
        let merged_size = !(self.size(u) + self.size(v));
        self.vertex[v_leader] = u_leader;
        self.diff_weights[v_leader] = w_sub;
        self.vertex[u_leader] = merged_size;


        Ok(w)
    }

    pub fn size(&mut self, u: usize) -> usize {
        let idx = self.leader(u);
        !self.vertex[idx]
    }

    // u, vが違う集合の場合はErrとして返す
    pub fn diff(&mut self, u: usize, v: usize) -> Result<T, ()> {
        if self.same(u, v) {
            Ok(F::merge(self.weight(v), F::invert(self.weight(u))))
        } else {
            Err(())
        }
    }

    fn weight(&mut self, u: usize) -> T {
        self.leader(u);
        self.diff_weights[u]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_potentiality_unionfind() {
        let mut uf :PotentialityUnionfind<i32, DefaultPotentialMergeOp> = PotentialityUnionfind::new(6, None);
        assert_eq!(uf.merge(0, 1, 1), Ok(1));
        assert_eq!(uf.merge(1, 2, 3), Ok(3));
        assert_eq!(uf.merge(2, 3, 1), Ok(1));
        assert_eq!(uf.merge(1, 3, 4), Ok(4));
        assert_eq!(uf.merge(3, 4, 2), Ok(2));
        assert_eq!(uf.merge(0, 4, 3), Err(7));
        assert!(uf.same(0, 4));
        assert_eq!(uf.diff(0, 4), Ok(7));
        assert_eq!(uf.diff(0, 5), Err(()));
    }
}
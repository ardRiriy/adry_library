use crate::misc::modint::Modint;

pub struct ChooseMod<const MOD: u64> {
    v: Vec<Modint<MOD>>,
}

impl<const MOD: u64> ChooseMod<MOD> {
    pub fn new(n: usize) -> Self {
        let mut v = vec![Modint::new(1); n+1];

        for i in 0..n {
            v[i+1] = v[i] * (i+1) as u64;
        }

        Self { v }
    }

    pub fn ncr(&self, n: usize, r: usize) -> Modint<MOD> {
        if n < r {
            return Modint::new(0);
        }

        self.v[n] / self.v[n-r] / self.v[r]
    }
    
}

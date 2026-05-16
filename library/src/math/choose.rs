use crate::misc::modint::Modint;

pub struct ChooseMod<const MOD: u64> {
    fact: Vec<Modint<MOD>>,
    inv_fact: Vec<Modint<MOD>>,
}

impl<const MOD: u64> ChooseMod<MOD> {
    pub fn new(n: usize) -> Self {
        let mut fact = vec![Modint::new(1); n + 1];
        for i in 0..n {
            fact[i + 1] = fact[i] * (i + 1) as u64;
        }
        let mut inv_fact = vec![Modint::new(1); n + 1];
        inv_fact[n] = Modint::new(1) / fact[n]; // ここだけ extended_gcd を1回
        for i in (0..n).rev() {
            inv_fact[i] = inv_fact[i + 1] * (i + 1) as u64;
        }
        Self { fact, inv_fact }
    }
    pub fn ncr(&self, n: usize, r: usize) -> Modint<MOD> {
        if n < r {
            return Modint::new(0);
        }
        self.fact[n] * self.inv_fact[n - r] * self.inv_fact[r]
    }
}

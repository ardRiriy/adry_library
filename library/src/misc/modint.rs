use std::fmt::Display;

use crate::math::euclidean::extended_gcd;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Modint<const MOD: u64> {
    pub value: u64,
}

impl<const MOD: u64> Modint<MOD> {
    pub fn new(value: u64) -> Self {
        Self { value: value % MOD }
    }

    pub fn raw(value: u64) -> Self {
        Self { value }
    }
    
    // mod MODにおけるvalの逆元
    pub fn inv(val: u64) -> Self {
        if let Some((x,_,_)) = extended_gcd(val as i128, MOD as i128) {
            Self::new(x as u64)
        } else {
            panic!("No inverse exists for {} in modulo {}", val, MOD);
        }
    }

    pub fn pow(self, exp: u32) -> Self {
        let mut base = self.value;
        let mut result = Self::new(1);
        let mut exp = exp;

        while exp > 0 {
            if exp & 1 == 1 {
                result *= base;
            }
            base *= base;
            exp >>= 1;
        }
        result
    }
}

impl<const MOD: u64> From<Modint<MOD>> for u64 {
    fn from(val: Modint<MOD>) -> Self {
        val.value
    }
}

impl<const MOD: u64, Rhs> std::ops::Add<Rhs> for Modint<MOD> 
where
    Rhs: Into<u64>,
{
    type Output = Self;
    fn add(self, other: Rhs) -> Self {
        Self::new(self.value + other.into())
    }
}

impl<const MOD: u64, Rhs> std::ops::Sub<Rhs> for Modint<MOD>
where
    Rhs: Into<u64>,
{
    type Output = Self;
    fn sub(self, other: Rhs) -> Self {
        Self::new(self.value + MOD - other.into())
    }
}

impl<const MOD: u64, Rhs> std::ops::Mul<Rhs> for Modint<MOD>
where
    Rhs: Into<u64>,
{
    type Output = Self;
    fn mul(self, other: Rhs) -> Self {
        Self::new(self.value * other.into())
    }
}

impl<const MOD: u64> std::ops::Neg for Modint<MOD> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(MOD - self.value)
    }
}

impl<Rhs, const MOD: u64> std::ops::AddAssign<Rhs> for Modint<MOD>
where
    Rhs: Into<u64>,
{
    fn add_assign(&mut self, other: Rhs) {
        self.value = (self.value + other.into()) % MOD;
    }
}

impl<Rhs, const MOD: u64> std::ops::SubAssign<Rhs> for Modint<MOD>
where
    Rhs: Into<u64>,
{
    fn sub_assign(&mut self, other: Rhs) {
        self.value = (self.value + MOD - other.into()) % MOD;
    }
}

impl<Rhs, const MOD: u64> std::ops::MulAssign<Rhs> for Modint<MOD> 
where
    Rhs: Into<u64>,
{
    fn mul_assign(&mut self, other: Rhs) {
        self.value = (self.value * other.into()) % MOD;
    }
}

impl<Rhs, const MOD: u64> std::ops::Div<Rhs> for Modint<MOD>
where
    Rhs: Into<u64>,
{
    type Output = Self;
    fn div(self, other: Rhs) -> Self {
        let inv = Self::inv(other.into());
        self * inv
    }
}

macro_rules! impl_from_prim {
    ($($t:ty),*) => {$(
        impl<const MOD: u64> From<$t> for Modint<MOD> {
            fn from(x: $t) -> Self {
                // i128 に拡張してから剰余を取れば負数も安全に扱える
                let v = (x as i128).rem_euclid(MOD as i128) as u64;
                Self::new(v)
            }
        }
    )*};
}
impl_from_prim!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);


impl<const MOD: u64> Display for Modint<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}


// テスト
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_modint() {
        let a = Modint::<5>::new(3);
        let b = Modint::<5>::new(9002);
        assert_eq!(a.value, 3);
        assert_eq!(b.value, 2);
        assert_eq!((a + b).value, 0);
        assert_eq!((a - b).value, 1);
        assert_eq!((a * b).value, 1);
    }

    #[test]
    fn test_modint_inv() {
        let a = Modint::<11>::new(6);
        let inv_a = Modint::<11>::inv(6);
        assert_eq!(a * inv_a, Modint::<11>::new(1));
    }

    #[test]
    fn test_modint_pow() {
        let a = Modint::<11>::new(2);
        assert_eq!(a.pow(3).value, 8);
        assert_eq!(a.pow(10).value, 1);
        assert_eq!(a.pow(20).value, 1);
    }
}
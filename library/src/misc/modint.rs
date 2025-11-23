use std::{
    fmt::{write, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::{math::euclidean::extended_gcd, utils::integer::Integer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Modint<const MOD: u64> {
    pub value: u64,
}

impl<const MOD: u64> Modint<MOD> {
    pub fn new<T: Integer>(val: T) -> Self {
        Self {
            value: Self::val_mod(val),
        }
    }

    #[inline]
    fn val_mod<T: Integer>(val: T) -> u64 {
        if val < T::zero() {
            ((val % T::from_u64(MOD) + T::from_u64(MOD)).to_u64()) % MOD
        } else {
            val.to_u64() % MOD
        }
    }

    pub fn inv(val: u64) -> Result<Self, ()> {
        if let Some((x, _, _)) = extended_gcd(val as i64, MOD as i64) {
            let value = if x < 0 {
                (x + MOD as i64) as u64
            } else {
                x as u64
            };
            Ok(Self { value })
        } else {
            Err(())
        }
    }

    pub fn pow(&self, k: u32) -> Self {
        let mut res = Self::new(1);
        let mut cur = *self;
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                res *= cur;
            }
            cur *= cur;
            k >>= 1;
        }
        res
    }
}

impl<const MOD: u64> Add for Modint<MOD> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}
impl<const MOD: u64> AddAssign for Modint<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        self.value = (self.value + rhs.value) % MOD;
    }
}

impl<const MOD: u64> Sub for Modint<MOD> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(MOD + self.value - rhs.value)
    }
}
impl<const MOD: u64> SubAssign for Modint<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value = (MOD + self.value - rhs.value) % MOD;
    }
}
impl<const MOD: u64> Mul for Modint<MOD> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value)
    }
}
impl<const MOD: u64> MulAssign for Modint<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = (self.value * rhs.value) % MOD;
    }
}
impl<const MOD: u64> Div for Modint<MOD> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let rhs_inv = Self::inv(rhs.value).unwrap();
        self * rhs_inv
    }
}
impl<const MOD: u64> DivAssign for Modint<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        let rhs_inv = Self::inv(rhs.value).unwrap();
        *self = *self * rhs_inv;
    }
}

impl<const MOD: u64, T: Integer> Add<T> for Modint<MOD> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        self + Self::new(rhs)
    }
}

impl<const MOD: u64, T: Integer> AddAssign<T> for Modint<MOD> {
    fn add_assign(&mut self, rhs: T) {
        *self += Self::new(rhs);
    }
}

impl<const MOD: u64, T: Integer> Sub<T> for Modint<MOD> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        self - Self::new(rhs)
    }
}

impl<const MOD: u64, T: Integer> SubAssign<T> for Modint<MOD> {
    fn sub_assign(&mut self, rhs: T) {
        *self -= Self::new(rhs);
    }
}

impl<const MOD: u64, T: Integer> Mul<T> for Modint<MOD> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self * Self::new(rhs)
    }
}

impl<const MOD: u64, T: Integer> MulAssign<T> for Modint<MOD> {
    fn mul_assign(&mut self, rhs: T) {
        *self *= Self::new(rhs);
    }
}

impl<const MOD: u64, T: Integer> Div<T> for Modint<MOD> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self / Self::new(rhs)
    }
}

impl<const MOD: u64, T: Integer> DivAssign<T> for Modint<MOD> {
    fn div_assign(&mut self, rhs: T) {
        *self /= Self::new(rhs);
    }
}

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
    fn test_modint_pow() {
        let a = Modint::<11>::new(2);
        assert_eq!(a.pow(3).value, 8);
        assert_eq!(a.pow(10).value, 1);
        assert_eq!(a.pow(20).value, 1);
    }

    #[test]
    fn test_modint_div() {
        assert_eq!(
            Modint::<11>::new(3) / Modint::<11>::new(2),
            Modint::<11>::new(7)
        );
        assert_eq!(Modint::<11>::new(8) / 2u64, Modint::<11>::new(4));
    }
}

use std::{convert::TryFrom, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shr, Sub, SubAssign}};

pub trait Integer:
    Copy
    + Default
    + Ord
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
    + TryFrom<i32>
    + Shr<u32, Output = Self>
    + Not<Output = Self>
{
    const MAX: Self;

    #[inline(always)]
    fn zero() -> Self { Self::default() }

    #[inline(always)]
    fn inf() -> Self { Self::MAX >> 2 }

    #[inline(always)]
    fn from_i32(val: i32) -> Self {
        Self::try_from(val).unwrap_or_else(|_| panic!("Cannot convert {} to {}", val, std::any::type_name::<Self>()))
    }
}

// 任意の整数プリミティブに実装
macro_rules! impl_int {
    ($($t:ty),*) => { $( impl Integer for $t { const MAX: Self = <$t>::MAX; } )* };
}

impl_int!(u8, u16, u32, u64, u128, usize,
          i8, i16, i32, i64, i128, isize);

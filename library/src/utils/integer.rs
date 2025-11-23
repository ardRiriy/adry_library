use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shr, Sub, SubAssign,
    },
};

pub trait Integer:
    Copy
    + Default
    + Debug
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
    + TryFrom<u64>
    + Shr<u32, Output = Self>
    + Not<Output = Self>
{
    const MAX: Self;
    const MIN: Self;

    #[inline(always)]
    fn zero() -> Self {
        Self::default()
    }

    #[inline(always)]
    fn inf() -> Self {
        Self::MAX >> 2
    }

    #[inline(always)]
    fn from_i32(val: i32) -> Self {
        Self::try_from(val).unwrap_or_else(|_| {
            panic!(
                "Cannot convert {} to {}",
                val,
                std::any::type_name::<Self>()
            )
        })
    }

    #[inline(always)]
    fn from_u64(val: u64) -> Self {
        Self::try_from(val).unwrap_or_else(|_| {
            panic!(
                "Cannot convert {} to {}",
                val,
                std::any::type_name::<Self>()
            )
        })
    }

    #[inline(always)]
    fn abs_diff(self, other: Self) -> Self {
        self.max(other) - self.min(other)
    }

    fn to_u8(&self) -> u8;
    fn to_u32(&self) -> u32;
    fn to_u64(&self) -> u64;
    fn to_u128(&self) -> u128;
    fn to_usize(&self) -> usize;
    fn to_i8(&self) -> i8;
    fn to_i32(&self) -> i32;
    fn to_i64(&self) -> i64;
    fn to_i128(&self) -> i128;
    fn to_isize(&self) -> isize;
}

// 任意の整数プリミティブに実装
macro_rules! impl_int {
    ($($t:ty),*) => {
        $( impl Integer for $t {
            const MAX: Self = <$t>::MAX;
            const MIN: Self = <$t>::MIN;

            #[inline]
            fn to_u8(&self) -> u8 { *self as u8 }
            #[inline]
            fn to_u32(&self) -> u32 { *self as u32 }
            #[inline]
            fn to_u64(&self) -> u64 { *self as u64 }
            #[inline]
            fn to_u128(&self) -> u128 { *self as u128 }
            #[inline]
            fn to_usize(&self) -> usize { *self as usize }
            #[inline]
            fn to_i8(&self) -> i8 { *self as i8 }
            #[inline]
            fn to_i32(&self) -> i32 { *self as i32 }
            #[inline]
            fn to_i64(&self) -> i64 { *self as i64 }
            #[inline]
            fn to_i128(&self) -> i128 { *self as i128 }
            #[inline]
            fn to_isize(&self) -> isize { *self as isize }
        } )*
    };
}

impl_int!(u8, u32, u64, u128, usize, i8, i32, i64, i128, isize);

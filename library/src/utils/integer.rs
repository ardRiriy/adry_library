use std::ops::{Add, AddAssign, Shr, Sub};

pub trait Integer:
    Copy
    + Ord
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + Shr<u32, Output = Self>
    + Default
{
    const MAX: Self;

    #[inline(always)]
    fn zero() -> Self { Self::default() }

    #[inline(always)]
    fn inf() -> Self { Self::MAX >> 2 }
}

// 任意の整数プリミティブに実装
macro_rules! impl_int {
    ($($t:ty),*) => { $( impl Integer for $t { const MAX: Self = <$t>::MAX; } )* };
}

impl_int!(u8, u16, u32, u64, u128, usize,
          i8, i16, i32, i64, i128, isize);

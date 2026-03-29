use std::ops::{Bound, RangeBounds};

use crate::utils::integer::Integer;

pub fn unpack_range<R, T>(range: R, lo_unbound: T, up_unbound: T) -> (T, T)
where
    R: RangeBounds<T>,
    T: Integer,
{
    let l = match range.start_bound() {
        Bound::Included(&s) => s,
        Bound::Excluded(&s) => s + T::from_i32(1),
        Bound::Unbounded => lo_unbound,
    };
    let r = match range.end_bound() {
        Bound::Included(&e) => e + T::from_i32(1),
        Bound::Excluded(&e) => e,
        Bound::Unbounded => up_unbound,
    };

    (l, r)
}

/// 整数に対してその桁数を返却する
/// 負数に対する結果は不定
/// # examples
/// ```
/// let x: i32 = 0;
/// assert_eq!(x.digit_count(), 1);
/// let x: i32 = 100;
/// assert_eq!(x.digit_count(), 3);
/// let x: i32 = 12345;
/// assert_eq!(x.digit_count(), 5);
/// let x: i32 = -12345;
/// assert_eq!(x.digit_count(), 5);
/// ```
pub fn digit_count<T>(x: T) -> u32
where
    T: Integer,
{
    if x == T::zero() {
        1
    } else {
        x.ilog10() + 1
    }
}

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

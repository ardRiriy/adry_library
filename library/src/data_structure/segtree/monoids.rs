use std::{fmt::Debug, marker::PhantomData};

use crate::utils::integer::Integer;


pub trait Monoid {
    type S: Copy + Debug;
    fn op(a: Self::S, b: Self::S) -> Self::S;
    fn id() -> Self::S;
}

pub trait MapMonoid {
    type M: Monoid;
    type F: Clone + PartialEq + Debug;
    fn op(a: &<Self::M as Monoid>::S, b: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        Self::M::op(*a, *b)
    }
    fn id() -> <Self::M as Monoid>::S {
        Self::M::id()
    }
    fn identity_map() -> Self::F;
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S;
    fn composition(f: &Self::F, g: &Self::F) -> Self::F;
}

pub struct RangeMinMonoid<T> (PhantomData<T>);
impl<T: Integer> Monoid for RangeMinMonoid<T> {
    type S = T;
    fn op(a: Self::S, b: Self::S) -> Self::S {
        a.min(b)
    }
    fn id() -> Self::S {
        T::MAX
    }
}

pub struct RangeSumMonoid<T> (PhantomData<T>);
impl<T: Integer> Monoid for RangeSumMonoid<T> {
    type S = T;
    fn op(a: Self::S, b: Self::S) -> Self::S {
        a+b
    }
    fn id() -> Self::S {
        T::from_i32(0)
    }
}
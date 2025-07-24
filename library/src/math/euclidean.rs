// ユ－クリッドの互除法

use std::ops::Neg;

use crate::utils::integer::Integer;

pub fn gcd<T: Integer>(a: T, b: T) -> T {
    if a == T::zero() { return b; }
    if b == T::zero() { return a; }
    return gcd(b, a % b);
}

pub fn lcm<T: Integer>(a: T, b: T) -> T {
    if a == T::zero() || b == T::zero() { return T::zero(); }
    return a / gcd(a, b) * b;
}

// 拡張ユークリッドの互除法
// ax + by = gcd(a, b) を満たす整数 x, y を求める
// 整数解が存在しない場合は None を返す
pub fn extended_gcd<T: Integer + Neg>(a: T, b: T) -> Option<(T, T, T)> {
    if a == T::zero() {
        if b == T::zero() { return None; }
        return Some((T::zero(), T::from_i32(1), b));
    }
    if b == T::zero() {
        return Some((T::from_i32(1), T::zero(), a));
    }

    if let Some((x,y,d)) = extended_gcd(b,a%b) {
        return Some((y, x - (a / b) * y, d));
    } else {
        return None;
    }
}

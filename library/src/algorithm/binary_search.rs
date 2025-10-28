use crate::utils::integer::Integer;

pub fn binary_search<T: Integer, F: Fn(T) -> bool>(ok: T, ng: T, f: F) -> T {
    let mut ok = ok;
    let mut ng = ng;

    while ok.abs_diff(ng) > T::from_i32(1) {
        let mid = (ok + ng) >> 1;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    ok
}

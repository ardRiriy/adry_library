pub fn zeta_transform<T: Copy + std::ops::Add<Output = T>>(f: &[T], n: usize) -> Vec<T> {
    let mut g = f.to_vec();
    for i in 0..n {
        for s in 0..1 << n {
            if s & (1 << i) != 0 {
                g[s] = g[s] + g[s ^ (1 << i)];
            }
        }
    }
    g
}

pub fn mobius_transform<T: Copy + std::ops::Sub<Output = T>>(g: &[T], n: usize) -> Vec<T> {
    let mut f = g.to_vec();
    for i in 0..n {
        for s in 0..1 << n {
            if s & (1 << i) != 0 {
                f[s] = f[s] - f[s ^ (1 << i)];
            }
        }
    }
    f
}

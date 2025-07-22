pub fn modpow<T>(a: T, n: T, p: T) -> T 
where T: num::PrimInt + Copy
{
    let mut x:T = a;
    let one = match T::from(1) {
        Some(x) => x,
        None => panic!()
    };
    let mut res= one;
    for i in 0..64 {
        if (n >> i) & one == one {
            res = (res * x) % p;
        }
        x = (x * x) % p;
    }
    res % p
}

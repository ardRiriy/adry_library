// verification-helper: PROBLEM https://judge.yosupo.jp/problem/aplusb

use library::utils::input::Input;

fn main() {
    let mut input = Input::new();
    let (a,b) = input.pair::<i64,i64>();
    println!("{}", a + b);
}

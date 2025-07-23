// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb_128bit

use library::utils::input::Input;

fn main() {
    let mut input = Input::new();
    let t = input.next::<usize>();
    let res: Vec<i128> = (0..t)
        .map(|_| {
            let (a, b): (i128, i128) = input.pair();
            a + b
        })
        .collect();

    println!("{}", res.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"));
}

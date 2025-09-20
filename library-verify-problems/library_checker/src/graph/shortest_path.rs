// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path

use library::{graph::dijkstra::Dijkstra, utils::input::Input};

fn main() {
    let mut input = Input::new();
    let (n, m) = input.pair::<usize>();
    let (s, t) = input.pair::<usize>();
    let graph = input.weighted_graph::<i64>(n, m, true, false);

    let djk = Dijkstra::new(s, &graph);
    if let Some(v) = djk.path(t) {
        println!("{} {}", djk.get(t), v.len() - 1);
        println!(
            "{}",
            v.windows(2)
                .map(|w| format!("{} {}", w[0], w[1]))
                .collect::<Vec<String>>()
                .join("\n")
        );
    } else {
        println!("-1");
    }
}

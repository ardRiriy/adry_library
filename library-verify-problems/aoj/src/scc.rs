// verification-helper: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_3_C&lang=ja
use library::{graph::scc::Scc, utils::input::Input};

fn solve(ip: &mut Input) {
    let (v, e) = ip.pair::<usize>();
    let mut graph = vec![Vec::new(); v];
    for _ in 0..e {
        let (s, t) = ip.pair::<usize>();
        graph[s].push(t);
    }

    let groups = Scc::new(graph).execute();
    let mut scc_id = vec![0usize; v];
    for (id, group) in groups.iter().enumerate() {
        for &node in group {
            scc_id[node] = id;
        }
    }

    let q = ip.next::<usize>();
    for _ in 0..q {
        let (u, v) = ip.pair::<usize>();
        println!("{}", if scc_id[u] == scc_id[v] { 1 } else { 0 });
    }
}

fn main() {
    let mut ip = Input::new();
    solve(&mut ip);
}
